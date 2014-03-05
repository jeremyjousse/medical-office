class CreateOffices < ActiveRecord::Migration
  def change
    create_table :offices do |t|
      t.string :name
      t.string :address
      t.string :city
      t.string :postal_code
      t.integer :country_id
      t.string :phone
      t.string :website
      t.string :email

      t.timestamps
    end
  end
end
