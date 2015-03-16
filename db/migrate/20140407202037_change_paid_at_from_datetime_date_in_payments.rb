class ChangePaidAtFromDatetimeDateInPayments < ActiveRecord::Migration
  def up
  	change_column :payments, :paid_at, :date
  end
  def down
  	change_column :payments, :paid_at, :datetime
  end
end
