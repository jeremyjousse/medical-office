class AddBankAccountIdToBankDeposits < ActiveRecord::Migration
  def change
  	add_column :bank_deposits, :bank_account_id, :integer
  end
end
