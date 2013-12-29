class PaymentBankCheck < ActiveRecord::Base
	has_one :payment
	belongs_to :user
end
