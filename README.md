<p align="center">
  <img alt="isorun" src="./docs/assets/logo.png" width="200" />
</p>

---

> Run JavaScript applications in your Rails application.

## Features

* Import JavaScript functions, objects, or just values and use them in Ruby
* An EMCAScript like Ruby DSL to load modules and import items
* Automatically converts arguments and return values
* Send messages between *JavaScript*<->*Ruby* (allows to intercept network requests and avoid network round-trips for e.g. API calls)
* Automatically reload modules when updated in development
* Automatically extracts state (Apollo) and hydrates client-side 
* Supports server-side rendering of multiple apps on a single page
* Examples for [React](./examples/rails-react-app), [Vue](./examples/rails-vue-app), [D3](./examples/rails-service-app) and a [multi-app](./examples/rails-multi-app) setup

## How to

```bash
rails new myproject --javascript esbuild
```

```json
{
  "scripts": {
    "build": "esbuild app/javascript/app.jsx --bundle --sourcemap --outdir=app/assets/builds --public-path=assets",
    "build-server": "esbuild app/javascript/app-server.jsx --bundle --sourcemap --outdir=app/assets/builds --public-path=assets --format=esm"
  }
}
```

```ruby
# config/initializers/isorun.rb
Isorun.configure do
  # …configure isorun
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

export default async function() {
  return Server.renderToString(<App/>);
}
```

```erb
<!--my_view.html.erb-->
<%= isorun_app("my_app") %>
```

## Demo

You can also check out this demo video on YouTube. It shows how you can utilize
*isorun* to render SVGs with Ruby on the server, utilizing JavaScript and the
D3 library.

[![How to use d3 in Ruby](./docs/assets/how-to-use-d3-in-ruby.png)](https://www.youtube.com/watch?v=EPHX4po4X4g)

## Why server-side rendering (SSR)?

The fastest way to deliver an application to the user is streaming HTML directly
to the browser. The slowest way to deliver an application, is downloading a
JavaScript file first, parse and execute it on the client side.

Server-side rendering is taking advantage of the fact that we can render a
JavaScript application directly on the server, and stream the resulting HTML
directly to the browser.
Then we fetch the JavaScript file and eventually the application will
(re-)hydrate the already rendered user interface.

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
workflow, without changing the development workflow for frontend engineers.

This gem provides a helper that can render a JavaScript application directly in
your Ruby process, embedding Google's *v8* library via [*deno_core*](https://crates.io/crates/deno_core).
You can think of it as running a headless JavaScript browser directly in your
Ruby process (threads). Using *v8* allows us to completely separate the
execution environments between individual renders and therefore prevent any
potential [Cross-Request State Pollution](https://vuejs.org/guide/scaling-up/ssr.html#cross-request-state-pollution).
It is essentiallly the same as opening many tabs in one browser.

## Why SSR for Ruby (on Rails)?

I personally enjoy and use *Ruby on Rails* a lot, but I like to use some
Vue and React for frontend work. The integration of frontend and backend always
felt a bit off, and I wanted something that "just works" for most of my use
cases.

One goal of **isorun** is that server-side rendering should feel naturally in
Ruby and Rails. A simple tag helper should be enough to render, deliver, and
hydrate your complex JavaScript application. And if we want to do something
nice with visualization libraries, it should be possible to run any JavaScript
program and return the result to the user without spinning up a separate
service.

### Alternative

If you want to go all-in on the server side, I highly recommend taking a look at
[HTML over the Wire](https://hotwired.dev/), and [StimulusReflex](https://docs.stimulusreflex.com/).

## Why not just spinning up a Node.js/deno/bun service?

**isorun** does SSR a bit different from how you would do it in a regular
Node.js service. In addition to being able to render the application, it also
supports more powerful features like network intercepts. This means, that you
can directly call into the Ruby process from the JavaScript application and
e.g. fetch data from the database. This is helpful for applications that
utilize APIs to fetch their data.
Even when server-side rendered, these applications issue network requests
against the production API endpoints to get access to data. In a lot of cases,
we can accelerate this process by forwarding the network requests directly to
the target controller/action in Rails.Instead of fetching 

**Example** A React applications queries a Rails GraphQL API

We can override the HttpLink `fetch` method and utilize the `@isorun/rails`
package to send the HTTP request for the GraphQL API directly to the Ruby
process, instead of sending it over the network.

```js
import {apollo} from "@isorun/rails";

import {App} from "../my_app/App.jsx";

const apolloClient = new ApolloClient({
  ssrMode: true,
  cache: new InMemoryCache(),
  link: new HttpLink({
    uri: 'http://localhost:3000/graphql',
    fetch: apollo.fetch
  })
});
```

```ruby
Isorun.configure do
  receiver do |request|
    query, variables, context, operation_name = parse(request)
    
    RailsAppSchema.execute(
      query,
      variables: variables,
      context: context,
      operation_name: operation_name
    )
  end
end
```

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
