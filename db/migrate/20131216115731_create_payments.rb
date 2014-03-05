class CreatePayments < ActiveRecord::Migration
  def change
    create_table :payments do |t|
      t.references :medical_treatment, index: true
      t.datetime :paid_at
      t.integer :type
      t.decimal :amount

      t.timestamps
    end
  end
end
