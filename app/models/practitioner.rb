class Practitioner < ActiveRecord::Base

	belongs_to :country
	belongs_to :user
	has_many :patients
  
  	validates :first_name, :presence => true, :length => 2..255
	validates :last_name, :presence => true, :length => 2..255
	validates :speciality_id, :presence => true
	validates :address, :presence => true, :length => 2..255
	validates :postal_code, :presence => true, :length => 2..255
	validates :city, :presence => true, :length => 2..255
	validates :country_id, :presence => true

	SPECIALITY = {0 => "Undefined", 1 => "Family doctor", 2 => "Physical Therapists", 3 => "Osteopath"}

end


