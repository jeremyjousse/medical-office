class Patient < ActiveRecord::Base
	belongs_to :country
	belongs_to :user
	belongs_to :family_doctor, class_name: :Practitioner
	belongs_to :physical_therapists, class_name: :Practitioner
	belongs_to :osteopath, class_name: :Practitioner
	has_many :medical_treatments

	validates :first_name, :presence => true, :length => 2..255
	validates :last_name, :presence => true, :length => 2..255
	validates :city, :presence => true, :length => 2..255
	validates :country_id, :presence => true
 # validates :address, :presence => true, :length => 2..255, :on => :update
 #  validates :postal_code, :presence => true, :length => 2..100, :on => :update
 #  validates :city, :presence => true, :length => 2..255, :on => :update
 #  validates :country_id, :presence => true, :on => :update
 #  validates :phone, :presence => true, :length => 5..100, :on => :update
 #  validates :email, :format => { :with => /\A([^@\s]+)@((?:[-a-z0-9]+\.)+[a-z]{2,})\z/i } , :length => 5..255
  
end