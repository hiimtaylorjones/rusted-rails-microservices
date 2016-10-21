class Author < ActiveRecord::Base
  validates :name, :email, presence: true
end
