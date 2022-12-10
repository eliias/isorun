# frozen_string_literal: true

# set environment variable to isorun gem path, we use this to request the main
# module in the native extension
ENV["ISORUN_NATIVE_GEM_PATH"] = File.expand_path("..", __dir__)

# load native extension
begin
  ruby_version = /(\d+\.\d+)/.match(::RUBY_VERSION)
  require "isorun/#{ruby_version}/isorun"
rescue LoadError
  require "isorun/isorun"
end

require "isorun/config"
require "isorun/context"
require "isorun/engine"
require "isorun/function"
require "isorun/module"
require "isorun/version"

module Isorun
  extend ActiveSupport::Autoload

  class Error < StandardError; end

  def self.with_receiver(receiver)
    self.receiver = receiver
    result = yield
    self.receiver = nil
    result
  end
end
