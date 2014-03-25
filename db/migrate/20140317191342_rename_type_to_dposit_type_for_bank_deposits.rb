class RenameTypeToDpositTypeForBankDeposits < ActiveRecord::Migration
  def change
  	rename_column :bank_deposits, :type, :deposit_type
  end
end
