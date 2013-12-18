class MedicalTreatment < ActiveRecord::Base

	belongs_to :user
	belongs_to :patient
	belongs_to :medical_treatment_type
	has_many :payments, :dependent => :destroy

	accepts_nested_attributes_for :payments
  
	validates :patient_id, :presence => true
	validates :date, :presence => true
	validates :location_id, :presence => true
	validates :medical_treatment_type_id, :presence => true
	validates :price, :presence => true


  LOCATIONS = {0 => "Undefined", 1 => "Office", 2 => "Home"}

end
