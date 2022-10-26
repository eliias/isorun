# Isorun

> A JavaScript render target for Ruby (on Rails)

## Server-side rendering

The slowest way to deliver a user interface, is delivering a JavaScript file
to the client, and then execute it to build up a DOM tree.

The fastest way is serving an HTML page with embedded styles, and doing so as a
stream of contents to the client. Eventually the application will (re-)hydrate
the already rendered user interface as soon as the downloaded JavaScript code
is executed.

You can take this concept even further and make your application work without
JavaScript at all, but still use React or Vue (or any other view-controller
library) to define your user interface.

Read
more: [Netflix functions without client-side React, and it's a good thing](https://jakearchibald.com/2017/netflix-and-react/))
.

Server-side rendering has a few challenges:

1. You need something that can compile and run JavaScript
2. You need to be able to integrate it into your language and framework
3. You need to deal with the reality of frontend clients making network requests

**isorun** aims to make it as simple as possible to integrate any existing
JavaScript application into your server-side development and deployment
workflow.

This gem provides a helper that can render a JavaScript application on the
server by embedding Google's v8 library directly in your server process(es).
You can think of it as running a headless browser in your Ruby process.
**isorun** utilizes V8 Isolates via the Rust crates: v8 and deno_core. This
allows us to completely separate applications from each other and to prevent
any [Cross-Request State Pollution](https://vuejs.org/guide/scaling-up/ssr.html#cross-request-state-pollution).
It is like having multiple tabs open in your browser.

## Why SSR for Ruby (on Rails)?

I use *Ruby on Rails* a lot for my own projects, and I also use Vue and React.
One of my goals for **isorun**, is that server-side rendering should feel
naturally in Rails. A simple tag helper should be enough to render, deliver,
and hydrate your complex JavaScript application.

I also like the relatively new concept of
[HTML over the Wire](https://hotwired.dev/), but I can't and don't want to use
it for everything.

## Why not just spinning up a Node.js/deno/bun service?

**isorun** does SSR a bit different than you would do it in a regular Node.js
service. It allows you to intercept network calls and can provide *state* to
the render context. This allows you to take a shortcut when rendering on the
server. Data is no longer fetched over the network, but provided almost directly
to the application.

I have built and operated Node.js SSR services, but it has always been super
tedious to set up a dedicated service, just for server side rendering.
Especially when your backend isn't written in JavaScript. A SSR service is just
another single point of failure, and it literally has to process every request.
It adds significantly to your infrastructure cost, migrating from an existing
app is hard, and there is also operational overhead.

Most importantly, it adds complexity, and that is something I try to avoid in
projects I am working on.

## Installation

Install the gem and add to the application's Gemfile by executing:

    $ bundle add isorun

If bundler is not being used to manage dependencies, install the gem by
executing:

    $ gem install isorun

## Usage

```bash
rails new myproject --javascript esbuild
```

```jsx
// app/javascript/my_app.jsx
import * as React from "react";
import {hydrateRoot} from "react-dom/client";

import {App} from "./my_app/App.jsx";

const container = document.querySelector('#my_app');
hydrateRoot(container, <App/>);

```

```jsx
// app/javascript/my_app-server.jsx
import * as React from "react";
import * as Server from "react-dom/server";

import {App} from "./my_app/App.jsx";

export function render() {
  return Promise.resolve(Server.renderToString(<App/>));
}
```

```erb
<!--my_view.html.erb-->
<%= isorun_app_tag("my_app") %>
```

```json
{
  "scripts": {
    "build": "esbuild app/javascript/*.* --bundle --sourcemap --outdir=app/assets/builds --public-path=assets --format=esm"
  }
}
```

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run
`rake spec` to run the tests. You can also run `bin/console` for an interactive
prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To
release a new version, update the version number in `version.rb`, and then run
`bundle exec rake release`, which will create a git tag for the version, push
git commits and the created tag, and push the `.gem` file to
[rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at
https://github.com/eliias/isorun.

## License

The gem is available as open source under the terms of the
[MIT License](https://opensource.org/licenses/MIT).
