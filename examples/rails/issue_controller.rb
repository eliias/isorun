# frozen_string_literal: true

class IssueController
  def show
    state = State.find

    respond_to do |format|
      format.turbo_stream { render turbo_app: app.render(state) }
    end
  end

  private

  def app
    @app ||= IssueApp.new("assets/issue.bundle.js")
  end
end
