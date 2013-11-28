class AddCreatedAtAndUpdatedAtToPatients < ActiveRecord::Migration
  def change
  	add_column :patients, :created_at, :datetime
  	add_column :patients, :updated_at, :datetime
  end
end
