# frozen_string_literal: true

Isorun.configure do
  # when the JavaScript application sends a message to ruby, we can decide to
  # respond to a given action and the arguments provided by the action
  #
  # @example
  #   on_app_send do |action, args|
  #     case action
  #       when "fetch"
  #       { data: { testField: "Hello from isorun" } }.to_json
  #     else
  #       ""
  #     end
  #   end
  on_app_send do |action, args|
    url, options = JSON.parse!(args)
                       .with_indifferent_access
                       .values_at(:url, :options)
    url = URI.parse(url)

    case action
    when "fetch"
      session = ActionDispatch::Integration::Session.new(Rails.application)
      session.host!("localhost:3000")
      session.process(
        options[:method], url.path, params: JSON.parse!(options[:body])
      )
      session.response.body
    else
      ""
    end
  end
end
