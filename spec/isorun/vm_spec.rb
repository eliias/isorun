# frozen_string_literal: true

RSpec.describe Isorun::VM do
  it "initialize vm" do
    code = File.read("examples/vanillajs/index.js")

    vm = described_class.new
    actual = vm.run code
    expected = "<h1>Hello, World!</h1>"

    expect(actual).to eq(expected)
  end
end
