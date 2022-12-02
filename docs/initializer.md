# Initializer

## Don't do this

A somewhat trivial way to forward and process fetch requests from JavaScript
clients is instantiating a test session, and returning the response. There are
some caveats though. The Rails render engine utilizes `ruby-concurrent` to check
for caches and this leads to an invalid attempt to release a read lock.
a concurrent way 

```ruby
session = ActionDispatch::Integration::Session.new(Rails.application)
session.host!("localhost:3000")
session.process(
  options[:method], url.path, params: JSON.parse!(options[:body])
)
session.response.body
```
