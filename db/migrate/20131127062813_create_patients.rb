class CreatePatients < ActiveRecord::Migration
  def change
    create_table :patients do |t|
      t.string :first_name
      t.string :last_name
      t.date :birthdate
      t.string :profession
      t.string :phone
      t.string :mobile_phone
      t.string :email
      t.string :address
      t.string :postal_code
      t.string :city
      t.integer :country_id
      t.string :national_insurance_number
      t.integer :family_doctor_id
      t.integer :physical_therapists_id
      t.integer :osteopath_id
      t.integer :legal_guardian_id
      t.text :note
      t.boolean :diabetic
      t.boolean :universal_healthcare_coverage
      t.boolean :long_duration_disease
      t.integer :diabete_network_id
    end
  end
end
