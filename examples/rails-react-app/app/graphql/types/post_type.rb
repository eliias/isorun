# frozen_string_literal: true

module Types
  class PostType < Types::BaseObject
    field :title, String
    field :rating, Integer
    field :comments, [Types::CommentType]
  end
end
