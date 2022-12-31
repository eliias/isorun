# frozen_string_literal: true

module Isorun
  class Context
    class Import
      attr_reader :context, :export_names

      def initialize(context, export_names)
        @context = context
        @export_names = export_names
      end

      # Specify the module to import from
      def from(module_path)
        mod = load(module_path)
        imports = export_names.map { |export_name| mod.import(export_name) }
        return imports.first if imports.size == 1

        imports
      end

      private

      CACHE_KEY = "isorun_module_path_mtime"
      private_constant :CACHE_KEY

      def load(module_path) # rubocop:disable Metrics/AbcSize, Metrics/MethodLength
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

        if cache_miss
          ActiveSupport::Notifications.instrument "load.module.isorun", { path: module_path }
          return context.load(module_path)
        end

        Rails.cache.write("#{CACHE_KEY}:#{key}", mtime) if prev_mtime < mtime

        ActiveSupport::Notifications.instrument "load.module.isorun", { path: module_path }
        context.load(module_path)
      end
    end

    private_constant :Import

    class << self
      # Creates a new context and yields the context as the first argument to
      # the block.
      #
      # @example Creates a new context, imports the default as function
      #   result = Isorun::Context.create do |context|
      #     func = context.import.from(module_path)
      #     func.call("Hello, World!")
      #   end
      #
      # @yield [Isorun::Context] The newly created JavaScript context
      # @yieldreturn [Object, nil] An optional return value from the execution context
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

    # Specify items you want to import from the module. If none is specified,
    # the default export is taken.
    #
    # @example Import default export
    #   result = Isorun::Context.create do |context|
    #     item = context.import.from(module_path)
    #   end
    #
    # @example Import default export explicitly
    #   result = Isorun::Context.create do |context|
    #     item = context.import(:default).from(module_path)
    #   end
    #
    # @example Import various exports
    #   result = Isorun::Context.create do |context|
    #     hello, world = context.import(:hello, :world).from(module_path)
    #   end
    def import(*export_names)
      export_names = [*export_names].map(&:to_s)
      export_names = [:default.to_s] if export_names.empty?
      Import.new(self, export_names)
    end

    # @!method receiver=(receiver)
    #
    # @param receiver [Proc, nil]
    # @return [Isorun::Context] the newly created context
  end
end
