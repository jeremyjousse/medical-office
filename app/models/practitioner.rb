class Practitioner < ActiveRecord::Base

	belongs_to :country
  belongs_to :user
  has_many :patients
  

  SPECIALITY = {0 => "Undefined", 1 => "Family doctor", 2 => "Physical Therapists", 3 => "Osteopath"}

end


