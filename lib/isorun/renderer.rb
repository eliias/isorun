# frozen_string_literal: true

module Isorun
  class Renderer
    def render(&block)
      renderer_render(block.to_proc)
    end
  end

  # @!method renderer_render(&block)
  #   Renders the JavaScript application, takes an optional callback block to
  #   control aspects of the render process (e.g. fetching data).
  #
  #   @example Render and intercept network requests to the GraphQL API
  #     renderer = Renderer.new
  #     renderer.render do |action, *args|
  #       if action == "fetch"
  #         execute_graphql(*args)
  #       end
  #     end
  #
  # @return [String] The rendered result as String
  # @!visibility private
end
