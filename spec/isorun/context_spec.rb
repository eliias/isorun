# frozen_string_literal: true

RSpec.describe Isorun::Context do
  let(:module_say) { Rails.root / "app" / "javascript" / "say.js" }
  let(:module_values) { Rails.root / "app" / "javascript" / "values.js" }

  it "creates a new context" do
    expect { described_class.new }.not_to raise_error
  end

  context "when importing from a module" do
    it "import a function" do
      described_class.create do |context|
        say = context.import(:say).from(module_say)

        expect(say).to be_a Isorun::Function
      end
    end

    it "imports null value" do
      described_class.create do |context|
        value = context.import(:tNull).from(module_values)

        expect(value).to be_nil
      end
    end

    it "imports boolean value" do
      described_class.create do |context|
        value = context.import(:tBoolean).from(module_values)

        expect(value).to be(true)
      end
    end

    it "imports numeric value" do
      described_class.create do |context|
        value = context.import(:tNumber).from(module_values)

        expect(value).to eq(1.0)
      end
    end

    it "imports string value" do
      described_class.create do |context|
        value = context.import(:tString).from(module_values)

        expect(value).to eq("A String")
      end
    end

    it "imports array values" do
      described_class.create do |context|
        value = context.import(:tArray).from(module_values)

        expect(value).to eq([1, 2, 3])
      end
    end

    it "imports object values" do
      described_class.create do |context|
        value = context.import(:tObject).from(module_values)

        expect(value).to eq({ "a" => 1, "b" => 2 })
      end
    end
  end
end
