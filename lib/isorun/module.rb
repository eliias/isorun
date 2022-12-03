# frozen_string_literal: true

module Isorun
  class Module
    class ModuleMissingError < StandardError
      def initialize
        super("Module does not exist. Check output of asset pipeline.")
      end
    end

    attr_writer :bundle_resolver, :message_receiver

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

      module_call(bundle_path, entrypoint, message_receiver, args, kwargs, block)
    end

    protected

    def resolve_bundle_path(id)
      bundle_resolver.call(id)
    end

    private

    def bundle_path
      resolve_bundle_path(id)
    end

    # @!attribute [r] id
    #   @return [String]

    # @!attribute [r] entrypoint
    #   @return [String]

    # @!method initialize(bundle_path, entrypoint)

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
