# frozen_string_literal: true

module Isorun
  module AppsHelper
    def isorun_app_tag(id)
      app = Isorun::App.new(resolve_bundle_path(id))

      html = ""
      html += tag.div id: id do
        app.render do |action, options|
          Rails.logger.debug { "intercepted '#{action}' to: '#{options}'" }

          {
            data: {
              testField: "Hello World!"
            }
          }.to_json
        end.html_safe
      end

      html += "\n"
      html += javascript_include_tag(id, defer: true)
      html.html_safe
    end

    private

    def resolve_bundle_path(id)
      if Rails.env.development?
        Rails.root.join("app", "assets", "builds", "#{id}-server.js").to_s
      else
        javascript_path(id)
      end
    end
  end
end
