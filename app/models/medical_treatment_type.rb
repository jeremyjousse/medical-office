class MedicalTreatmentType < ActiveRecord::Base

	belongs_to :user
	has_many :medical_treatments, :dependent => :restrict_with_error
	
	validates :name, :presence => true, :length => 2..255
	validates :price, :presence => true


	def as_json(options)
    	{ id: id, text: price }
  	end

end
