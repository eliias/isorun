# frozen_string_literal: true

require_relative "lib/isorun_native/version"

Gem::Specification.new do |spec|
  spec.name = "isorun-native"
  spec.version = IsorunNative::VERSION
  spec.authors = ["Hannes Moser"]
  spec.email = ["box@hannesmoser.at"]

  spec.summary = "Run JavaScript applications in your Rails application."
  spec.description = <<~DESC
    Import ECMAScript modules into Ruby and use values and functions like#{' '}
    JavaScript is part of Ruby. Enables easy to set up server-side rendering for
    modern frontend stacks.

    Isorun embeds v8 into Ruby via a native extension built with Rust and
    deno_core.
  DESC
  spec.homepage = "https://github.com/eliias/isorun"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 2.7.0"

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

  spec.add_dependency "rake", "~> 13.0"
  spec.add_dependency "rb_sys", "~> 0.9.46"

  spec.add_development_dependency "rake-compiler"
  spec.add_development_dependency "rake-compiler-dock"

  spec.extensions = ["ext/isorun_native/extconf.rb"]
end
