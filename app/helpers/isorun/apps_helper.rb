# frozen_string_literal: true

module Isorun
  module AppsHelper
    def isorun_app_tag(id) # rubocop:disable Metrics/MethodLength
      app = Isorun::App.new(id)

      html = ""
      if app.exist?
        html += tag.div id: id do
          app
            .render(&Isorun.configuration.on_app_send)
            .html_safe # rubocop:disable Rails/OutputSafety
        end
      else
        Rails.logger.warn("[ISORUN] the requested app does not exist or " \
                             "does not have a server entrypoint. Please " \
                             "check if an asset with filename " + "
                               `#{id}-server.js` exists.")
      end

      html += "\n"
      html += javascript_include_tag(id, defer: true)
      html.html_safe # rubocop:disable Rails/OutputSafety
    end
  end
end
