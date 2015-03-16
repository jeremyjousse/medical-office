class AddTitleToDocuementScans < ActiveRecord::Migration
  def change
    add_column :document_scans, :title, :string
  end
end
