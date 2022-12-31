# frozen_string_literal: true

require "rails/engine"

module Isorun
  class Engine < Rails::Engine
    isolate_namespace Isorun
    config.eager_load_namespaces << Isorun
    config.autoload_once_paths = %W[
      #{root}/app/helpers
    ]

    initializer "isorun.helpers", before: :load_config_initializers do
      ActiveSupport.on_load(:action_controller_base) do
        helper Isorun::Engine.helpers
      end
    end
  end
end
