# frozen_string_literal: true

require_relative "lib/isorun/version"

Gem::Specification.new do |spec|
  spec.name = "isorun"
  spec.version = Isorun::VERSION
  spec.authors = ["Hannes Moser"]
  spec.email = ["box@hannesmoser.at"]

  spec.summary = "Run JavaScript applications in your Rails application."
  spec.description = <<~DESC
    Import ECMAScript modules into Ruby and use values and functions like
    JavaScript is part of Ruby. Enables easy to set up server-side rendering for
    modern frontend stacks.

    Isorun embeds v8 into Ruby via a native extension built with Rust and
    deno_core.
  DESC
  spec.homepage = "https://github.com/eliias/isorun"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 3.0.0"

  spec.metadata["allowed_push_host"] = "https://rubygems.org"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/eliias/isorun"
  spec.metadata["changelog_uri"] = "https://github.com/eliias/isorun"
  spec.metadata["documentation_uri"] = "https://eliias.github.io/isorun"

  spec.files = Dir[
    "{app,lib}/**/*",
    "ext/**/*.{js,rs,toml,lock,rb}",
    "LICENSE",
    "Rakefile",
    "README.md"
  ]

  spec.metadata["rubygems_mfa_required"] = "true"

  spec.add_dependency "railties", ">= 6.0.0"
  spec.add_dependency "rake", "~> 13.0"
  spec.add_dependency "rb_sys", "~> 0.9.81"

  spec.add_development_dependency "rails"
  spec.add_development_dependency "rake-compiler"
  spec.add_development_dependency "rake-compiler-dock"
  spec.add_development_dependency "rspec"
  spec.add_development_dependency "rspec-rails"
  spec.add_development_dependency "simplecov", "~> 0.22.0"

  spec.extensions = ["ext/isorun/extconf.rb"]
end
