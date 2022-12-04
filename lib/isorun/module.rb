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
      :entrypoint,
      :message_receiver,
      :force_reload,
      keyword_init: true
    )

    # the default ECMAScript module export name
    DEFAULT_ENTRYPOINT = "default"

    CACHE_KEY = "isorun_asset_mtime_cache"

    private_constant :CallOptions, :DEFAULT_ENTRYPOINT, :CACHE_KEY

    attr_writer :entrypoint, :bundle_resolver, :message_receiver

    def bundle_resolver
      @bundle_resolver || Isorun.configuration.bundle_resolver
    end

    def message_receiver
      @message_receiver || Isorun.configuration.message_receiver
    end

    def exist?
      File.exist?(bundle_path)
    end

    def call(*args, **kwargs, &block)
      raise ModuleMissingError unless exist?

      options = CallOptions.new(
        bundle_path: bundle_path,
        environment: environment,
        entrypoint: entrypoint,
        message_receiver: message_receiver,
        force_reload: force_reload
      )

      module_call(options, args, kwargs, block)
    end

    protected

    def resolve_bundle_path(id)
      bundle_resolver.call(id)
    end

    private

    def bundle_path
      resolve_bundle_path(id)
    end

    def environment
      Rails.env.to_s
    end

    def entrypoint
      @entrypoint || DEFAULT_ENTRYPOINT
    end

    def force_reload
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

    # @!method module_call(bundle_path, entrypoint, *args, **kwargs, &block)
    #   Executes the module by invoking the function declared as entrypoint
    #   control aspects of the render process (e.g. fetching data).
    #
    # @param entrypoint [String]
    # @param *args [Array<Object>] Any number of Objects
    # @param *kwargs [Hash] Any number of named Objects
    # @yield [Array<Object>] Any number of Objects
    # @return [Object] Any Object
    # @!visibility private
  end
end
