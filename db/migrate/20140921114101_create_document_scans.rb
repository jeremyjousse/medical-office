class CreateDocumentScans < ActiveRecord::Migration
  def change
    create_table :document_scans do |t|
      t.references :patient, index: true
      t.references :user, index: true
      t.string :file
      t.timestamps
    end
  end
end
