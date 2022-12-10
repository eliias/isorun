# frozen_string_literal: true

module Isorun
  module Resolver
    SSR_APP_RESOLVER = lambda { |bundle_id|
      if Rails.env.development?
        Rails.root.join("app", "assets", "builds", "#{bundle_id}-server.js").to_s
      else
        asset_path("#{bundle_id}-server")
      end
    }

    SIMPLE_RESOLVER = lambda { |bundle_id|
      if Rails.env.development?
        Rails.root.join("app", "assets", "builds", "#{bundle_id}.js").to_s
      else
        asset_path(bundle_id)
      end
    }

    private

    def asset_path(asset)
      asset_path = Rails.application.assets_manifest.assets["#{asset}.js"]
      "#{Rails.application.assets_manifest.directory}/#{asset_path}"
    end
  end
end
