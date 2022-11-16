# frozen_string_literal: true

module Isorun
  module AppsHelper
    def isorun_app_tag(id)

      vm = Container.vm(id)

      html = ""

      html += tag.div id: id do
        vm.render do |action, options|
          puts "intercepted '#{action}' to: '#{options}'"

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

    class Container
      class << self
        def vm(id)
          @vm ||= {}
          @vm[id] ||= begin
            bundle_path = resolve_bundle_path(id)
            Isorun::Renderer.new(bundle_path)
          end
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
  end
end
