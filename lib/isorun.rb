# frozen_string_literal: true

# load native extension
begin
  ruby_version = /(\d+\.\d+)/.match(::RUBY_VERSION)
  require "#{ruby_version}/isorun/isorun"
rescue LoadError
  require "isorun/isorun"
end

require "isorun/app"
require "isorun/engine"
require "isorun/renderer"
require "isorun/version"

module Isorun
  extend ActiveSupport::Autoload

  class Error < StandardError; end
end
