class CreateBankAccounts < ActiveRecord::Migration
  def change
    create_table :bank_accounts do |t|
      t.string :bank_name
      t.integer :bank_check_deposit_number
      t.integer :user_id

      t.timestamps
    end
  end
end
