class CreateMedicalTreatments < ActiveRecord::Migration
  def change
    create_table :medical_treatments do |t|
      t.integer :user_id
      t.integer :patient_id
      t.date :date
      t.integer :location_id
      t.integer :treatment_type_id
      t.decimal :price
      t.text :note

      t.timestamps
    end
  end
end
