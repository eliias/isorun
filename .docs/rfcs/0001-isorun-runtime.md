# RFC 0001: isorun runtime

A custom JavaScript runtime for isorun, built on `rusty_v8`.

- Status: draft
- Author: Hannes Moser
- Created: 2026-10-20

## tl;dr

Replace `deno_core` and `deno_runtime` with a small runtime that we own, built
directly on the `v8` crate (rusty_v8). Run V8 on dedicated threads (instead of
on Ruby threads). A Ruby thread waits for JS with the GVL released, so async JS
work no longer blocks other Ruby threads (today it does). Drop the magnus fork,
tokio, protobuf and libffi from the build. Do not keep compatibility with the
Deno runtime.

## Background

Cloudflare acquired Deno on 2026-10-09. The Deno runtime gets bug-fix and
security releases for one year, then its development ends. Cloudflare keeps
rusty_v8 and plans to integrate it into workerd (see References).

isorun uses `deno_runtime` for five things:

1. The ES module load.
2. The event loop (promises and timers), driven by tokio.
3. Ops, the calls from JS to Rust. isorun has one op, `op_send_to_ruby`.
4. Web APIs (fetch, URL, TextEncoder, streams, console).
5. The worker setup (isolate, permissions, bootstrap).

isorun uses a small part of each, but the build pays for all of `deno_runtime`:
FFI, file system, network and more. The native gems take about 40 min to build
(#33) and are big (about 67 MB per Ruby version, #72). Each deno upgrade broke
many APIs (#114, #172).

The current design also has three structural problems:

- **V8 runs on Ruby threads.** Each Ruby thread gets its own isolate in a
  `thread_local`. V8 and Ruby share the thread stack, signals and the GVL.
- **The event loop holds the GVL.** isorun releases the GVL only for the sync
  JS call. When JS awaits (fetch, timers), the loop runs with the GVL held, and
  other Ruby threads wait.
- **The GVL release needs a magnus fork.** Upstream magnus has no API for it
  (matsadler/magnus#129). We maintain the fork only for this one feature.

## Goals

- One runtime layer that we own, with rusty_v8 as the only engine dependency.
- No V8 code on Ruby threads, and no Ruby code on V8 threads.
- Ruby threads wait for JS with the GVL released, for the full call.
- A clean and small set of Web APIs, sufficient for the SSR bundles that we
  test (React, Vue, Vite, D3).
- Clear errors: a JS exception or a bad value gives a Ruby exception, never a
  Rust panic.
- A runtime core that we can test with `cargo test`, without Ruby.
- A smaller and faster build.

## Non-goals

- Compatibility with the Deno runtime, its globals (`Deno.*`) or its ops.
- A general-purpose JS runtime (no file system, no network, no npm resolver,
  no Node.js compatibility layer).
- A second engine now. We keep all V8 code in one crate, so a later switch
  (for example to QuickJS) replaces one crate. We do not add an engine trait
  with one implementation.
- Compatibility of the Ruby API. We keep the parts that work well (the import
  DSL), and we change the rest where it makes the design simpler.

## Proposed solution

### Crates

We split the Rust code into two crates in one workspace:

- `crates/isorun-runtime`: the runtime. It depends on `v8`, and it has no Ruby
  code. It contains the worker threads, the module load, the event loop, the
  Web APIs and the value type.
- `ext/isorun`: the Ruby extension. It depends on `isorun-runtime`, upstream
  `magnus` and `rb-sys`. It contains no V8 code.

The split makes the rule from the Goals a compile-time fact: the Ruby crate
cannot touch V8, and the runtime crate cannot touch Ruby. It also lets us test
the runtime with `cargo test` (today, the extension does not link outside of
`rake compile`). A later Python binding (#225) can use the same runtime crate.

### Threads

```
Ruby thread (Puma)                      JS worker thread (one isolate)
------------------                      ------------------------------
convert args to Value
send Call ----------------------------> run the JS function
release the GVL, wait                   run the event loop
                    <------------------ HostCall(message)  (JS awaits)
take the GVL, call the receiver Proc
send HostReply -----------------------> resolve the JS promise
release the GVL, wait                   run the event loop
                    <------------------ Result(Value) or Error
take the GVL, convert Value to Ruby
```

The runtime starts a pool of worker threads. Each worker thread owns one
isolate for its full life. The isolate never moves to a different thread, so we
do not need `v8::Locker`.

A Ruby thread and a worker thread share no V8 objects and no Ruby objects. They
send messages through channels:

- `Call`, `HostReply` and `Cancel` go from Ruby to the worker.
- `HostCall`, `Result` and `Error` go from the worker to Ruby.

All message data uses the `Value` type (see "Values").

The Ruby thread waits on the channel inside `rb_thread_call_without_gvl`. When
a message arrives, the wait returns, and the Ruby thread has the GVL again. It
handles the message (for example, it calls the receiver), and then it waits
again. We call `rb_thread_call_without_gvl` from `rb-sys` directly. We do not
need `rb_thread_call_with_gvl`, because no code takes the GVL back from inside
the wait. This removes the reason for the magnus fork.

The unblock function of `rb_thread_call_without_gvl` sends `Cancel` and calls
`terminate_execution` on the isolate handle. This makes `Thread#raise`,
`Thread#kill` and `Timeout.timeout` work. The handle is thread-safe in rusty_v8.

### Contexts

A `Context` is one V8 context on one worker thread, with its own module map
and its own globals. A context stays on its worker thread for its full life.
Two contexts do not share module state (today they can).

The Ruby side decides the life of a context:

- A long-lived context per bundle gives the fastest calls. State stays from one
  call to the next.
- A new context per request gives full isolation. Each request evaluates the
  bundle again.

The Rails helpers use one context per bundle and worker thread, created on the
first use. In development, the helpers make a new context when the bundle file
changes. This replaces the `?t=<mtime>` URL trick, and the old modules go away
with the old context (today they stay in memory).

### Modules

The runtime loads ES modules from files only. The resolve callback accepts
relative and absolute paths and `file://` URLs, and it caches modules per
context by canonical path. It does not resolve bare specifiers (`"react"`):
the bundler resolves those at build time. Dynamic `import()` uses the same
resolver.

### Event loop

A call runs these steps on the worker thread:

1. Call the exported JS function.
2. If the result is a promise, run the loop until the promise settles:
   1. Run the microtask checkpoint.
   2. Run the timers that are due.
   3. Resolve the promises of host calls that got a `HostReply`.
   4. If no work is left and the promise is still pending, fail the call with a
      "promise never settled" error.
3. Send `Result` or `Error` to the Ruby thread.

A call has a timeout (configurable). After the timeout, the worker calls
`terminate_execution` and sends `Error`.

### Host calls

JS calls Ruby through one global function:

```js
const reply = await isorun.send({ action: "fetch", args: { url } });
```

`isorun.send` returns a promise. The worker sends `HostCall` to the waiting Ruby
thread, and it resolves the promise when the `HostReply` arrives. If the Ruby
receiver raises, the promise rejects with an `Error` that has the Ruby message.

`isorun.send` is the only way from JS to Ruby. The `fetch` global uses it too:
it sends a request to Ruby, and the Ruby receiver decides what to do with it
(for example, it executes a GraphQL query in process). There is no network
access from JS.

### Web APIs

We give the smallest set of Web APIs that the test bundles need. We add an API
only when a real bundle needs it. The first set comes from the spec bundles
(`render.js` and `vite.js`) and the examples:

| API | Implementation |
|---|---|
| `console` | Rust. Writes to the Ruby logger through a host call. |
| `setTimeout`, `clearTimeout`, `setInterval`, `clearInterval`, `setImmediate` | Rust, in the event loop. |
| `queueMicrotask` | V8. |
| `TextEncoder`, `TextDecoder` | Rust (UTF-8 only). |
| `URL`, `URLSearchParams` | JS polyfill. |
| `ReadableStream` and related stream classes | JS polyfill. |
| `AbortController`, `AbortSignal`, `EventTarget` | JS polyfill. |
| `MessageChannel` | JS, on top of the timer queue. |
| `fetch`, `Request`, `Response`, `Headers` | JS, on top of `isorun.send`. |
| `performance.now` | Rust. |
| `crypto.getRandomValues`, `crypto.randomUUID` | Rust. |
| `structuredClone` | V8 serializer. |
| `process.env.NODE_ENV` | JS. A minimal `process` object, nothing more. |

The JS parts live in one `runtime.js` file in the runtime crate. A build step
bundles the polyfills from npm into this file, and we commit the result. The
worker evaluates `runtime.js` in each new context.

`Buffer` appears in `vite.js`. We do not add it in the first version. We check
if the bundler can remove it (it is a Node.js API) before we decide.

### Values

The `Value` type is the only data that crosses between Ruby and V8:

| Ruby | `Value` | JS |
|---|---|---|
| `nil` | `Null` | `null`, `undefined` |
| `true`, `false` | `Bool` | `boolean` |
| `Integer` (safe range) | `Int(i64)` | `number` (integral, safe range) |
| `Integer` (other) | `BigInt` | `bigint` |
| `Float` | `Float(f64)` | `number` |
| `String` (UTF-8) | `String` | `string` |
| `String` (binary) | `Bytes` | `Uint8Array` |
| `Symbol` | `String` | `string` |
| `Array` | `Array` | `Array` |
| `Hash` (String or Symbol keys) | `Object` (ordered) | plain object |
| `Time` | `Date` | `Date` |

Any other type is an error, not `nil` (today, an unknown type silently becomes
`null` or `nil`). A JS function is not a `Value`. An import of a function export
gives an `Isorun::Function` handle that points to the function in its context.

### Errors

- A JS exception gives `Isorun::JavaScriptError`, with the JS message and stack.
- A timeout gives `Isorun::TimeoutError`.
- An unsupported value gives `Isorun::ValueError`.
- A Ruby exception in the receiver rejects the promise in JS.
- The Ruby crate catches Rust panics at the boundary and raises `Isorun::Error`.
  The runtime crate does not use `unwrap` on values that come from JS or Ruby.

### Ruby API

```ruby
Isorun.configure do
  workers 4          # number of JS worker threads
  timeout 5          # seconds per call
  receiver { |message| ... }
end

context = Isorun::Context.new(receiver: ->(message) { ... })
render = context.import(:render).from("app/assets/builds/app-server.js")
html = render.call({ path: "/" })
```

The import DSL stays. `Isorun::Context.create` with a block and the thread-wide
receiver go away: the receiver belongs to the context. `Isorun.stats` gives the
heap statistics of each worker.

### Process fork

V8 starts background threads when it starts its platform. A fork copies only
the calling thread, so a V8 platform from before the fork does not work in the
child. Puma in cluster mode with `preload_app!` boots the app first and forks
the workers after that.

The runtime starts the V8 platform and the worker threads on the first call,
not on `require`. It stores the process ID at start. If a call comes from a
different process ID, isorun raises an error that tells the user to move the
first call to after the fork. A V8 platform cannot start a second time in one
process, so a restart in the child is not an option.

### Build and packaging

The new build needs Rust and the prebuilt V8 static library, which the `v8`
crate downloads at build time. It does not need `protoc`, `cmake`, `libffi` or
tokio. rusty_v8 publishes prebuilt libraries for Linux (x86_64 and aarch64, glibc
and musl), macOS (x86_64 and aarch64) and Windows. This adds Linux ARM64 (#242).

We pin an exact `v8` version and upgrade it on purpose, because each V8 release
is a major version of the crate.

## Alternatives

- **Keep `deno_core` only.** It removes most of `deno_runtime`, but it keeps a
  dependency on code whose owner stops its development.
- **Write the runtime in C++ on libv8-node (as mini_racer does).** It gives the
  same V8, but we lose Rust and the prebuilt libraries.
- **Run workerd or Node.js as a separate process.** It removes V8 from the Ruby
  process, but every host call becomes inter-process communication. In-process
  host calls are the main reason isorun exists.
- **QuickJS (rquickjs).** It is small and builds fast, but it has no JIT. We
  expect React SSR to be several times slower (not measured). It is the backup
  if rusty_v8 stops its releases.
- **Boa, SpiderMonkey (mozjs).** Boa is too slow for SSR. mozjs has a heavy
  build, and Servo is its main user.

## Risks

- **rusty_v8 releases stop.** The announcement promises support, but it does
  not promise crates.io releases or prebuilt libraries after the workerd
  integration. Mitigation: `RUSTY_V8_MIRROR` lets us host the libraries, and
  `V8_FROM_SOURCE` builds V8 locally. We watch the release cadence.
- **Web API gaps.** A bundle can use an API that we do not have. Mitigation:
  the bundles in `spec/` and `examples/` are the test set, and a missing API
  gives a clear `ReferenceError`.
- **Copy cost.** Each call copies its arguments and its result through `Value`.
  For SSR, the result is one HTML string, so we expect a small cost. We measure
  it with `benchmarks/`.
- **Thread count.** Each worker thread has one isolate (tens of MB of heap). The
  pool size limits the memory, but also the JS concurrency.

## Open items

1. The default pool size: the number of CPU cores, or the number of Puma
   threads?
2. The Rails helpers: one context per bundle and worker thread, or a pool of
   contexts per bundle?
3. A startup snapshot with `runtime.js` (and maybe the bundle) for faster
   context creation (#27). We measure the context creation time first.
4. A heap limit per isolate with `add_near_heap_limit_callback` (#26).
5. The next version of `@isorun/rails`: `isorun.send` instead of `Deno` ops.

## Roadmap

Each step is one PR.

1. `crates/isorun-runtime`: worker thread, module load, call, `Value`, event
   loop. `cargo test` loads `say.js` and `values.js`.
2. `ext/isorun`: Ruby bridge on the new runtime, GVL release, receiver, errors.
   The import specs pass. Remove the deno crates and the magnus fork.
3. Web APIs and `runtime.js`. The `render.js` and `vite.js` specs pass.
4. Rails helpers, `@isorun/rails` and the examples on the new API.
5. Timeout, process fork check, `Isorun.stats`, benchmarks against the Deno
   version.

## References

- Deno is joining Cloudflare: https://deno.com/blog/cloudflare
- rusty_v8: https://github.com/denoland/rusty_v8
- magnus GVL issue: https://github.com/matsadler/magnus/issues/129
- mini_racer (V8 in Ruby, C++): https://github.com/rubyjs/mini_racer
- isorun issues: #18 (lighter worker), #24 (timeout), #25 (worker pool),
  #26 (memory limit), #27 (snapshots), #33 (build time), #34 (GVL segfault),
  #72 (gem size), #225 (runtime core), #242 (deno 2.x)
