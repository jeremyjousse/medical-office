class BankAccount < ActiveRecord::Base
	belongs_to :user
	has_many :bank_deposits
end
