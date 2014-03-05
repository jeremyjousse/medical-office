class AddMedicalTreatmentTypeIdToMedicalTreatments < ActiveRecord::Migration
  def change
  	add_column :medical_treatments, :medical_treatment_type_id, :integer
  end
end
