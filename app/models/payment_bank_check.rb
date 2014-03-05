class PaymentBankCheck < ActiveRecord::Base
	has_one :payment
	belongs_to :user

	validates :account_owner, :presence => true
	validates :bank_name, :presence => true
	validates :check_number, :presence => true

	STATUS = {-1 => "Rejected", 0 => "Uncashed", 1 => "Cashed"}
end
