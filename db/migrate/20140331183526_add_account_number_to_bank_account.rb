class AddAccountNumberToBankAccount < ActiveRecord::Migration
  def change
  	add_column :bank_accounts, :account_number, :integer
  end
end
