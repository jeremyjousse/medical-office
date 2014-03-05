class AddUserIdToPaymentBankChecks < ActiveRecord::Migration
  def change
  	add_column :payment_bank_checks, :user_id, :integer
  end
end
