class Patient < ActiveRecord::Base
	belongs_to :country
	belongs_to :user
	belongs_to :family_doctor, class_name: :Practitioner
	belongs_to :physical_therapists, class_name: :Practitioner
	belongs_to :osteopath, class_name: :Practitioner
	has_many :medical_treatments
	has_many :postural_analyses

	#validates :first_name, :presence => true, :length => 2..255
	validates :last_name, :presence => true, :length => 2..255
	validates :city, :presence => true, :length => 2..255
	validates :country_id, :presence => true

	before_save :set_first_name_and_last_name_uppercase



	scope :finder, lambda { |q| where("last_name like :q", q: "%#{q}%") }

	def as_json(options)
    	{ id: id.to_s, text: first_name.to_s + ' ' + last_name.to_s + '(' + city.to_s + ')' }
  	end

	def full_name_for_select
  		self.first_name + ' ' + self.last_name + ' (' + self.city + ')'
  	end


  	def self.import_from_google_contacts(file, user_id)
		require 'CSV'
		require 'CGI'

		imported_patients = nil

		# Check user exists
		if User.find(user_id) == nil
			return false
		end



		CSV.foreach(file.path, {:encoding => 'bom|utf-16le', headers: true}) do |row|

			if !row["Family Name"].nil?
				patient_exists = Patient.where("user_id = :user_id AND last_name LIKE :last_name AND first_name LIKE :first_name AND phone LIKE :phone",
  {user_id: user_id, last_name: row["Family Name"], first_name: row["Given Name"], phone: row["Phone 1 - Value"]})

				if patient_exists.count == 0


					patient = Patient.new
					patient.user_id = user_id
					patient.first_name = CGI::escapeHTML(row["Given Name"])
					patient.last_name = CGI::escapeHTML(row["Family Name"])
					patient.birthdate = CGI::escapeHTML(row["Birthday"])
					patient.profession = CGI::escapeHTML(row["Occupation"])
					patient.email = CGI::escapeHTML(row["E-mail 1 - Value"])
					patient.phone = CGI::escapeHTML(row["Phone 1 - Value"])
					patient.mobile_phone = CGI::escapeHTML(row["Phone 2 - Value"])
					patient.address = CGI::escapeHTML(row["Address 1 - Street"])
					patient.postal_code = CGI::escapeHTML(row["Address 1 - Postal Code"])
					patient.city = CGI::escapeHTML(row["Address 1 - City"])
					#patient.country_id = CGI::escapeHTML(row["Address 1 - Country"])
					patient.note = CGI::escapeHTML(row["Notes"])


					if patient.save(validate: false)
						imported_patients = imported_patients + 1
					end


				end
			end

		end

		return imported_patients

  	end

		def set_first_name_and_last_name_uppercase

			self.first_name = self.first_name.gsub(/\b(\w)/){|m| m.capitalize}
			self.last_name = self.last_name.gsub(/\b(\w)/){|m| m.capitalize}
			self.address = self.address.gsub(/\b(\w)/){|m| m.capitalize}
			self.city = self.city.gsub(/\b(\w)/){|m| m.capitalize}

		end

end
