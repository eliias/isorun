# isorun

isorun runs JavaScript inside a Ruby process. A Rust native extension embeds V8
through `deno_runtime`, and Ruby imports ES module exports as values or callable
functions. The main use case is server-side rendering (SSR) of React or Vue apps
in Rails, with JS fetch calls sent to Ruby instead of the network.

The project was archived from 2024 to 2026. The last real code change before the
archive was 8870aa5 (2024-01-23). All later commits up to 2025-02 came from
dependabot.

Design proposals live in `.docs/rfcs/`. RFC 0001 replaces the deno runtime with
a custom runtime on rusty_v8.

## Layout

- `lib/`: the Ruby API. `Isorun::Context` (import DSL, module reload in
  development), config, Rails engine. `lib/isorun/module.rb` and `function.rb` are
  empty, the extension defines the methods.
- `app/helpers/isorun/`: the Rails view helpers `isorun_app` (esbuild/sprockets
  bundles) and `isorun_vite_app` (vite-ruby).
- `ext/isorun/src/`: the Rust extension.
  - `lib.rs`: the `#[magnus::init]` entry, it defines the Ruby classes.
  - `isorun/`: the Ruby-facing wrappers and the value conversion (`utils.rs`).
  - `js/worker.rs`: the V8/deno worker, the event loop, and the GVL handling.
  - `ext/isorun.rs`: the deno op `op_send_to_ruby`.
  - `call.js`: the main module that the worker loads at boot.
- `packages/isorun-rails`: the npm package `@isorun/rails`. It gives `send` (JS to
  Ruby messages) and an Apollo `fetch` that sends GraphQL requests to Ruby.
- `spec/`: RSpec with a Rails dummy app in `spec/dummy`. `render.js` and `vite.js`
  in `spec/dummy/app/javascript` are prebuilt bundles (about 1 MB each). Do not
  edit them by hand.
- `examples/`: Rails apps for React, React with Vite, Vue, D3 and a multi-app page.
  Each one uses the gem from `../..`.
- `docs/`: user docs, published as YARD to gh-pages.

## Build and test

You need Rust stable, `cmake` and `protobuf` (`brew install cmake protobuf`).

```sh
bundle install
bundle exec rake compile   # builds lib/isorun/isorun.bundle
bundle exec rspec          # or: bundle exec rake (compile + spec)
cargo fmt --check
cargo clippy
```

- A plain `cargo build` fails at link time, because only `rake compile` sets the
  Ruby link flags. Use `cargo check` or `cargo clippy` for fast feedback.
- The built extension is specific to one Ruby ABI. After you change the Ruby
  version, run `rake compile` again. A stale build gives errors like "no implicit
  conversion of Isorun::Context into Isorun::Context", or a segfault.
- Supported Ruby: 3.2, 3.3, 3.4 and 4.0.
- JS: `yarn install` at the root (yarn 1, workspaces). Use
  `npx turbo run build lint --filter=@isorun/rails` for the companion package.
- CI (`.github/workflows/tests.yml`) runs the specs, clippy and rustfmt. It does
  not run RuboCop. RuboCop reports about 47 old offenses.

## Runtime model

Each Ruby thread gets its own worker: one `MainWorker` with one V8 isolate and one
tokio `current_thread` runtime, in a `thread_local!` in `js/worker.rs`. The worker
starts on the first call and lives until the thread exits. Puma with 5 threads
means 5 isolates.

One `Function#call` does these steps:

1. It converts the Ruby arguments to V8 values. The GVL is held.
2. It releases the GVL (`magnus::gvl::without_gvl`) and calls the JS function.
3. It takes the GVL back and drives the event loop until the promise settles.
   This step holds the GVL, so async JS work blocks other Ruby threads.
4. It converts the result to Ruby.

JS sends a message to Ruby with `op_send_to_ruby`. If the GVL is released at that
time, the op takes it back with `GVLContext::with_gvl` and calls the receiver
`Proc`. The receiver belongs to the thread, not to the context.

Rule: touch Ruby objects only while the GVL is held. A break of this rule caused
the segfaults in #34 ("object allocation during garbage collection phase"). PR #43
fixed them: Ruby values are converted before the GVL release, and only the V8 call
runs without it.

`Isorun::Context.new` creates a `JsRealm`. Modules load on the main runtime and
are cached per path for the life of the thread, so module state can leak across
contexts (from a code read, not tested). In development, the import DSL adds
`?t=<mtime>` to the module URL to reload a changed bundle. Old modules stay in
memory.

## The magnus fork

Upstream magnus has no API to release the GVL (matsadler/magnus#129 is still
open). The fork at https://github.com/eliias/magnus adds `src/gvl.rs`:
`without_gvl` wraps `rb_thread_call_without_gvl`, and `GVLContext::with_gvl` wraps
`rb_thread_call_with_gvl`.

- Branch `gvl-api-0.9.3`: upstream 0.9.3 plus the GVL commit. isorun uses this
  branch.
- Branch `add-gvl-api`: the old branch (magnus 0.6 era). It does not build on
  Ruby 4.0.
- To move to a new magnus release, cherry-pick the GVL commit onto the release
  tag and push it as a new branch. Do not rebase the old branches, because old
  isorun commits pin their revisions.

isorun still uses the deprecated magnus free functions (`qnil()`,
`RString::new`, and more). They give about 23 deprecation warnings. The fix is to
pass a `&Ruby` handle through the code.

## Dependency state

- deno is pinned old: `deno_core` 0.204, `deno_runtime` 0.126, `v8` 0.74. Each
  deno upgrade changed many APIs (worker options, op macros, realms), see #114
  and #172. A `cargo update` of everything fails, because `deno_ops` 0.82 needs
  a yanked crate. Update single crates with `cargo update -p <crate>`.
- `deno_ffi` pins `libffi-sys` 2.3.0, and its bundled libffi does not build with
  the current Apple clang. On macOS, `ext/isorun/Cargo.toml` links the libffi of
  the SDK instead (the `system` feature).
- Dependabot opens PRs for single deno crates. Do not merge them alone: the deno
  crates must move together, or the lockfile gets several copies of v8.

## Known problems and open ideas

- GVL during the event loop: async JS holds the GVL (see "Runtime model").
- Memory: one isolate per Ruby thread, never shut down. Open ideas: a worker pool
  (#25), soft timeout (#24) and memory limit (#26) like mini_racer, V8 snapshots
  (#27), a lighter worker without the full deno extensions (#18).
- Panics: many `.unwrap()` calls in the conversion and call paths. A JS exception
  or an Integer outside the i32 range becomes a Rust panic.
- Conversion gaps: a Ruby Symbol becomes a JS string (`docs/types.md` says
  Symbol). JS functions and Dates become hashes.
- Web workers: `worker.rs` has a `todo!()`, a JS `new Worker()` panics.
- Native gems: the last native gems on RubyGems are 0.1.11. 0.1.12 is
  source-only, so users compile Rust locally. Builds are slow (about 40 min, #33)
  and big (about 67 MB per Ruby, #72).
- `benchmarks/app_bench.rb` uses `Isorun::VM`, which does not exist anymore.

Roadmap issues with no code yet: #122 (1.0, React focus), #225 (a JS runtime core
with v8 or JavaScriptCore and bindings for more languages), #242 (deno 2.x, smaller
binaries, Linux ARM64), #299 (magnus 0.7+, a pre-warmed V8 thread pool), #210
(React server components), #28 (no Rails dependency).
