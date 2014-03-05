class RenamePaimentIdToPaymentId < ActiveRecord::Migration
  def change
  	rename_column :payment_bank_checks, :paiment_id, :payment_id
  end
end
