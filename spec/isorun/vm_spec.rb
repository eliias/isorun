# frozen_string_literal: true

RSpec.describe Isorun::VM do
  it "initialize vm" do
    vm = described_class.new
    vm.run
  end
end
