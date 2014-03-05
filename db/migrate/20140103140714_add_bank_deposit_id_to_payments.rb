class AddBankDepositIdToPayments < ActiveRecord::Migration
  def change
  	add_column :bank_deposits, :user_id, :integer
  	add_column :payments, :bank_deposit_id, :integer
  end
end
