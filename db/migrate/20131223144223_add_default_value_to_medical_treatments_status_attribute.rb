class AddDefaultValueToMedicalTreatmentsStatusAttribute < ActiveRecord::Migration
	def up
	    change_column :medical_treatments, :status, :integer, :default => 0

	    medical_treatments = MedicalTreatment.where(status: nil)

	    medical_treatments.each do |medical_treatment|
	       medical_treatment.status = 0
	       medical_treatment.save
	    end


	end

	def down
	    change_column :medical_treatments, :status, :integer, :default => nil
	end
end
