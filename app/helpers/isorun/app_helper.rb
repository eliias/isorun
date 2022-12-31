# frozen_string_literal: true

module Isorun
  module AppHelper
    # The isorun_app helper is the most convenient way to server-side render
    # a JavaScript application, including state extraction and automatic
    # rehydration. The helper tries to render the application and will also
    # inject the client-side code immediately after the rendered result.
    #
    # @example Renders a JavaScript application
    #   <html>
    #   <body>
    #     <%= isorun_app("my_app") %>
    #   </body>
    #   </html>
    #
    # @param id [String] An ID representing both, the asset bundle, and by
    #   convention, the target node (e.g. `<div id="my_app">`)
    # @return [String]
    def isorun_app(id) # rubocop:disable Metrics/AbcSize, Metrics/MethodLength
      ActiveSupport::Notifications.instrument "start.render.isorun", { ts: Time.current }

      module_path = Isorun.config.module_resolver.call(id)

      ssr_html = Isorun::Context.create(receiver: Isorun.config.receiver) do |context|
        render_context = { environment: Rails.env.to_s }
        render_function = context.import.from(module_path)

        if render_function.blank?
          Rails.logger.warn("[ISORUN] the requested app does not exist or " \
                            "does not have a server entrypoint. Please " \
                            "check if an asset with filename " + "
                               `#{id}-server.js` exists.")
          return ""
        end

        html = render_function.call(render_context)

        ActiveSupport::Notifications.instrument "finish.render.isorun", { ts: Time.current }
        ActiveSupport::Notifications.instrument "stats.isorun", Isorun.stats

        html
      end

      html = if ssr_html.present?
               tag.div id: id do
                 ssr_html.html_safe # rubocop:disable Rails/OutputSafety
               end
             else
               Rails.logger.warn("[ISORUN] The server-side rendered result is empty.")
               ""
             end

      html += "\n"
      html += javascript_include_tag(id, defer: true)
      html.html_safe # rubocop:disable Rails/OutputSafety
    end
  end
end
