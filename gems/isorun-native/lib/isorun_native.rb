# frozen_string_literal: true

# set environment variable to isorun gem path, we use this to request the main
# module in the native extension
ENV["ISORUN_NATIVE_GEM_PATH"] = File.expand_path("..", __dir__)

# load native extension
begin
  ruby_version = /(\d+\.\d+)/.match(::RUBY_VERSION)
  require "isorun_native/#{ruby_version}/isorun_native"
rescue LoadError
  require "isorun_native/isorun_native"
end

module IsorunNative
end
