class MedicalTreatmentType < ActiveRecord::Base

	belongs_to :user
	has_many :medical_treatments
	
end
