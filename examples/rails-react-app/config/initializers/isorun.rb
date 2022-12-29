# frozen_string_literal: true

Isorun.configure do
  # when the JavaScript application sends a message to ruby, we can decide to
  # respond to a given action and the arguments provided by the action
  #
  # @example
  #   receiver do |action, args|
  #     case action
  #       when "fetch"
  #       { data: { testField: "Hello from isorun" } }.to_json
  #     else
  #       ""
  #     end
  #   end
  receiver do |message|
    action, args = message.with_indifferent_access.values_at(:action, :args)

    case action
    when "test"
      args
    when "fetch"
      options, = args.with_indifferent_access
                     .values_at(:options)
      body, = JSON.parse!(options).with_indifferent_access
                  .values_at(:body)

      context = {}
      operation_name, query, variables = JSON.parse!(body)
                                             .with_indifferent_access
                                             .values_at(:operation_name, :query, :variables)

      puts "[ISORUN] Process JavaScript GraphQL request:\n\n#{query}\n\n"

      result = RailsAppSchema.execute(
        query,
        variables: variables,
        context: context,
        operation_name: operation_name
      )

      result.to_json
    end
  rescue StandardError => e
    Rails.logger.error("[ISORUN] Cannot process received message: #{e.message}\n\n#{e.backtrace&.join("\n")}")
  end
end
