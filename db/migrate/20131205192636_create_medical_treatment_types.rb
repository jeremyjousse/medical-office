class CreateMedicalTreatmentTypes < ActiveRecord::Migration
  def change
    create_table :medical_treatment_types do |t|
      t.string :name
      t.decimal :price
      t.integer :user_id

      t.timestamps
    end
  end
end
