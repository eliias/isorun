# frozen_string_literal: true

module Isorun
  class App
    attr_reader :id, :bundle_path

    def initialize(id)
      @id = id
      @bundle_path = resolve_bundle_path(id)
    end

    def render(&block)
      render_if_bundle_exists(&block)
    end

    def exist?
      File.exist?(bundle_path)
    end

    private

    def render_if_bundle_exists(&block)
      return unless exist?

      renderer.render(bundle_path, block.to_proc)
    end

    def resolve_bundle_path(id)
      if Rails.env.development?
        Rails.root.join("app", "assets", "builds", "#{id}-server.js").to_s
      else
        javascript_path(id)
      end
    end

    def renderer
      @renderer ||= Isorun::Renderer.new
    end
  end
end
