# frozen_string_literal: true

RSpec.describe Isorun::VM do
  it "initialize vm and render vanilla JS" do
    vm = described_class.new
    vm.load("my_app", "examples/vanillajs/index.js")

    html = vm.render("my_app")

    expect(html).to eq("<h1>Hello, World!</h1>")
  end

  it "basic vue app" do
    vm = described_class.new
    vm.load("my_app", "examples/vuejs/dist/main.js")
    html = vm.render("my_app")

    expect(html).to include("Hello, World!")
  end

  it "vue app with list" do
    vm = described_class.new
    vm.load("my_app", "examples/vuejs-list/dist/main.js")
    html = vm.render("my_app")

    expect(html).to include("You’ve successfully created a project with")
  end
end
