# frozen_string_literal: true

require "isorun/config/abstract_builder"
require "isorun/config/option"
require "isorun/config/validations"

# taken from https://github.com/doorkeeper-gem/doorkeeper/blob/main/lib/doorkeeper/config.rb
module Isorun
  class MissingConfiguration < StandardError
    def initialize
      super("Configuration for isorun is missing. Do you have a isorun initializer?")
    end
  end

  class MissingConfigurationBuilderClass < StandardError; end

  class << self
    def configure(&block)
      @config = Config::Builder.new(&block).build
    end

    def configuration
      @config || (raise MissingConfiguration)
    end

    alias config configuration
  end

  class Config
    class Builder < AbstractBuilder
    end

    # Replace with `default: Builder` when we drop support of Rails < 5.2
    mattr_reader(:builder_class) { Builder }

    extend Option
    include Validations

    option :bundle_resolver, default: Isorun::Resolver::SSR_APP_RESOLVER
    option :message_receiver, default: ->(*_args, **_kwargs) {}
  end
end
