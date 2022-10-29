# frozen_string_literal: true

# Read more: https://github.com/cyu/rack-cors
#
Rails.application.config.middleware.insert_before 0, Rack::Cors do
  allow do
    origins "localhost:3000"

    resource "/graphql",
             headers: :any,
             methods: %i[get post options head]
  end
end
