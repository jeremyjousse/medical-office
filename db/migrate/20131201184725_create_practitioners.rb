class CreatePractitioners < ActiveRecord::Migration
  def change
    create_table :practitioners do |t|
      t.string :first_name
      t.string :last_name
      t.string :phone
      t.string :mobile_phone
      t.string :email
      t.string :address
      t.string :postal_code
      t.string :city
      t.integer :country_id
      t.text :note
      t.integer :speciality_id

      t.timestamps
    end
  end
end
