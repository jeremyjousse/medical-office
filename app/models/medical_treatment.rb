class MedicalTreatment < ActiveRecord::Base

  belongs_to :user
  belongs_to :patient
  belongs_to :medical_treatment_type
  has_many :payments
  
  LOCATIONS = {0 => "Undefined", 1 => "Office", 2 => "Home"}

end
