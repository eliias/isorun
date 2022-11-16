# frozen_string_literal: true

module Isorun
  class App
    attr_reader :bundle_path

    def initialize(bundle_path)
      @bundle_path = bundle_path
    end

    def render(&block)
      renderer.render(bundle_path, block.to_proc)
    end

    private

    def renderer
      @renderer ||= Isorun::Renderer.new
    end
  end
end
