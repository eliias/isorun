# frozen_string_literal: true

module Isorun
  module AppHelper
    def isorun_app(id) # rubocop:disable Metrics/AbcSize, Metrics/MethodLength
      module_path = Isorun.configuration.module_resolver.call(id)

      ssr_html = Isorun::Context.create do |context|
        render_context = { environment: Rails.env.to_s }
        render_function = context.import.from(module_path)

        if render_function.blank?
          Rails.logger.warn("[ISORUN] the requested app does not exist or " \
                            "does not have a server entrypoint. Please " \
                            "check if an asset with filename " + "
                               `#{id}-server.js` exists.")
        end

        render_function.call_without_gvl(
          render_context,
          Isorun.configuration.receiver
        )
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
