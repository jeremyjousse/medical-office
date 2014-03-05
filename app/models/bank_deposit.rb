class BankDeposit < ActiveRecord::Base

  has_many :payments
  has_one :user

  TYPES = {1 => "Check", 2 => "Cash"}

end
