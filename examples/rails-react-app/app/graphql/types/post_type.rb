# frozen_string_literal: true

module Types
  class PostType < Types::BaseObject
    field :title, String
    field :rating, Integer
  end
end
