# A Simple Blog Post
class Post < ActiveRecord::Base
  validates :title, :body, :published, presence: true
end
