class AddDepositDateToBankDeposits < ActiveRecord::Migration
  def change
  	add_column :bank_deposits, :deposit_date, :date
  end
end