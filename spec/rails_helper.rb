# frozen_string_literal: true

require "spec_helper"

ENV["RAILS_ENV"] ||= "test"

require File.expand_path("./dummy/config/environment", __dir__)

# Prevent database truncation if the environment is production
abort("The Rails environment is running in production mode!") if Rails.env.production?

require "rspec/rails"

require "simplecov"

SimpleCov.start "rails"
SimpleCov.formatters = SimpleCov::Formatter::MultiFormatter.new([SimpleCov::Formatter::HTMLFormatter])

# autoload support files
Dir[Rails.root / "spec" / "support" / "**" / "*.rb"].sort.each { |f| require f }

RSpec.configure(&:filter_rails_from_backtrace!)
