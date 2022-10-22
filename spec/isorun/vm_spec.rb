# frozen_string_literal: true

RSpec.describe Isorun::VM do
  it "initialize vm and render vanilla JS" do
    skip

    code = File.read("examples/vanillajs/index.js")

    vm = described_class.new
    actual = vm.run code
    expected = "<h1>Hello, World!</h1>"

    expect(actual).to eq(expected)
  end

  it "initialize vm and call async render" do
    code = File.read("examples/deno/renderer.js")

    vm = described_class.new
    actual = vm.run code

    expect(actual).to eq("<h1>Some rendered stuff</h1>")
  end
end
