# frozen_string_literal: true

module Isorun
  module AppsHelper
    def isorun_app_tag(id)
      # bundle_path = asset_path("#{id}")

      bundle_path = if Rails.env.development?
                      Rails.root.join("app", "assets", "builds", "#{id}-server.js").to_s
                    else
                      javascript_path(id)
                    end

      vm = Isorun::VM.new
      vm.load(id, bundle_path)

      html = ""

      html += tag.div id: id do
        vm.render(id).html_safe
      end

      html += "\n"

      html += javascript_include_tag(id, defer: true)

      html.html_safe
    end
  end
end
