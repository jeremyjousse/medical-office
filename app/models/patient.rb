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
  

	scope :finder, lambda { |q| where("last_name like :q", q: "%#{q}%") }

	def as_json(options)
    { id: id, text: first_name + ' ' + last_name + '(' + city + ')' }
  end

  def full_name_for_select
  	self.first_name + ' ' + self.last_name + ' (' + self.city + ')'
  end

end