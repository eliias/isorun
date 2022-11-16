# frozen_string_literal: true

# load native extension
begin
  ruby_version = /(\d+\.\d+)/.match(::RUBY_VERSION)
  require_relative "#{ruby_version}/isorun/isorun"
rescue LoadError
  require "isorun/isorun"
end

require_relative "isorun/app"
require_relative "isorun/renderer"
require_relative "isorun/version"

require "isorun/engine"

module Isorun
  extend ActiveSupport::Autoload

  class Error < StandardError; end
end
