class PagesController < ApplicationController
  def index
    service = Isorun::Module.new("service", "generate")
    @result = service.call(1, "a", 2.0, [1, 2, "test"], a: 1, b: 2, c: {test: "me"})
  end
end
