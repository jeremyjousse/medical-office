class BankDeposit < ActiveRecord::Base

  has_many :payments
  has_many :payment_bank_checks
  belongs_to :user
  belongs_to :bank_account

  DEPOSIT_TYPES = {1 => "Check", 2 => "Cash"}

end
