class AddStatusToMedicalTreatments < ActiveRecord::Migration
  def change
  	add_column :medical_treatments, :status, :integer
  end
end
