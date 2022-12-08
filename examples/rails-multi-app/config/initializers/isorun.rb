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
  receiver do |action, args|
    options = JSON.parse!(args)
                  .with_indifferent_access
                  .values_at(:options)
                  .first

    case action
    when "fetch"
      query, variables = JSON.parse!(options["body"])
                             .with_indifferent_access
                             .values_at(:query, :variables)

      puts "[ISORUN] process JavaScript GraphQL request:\n\nquery #{query}\n"

      result = RailsViteAppSchema.execute(
        query,
        variables: variables,
        context: {},
        operation_name: nil
      )
      result.to_json
    else
      ""
    end
  rescue StandardError => e
    Rails.logger.error("[ISORUN] Cannot process send: #{e.message}\n\n#{e.backtrace&.join("\n")}")
  end
end
