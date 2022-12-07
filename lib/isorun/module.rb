# frozen_string_literal: true

module Isorun
  class Module
    class ModuleMissingError < StandardError
      def initialize
        super("Module does not exist. Check output of asset pipeline.")
      end
    end

    CallOptions = Struct.new(
      :environment,
      :bundle_path,
      :message_receiver,
      :force_reload,
      keyword_init: true
    )
    DEFAULT_ENTRYPOINT = "default" # the default ECMAScript module export name
    CACHE_KEY = "isorun_asset_mtime_cache"
    private_constant :CallOptions, :DEFAULT_ENTRYPOINT, :CACHE_KEY

    attr_writer :bundle_resolver, :message_receiver

    def self.load(id)
      resolver = Isorun.configuration.bundle_resolver
      module_path = resolver.call(id)

      Module.new(id, module_path)
    end

    def bundle_resolver
      @bundle_resolver || Isorun.configuration.bundle_resolver
    end

    def message_receiver
      @message_receiver || Isorun.configuration.message_receiver
    end

    def exist?
      File.exist?(bundle_path)
    end

    # def import(export_name)
    #   module_import(export_name)
    # end

    private

    def bundle_path
      bundle_resolver.call(id)
    end

    def environment
      Rails.env.to_s
    end

    def force_reload # rubocop:disable Metrics/MethodLength
      return false if Rails.env.production?

      file = File.open(bundle_path)
      mtime = file.mtime

      cache_miss = false

      prev_mtime = Rails.cache.fetch("#{CACHE_KEY}:#{id}") do
        cache_miss = true
        mtime
      end

      return true if cache_miss

      if prev_mtime < mtime
        Rails.cache.write("#{CACHE_KEY}:#{id}", mtime)
        return true
      end

      false
    end

    # @!attribute [r] id
    #   @return [String]

    # @!method initialize(id)

    # @!method module_import(export_name)
    #
    # @param export_name [String]
    # @return [Isorun::ModuleItem] The exported module item
    # @!visibility private
  end
end
