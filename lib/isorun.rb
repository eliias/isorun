# frozen_string_literal: true

# load native extension
begin
  ruby_version = /(\d+\.\d+)/.match(::RUBY_VERSION)
  require_relative "#{ruby_version}/isorun/isorun"
rescue LoadError
  require "isorun/isorun"
end

require_relative "isorun/vm"
require_relative "isorun/version"

module Isorun
  class Error < StandardError; end
end
