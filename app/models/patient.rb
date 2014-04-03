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


  	def self.import_from_google_contacts(file, user)
		require 'CSV'

		imported_lines = nil

		

		CSV.foreach(file.path, {:encoding => 'bom|utf-16le', headers: true}) do |row|

			if !row["Family Name"].nil?
				patient_exists = Patient.where("user_id = :user_id AND last_name LIKE :last_name AND first_name LIKE :first_name AND phone LIKE :phone",
  {user_id: user, last_name: row["Family Name"], first_name: row["Given Name"], phone: row["Phone 1 - Value"]})

				if patient_exists.count == 0
					

					patient = Patient.new
					patient.user_id = user
					patient.first_name = row["Given Name"]
					patient.last_name = row["Family Name"]
					patient.birthdate = row["Birthday"]
					patient.profession = row["Occupation"]
					patient.email = row["E-mail 1 - Value"]
					patient.phone = row["Phone 1 - Value"]
					patient.mobile_phone = row["Phone 2 - Value"]
					patient.address = row["Address 1 - Street"]
					patient.postal_code = row["Address 1 - Postal Code"]
					patient.city = row["Address 1 - City"]
					patient.note = row["Notes"]
					#patient.mobile_phone = row["Address 1 - Postal Country"]

					patient.save(validate: false)

					

					

				end
			end
# Given Name
# Family Name
# E-mail 1 - Value
# Phone 1 - Value
# Phone 2 - Value
# Address 1 - Street
# Address 1 - City
# Address 1 - Postal Code
# Address 1 - Country


			#logger.info '-------' + row.inspect

		end


  	end

end