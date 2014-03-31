class RenameCheckDepositIdToBankDepositIdForPaymentBankChecks < ActiveRecord::Migration
  def change
  	rename_column :payment_bank_checks, :check_deposit_id, :bank_deposit_id
  end
end
