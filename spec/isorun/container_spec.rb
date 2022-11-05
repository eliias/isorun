# frozen_string_literal: true

RSpec.describe Isorun::Container do
  it "initialize container and render" do
    container = described_class.new
    container.render(->(args) { puts args })
  end
end
