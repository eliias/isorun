# frozen_string_literal: true

module Isorun
  class Renderer # rubocop:disable Lint/EmptyClass
    # @!method render(bundle_path, proc)
    #   Renders the JavaScript application, takes an optional callback block to
    #   control aspects of the render process (e.g. fetching data).
    #
    #   @example Render and intercept network requests to the GraphQL API
    #     renderer = Renderer.new
    #     renderer.render(bundle_path) do |action, *args|
    #       if action == "fetch"
    #         execute_graphql(*args)
    #       end
    #     end
    #
    # @param [String] bundle_path
    # @param [Proc] block
    # @return [String] The rendered result as String
    # @!visibility private
  end
end
