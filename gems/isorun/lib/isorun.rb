# frozen_string_literal: true

require "isorun-native" # native extension
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
