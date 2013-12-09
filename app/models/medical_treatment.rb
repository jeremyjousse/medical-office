class MedicalTreatment < ActiveRecord::Base

  belongs_to :user
  belongs_to :patient
  belongs_to :treatment_type, class_name: :medical_treatment_type
  
end
