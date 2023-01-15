# frozen_string_literal: true

module Isorun
  module Resolver
    SSR_APP_RESOLVER = lambda { |bundle_id|
      if Rails.env.development?
        Rails.root.join("app", "assets", "builds", "#{bundle_id}-server.js").to_s
      else
        Isorun::Resolver.module_path("#{bundle_id}-server")
      end
    }

    SSR_VITE_APP_RESOLVER = lambda { |_bundle_id|
      Rails.public_path.join("vite-ssr/ssr.js").to_s
    }

    SIMPLE_RESOLVER = lambda { |bundle_id|
      if Rails.env.development?
        Rails.root.join("app", "assets", "builds", "#{bundle_id}.js").to_s
      else
        Isorun::Resolver.module_path(bundle_id)
      end
    }

    def self.module_path(asset)
      file = Rails.application.assets_manifest.assets["#{asset}.js"]
      "#{Rails.application.assets_manifest.directory}/#{file}"
    end
  end
end
