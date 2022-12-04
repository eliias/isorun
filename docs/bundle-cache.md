# Bundle cache

In a `production` environment, bundles are never reloaded, but use the import
cache. This means that once a bundle has been loaded into the JavaScript VM,
it will never update, even when the underlying file changes.

In a `development` environment, bundles are forced to load on every single
request. This leads to relatively slow response times due to the VM cold-start.
*isorun* supports caching of bundles in development mode for environments that
configure a cache.

```ruby
# development.rb
config.cache_store = :memory_store
```
