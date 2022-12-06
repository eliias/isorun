# frozen_string_literal: true

module Isorun
  class ModuleItem
    def call(*args, **kwargs, &block)
      module_item_call(args, kwargs, block)
    end

    private # rubocop:disable Lint/UselessAccessModifier

    # @!method module_item_call(args, kwargs, block)
    #
    # @param *args [Array<Object>] Any number of Objects
    # @param *kwargs [Hash] Any number of named Objects
    # @return [Object] The result returned by call
    # @!visibility private
  end
end
