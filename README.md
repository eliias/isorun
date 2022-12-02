<p align="center">
  <img alt="isorun" src="./docs/assets/logo.png" width="200" />
</p>

---

> A JavaScript render target for Ruby (on Rails)

**⚠ ️Attention:** Don't use this for anything but experiments. There are better ways to embed V8
in your application. Check out [miniracer](https://github.com/rubyjs/mini_racer).

## Usage

```bash
rails new myproject --javascript esbuild
```

```ruby
# config/initializers/isorun.rb
Isorun.configure do
end
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

## Server-side rendering

The slowest way to deliver a website, is delivering a JavaScript file
to the client, and then execute it to build up a DOM tree.

The fastest way is serving an HTML page with embedded styles, and doing so as a
stream of contents to the client. Eventually the application will (re-)hydrate
the already rendered user interface as soon as the downloaded JavaScript code
is executed.

You can take this concept even further and make your application work without
JavaScript at all, but still use React or Vue (or any other view-controller
library) to define your user interface.

Read
more: [Netflix functions without client-side React, and it's a good thing](https://jakearchibald.com/2017/netflix-and-react/).

Server-side rendering has a few challenges:

1. You need something that can compile and run JavaScript
1. You need to be able to integrate the app with your preferred framework
1. You need to deal with the reality of frontend clients making network requests and managing state

**isorun** aims to make it as simple as possible to integrate a
JavaScript application into your server-side development and deployment
workflow.

This gem provides a helper that can render a JavaScript application directly in
your Ruby process, embedding Google's v8 library.
You can think of it as running a headless JavaScript VM in your Ruby process.
**isorun** utilizes V8 Isolates via the Rust crates: v8 and deno_core. This
allows us to completely separate applications from each other and to prevent
any [Cross-Request State Pollution](https://vuejs.org/guide/scaling-up/ssr.html#cross-request-state-pollution).
This is like having multiple tabs open in your browser.

## Why SSR for Ruby (on Rails)?

I use *Ruby on Rails* a lot for projects, and I also use both, Vue and React.
One of my goals for **isorun**, is that server-side rendering should feel
naturally in Rails. A simple tag helper should be enough to render, deliver,
and hydrate your complex JavaScript application.

### Alternative

I also recommend taking a look at [HTML over the Wire](https://hotwired.dev/),
and [StimulusReflex](https://docs.stimulusreflex.com/), but this is not for
everyone.

## Why not just spinning up a Node.js/deno/bun service?

**isorun** does SSR a bit different from how you would do it in a regular
Node.js service. In addition to render an application and extracting state and
styles, it also allows you to talk directly with your Rails application. 
This enables you to take shortcuts for certain scenarios, for example rendering
the result of network calls (Apollo), without hitting the network once. Instead
of fetching data from the server via HTTP, you can just call a Ruby function
from you JavaScript application and request the data you need to render the
view.

I have built and operated many Node.js SSR services, but it has always been
super tedious to set up such a dedicated service.
This is especially true when your backend isn't written in JavaScript. In
reality, a SSR service is just another single point of failure, and you need
to proxy every single request through it. This can significantly add to your
infrastructure cost, migrating from an existing app becomes harder than it
should be, and there is operational complexity and overhead.

## Installation

Install the gem and add to the application's Gemfile by executing:

    $ bundle add isorun

If bundler is not being used to manage dependencies, install the gem by
executing:

    $ gem install isorun

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
