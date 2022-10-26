# frozen_string_literal: true

begin
  require_relative "lib/isorun/version"
rescue LoadError
  puts "WARNING: Could not load Isorun::VERSION"
end

Gem::Specification.new do |spec|
  spec.name = "isorun"
  spec.version = defined?(Isorun::VERSION) ? Isorun::VERSION : "0.0.0"
  spec.authors = ["Hannes Moser"]
  spec.email = ["box@hannesmoser.at"]

  spec.summary = "V8 runtime for Ruby on Rails."
  spec.description = "A V8 runtime used to render JavaScript applications on the server."
  spec.homepage = "https://github.com/eliias/isorun"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 2.7.0"
  # https://github.com/rubygems/rubygems/pull/5852#issuecomment-1231118509
  spec.required_rubygems_version = ">= 3.3.21"

  spec.metadata["allowed_push_host"] = "https://rubygems.org"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/eliias/isorun"
  spec.metadata["changelog_uri"] = "https://github.com/eliias/isorun"

  spec.files = Dir["{app,lib}/**/*", "ext/**/*.{rs,toml,lock,rb}"]

  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]

  spec.metadata["rubygems_mfa_required"] = "true"

  spec.add_dependency "rake", "> 1"
  spec.add_dependency "rb_sys", "~> 0.9.31"

  spec.add_development_dependency "rails", "~> 7.0.4"
  spec.add_development_dependency "rake-compiler", "~> 1.2.0"
  spec.add_development_dependency "rake-compiler-dock", "~> 1.2.2"

  spec.extensions = ["ext/isorun/extconf.rb"]
end
