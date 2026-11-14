# frozen_string_literal: true

require "mkmf"
require "rb_sys/mkmf"

create_rust_makefile("isorun/isorun") do |r|
  # A Ruby built by GCC 15 (C23 by default) expects `bool` to be a keyword, and
  # its headers do not define it. bindgen parses the headers as C17 otherwise.
  r.env = { "BINDGEN_EXTRA_CLANG_ARGS" => "-std=gnu2x" }
end
