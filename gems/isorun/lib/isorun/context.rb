# frozen_string_literal: true

module Isorun
  class Context
    class Import
      attr_reader :context, :export_names

      def initialize(context, export_names)
        @context = context
        @export_names = export_names
      end

      def from(module_path)
        mod = load(module_path)
        imports = export_names.map { |export_name| mod.import(export_name) }
        return imports.first if imports.size == 1

        imports
      end

      private

      CACHE_KEY = "isorun_module_path_mtime"
      private_constant :CACHE_KEY

      def load(module_path)
        return context.load(module_path) if Rails.env.production?

        key = module_path.parameterize

        file = File.open(module_path)
        mtime = file.mtime.to_i

        cache_miss = false

        prev_mtime = Rails.cache.fetch("#{CACHE_KEY}:#{key}") do
          cache_miss = true
          mtime
        end

        # map to URI scheme to allow adding a timestamp to bust module cache
        module_path = "file://#{module_path}?t=#{mtime}"

        return context.load(module_path) if cache_miss

        Rails.cache.write("#{CACHE_KEY}:#{key}", mtime) if prev_mtime < mtime

        context.load(module_path)
      end
    end

    private_constant :Import

    class << self
      def create(&block)
        raise "[Isorun::Context] block missing when creating context" unless block

        context = Isorun::Context.new
        yield(context)
      end

      # @!method new()
      # @return [Isorun::Context] the newly created context

      private

      def default_options
        {
          receiver: Isorun.configuration.receiver
        }
      end
    end

    def import(*export_names)
      export_names = [*export_names].map(&:to_s)
      export_names = [:default.to_s] if export_names.empty?
      Import.new(self, export_names)
    end
  end

  # @!method receiver=(receiver)
  # @param receiver [Proc]
  # @return [Isorun::Context] the newly created context
end
