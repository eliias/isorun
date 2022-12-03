# frozen_string_literal: true

module Isorun
  module Resolver
    SSR_APP_RESOLVER = lambda { |bundle_id|
      if Rails.env.development?
        Rails.root.join("app", "assets", "builds", "#{bundle_id}-server.js").to_s
      else
        javascript_path("#{bundle_id}-server")
      end
    }

    SIMPLE_RESOLVER = lambda { |bundle_id|
      if Rails.env.development?
        Rails.root.join("app", "assets", "builds", "#{bundle_id}.js").to_s
      else
        javascript_path(bundle_id)
      end
    }
  end
end
