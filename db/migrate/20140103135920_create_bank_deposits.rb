class CreateBankDeposits < ActiveRecord::Migration
  def change
    create_table :bank_deposits do |t|
      t.integer :type
      t.integer :number
      t.decimal :amount

      t.timestamps
    end
  end
end
