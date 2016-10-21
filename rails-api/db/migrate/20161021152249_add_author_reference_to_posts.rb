# Relates authors and posts
class AddAuthorReferenceToPosts < ActiveRecord::Migration
  def change
    add_reference :posts, :author
  end
end
