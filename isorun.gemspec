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

  spec.summary = "A Ruby on Rails render helper for JavaScript applications."
  spec.description = <<~DESC
    An embedded V8 runtime, used to render JavaScript applications directly in
    the current Ruby process.
  DESC
  spec.homepage = "https://github.com/eliias/isorun"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 2.7.0"

  spec.metadata["allowed_push_host"] = "https://rubygems.org"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/eliias/isorun"
  spec.metadata["changelog_uri"] = "https://github.com/eliias/isorun"
  spec.metadata["documentation_uri"] = "https://eliias.github.io/isorun"

  spec.files = Dir["{app,lib}/**/*", "ext/**/*.{js,rs,toml,lock,rb}", "LICENSE", "Rakefile", "README.md"]

  spec.bindir = "exe"
  spec.require_paths = ["lib"]

  spec.metadata["rubygems_mfa_required"] = "true"

  spec.add_dependency "actionpack", ">= 6.0.0"
  spec.add_dependency "railties", ">= 6.0.0"
  spec.add_dependency "rake", "~> 13.0"
  spec.add_dependency "rb_sys", "~> 0.9.44"

  spec.add_development_dependency "rake-compiler"
  spec.add_development_dependency "rake-compiler-dock"
  spec.add_development_dependency "rspec-rails"

  spec.extensions = ["ext/isorun/extconf.rb"]
end
