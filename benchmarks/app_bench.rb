# frozen_string_literal: true

require 'benchmark/ips'
require "bundler/setup"
require "isorun"

vm1 = Isorun::VM.new
vm1.load("basic_vue_app", "examples/vuejs/dist/main.js")

vm2 = Isorun::VM.new
vm2.load("list_vue_app", "examples/vuejs-list/dist/main.js")

Benchmark.ips do |x|
  x.config(time: 10, warmup: 1)
  x.report("render basic vue app") { vm1.render("basic_vue_app") }
  x.report("render list vue app") { vm2.render("list_vue_app") }

  x.compare!
end
