class MedicalTreatment < ActiveRecord::Base

  belongs_to :user
  belongs_to :patients
  
end
