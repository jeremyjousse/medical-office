class CreatePaymentBankChecks < ActiveRecord::Migration
  def change
    create_table :payment_bank_checks do |t|
      t.string :account_owner
      t.string :bank_name
      t.string :check_number
      t.decimal :amount
      t.integer :status
      t.integer :check_deposit_id
      t.integer :paiment_id

      t.timestamps
    end
  end
end
