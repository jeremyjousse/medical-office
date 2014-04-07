class PaymentBankCheck < ActiveRecord::Base
	has_one :payment
	belongs_to :user
	belongs_to :bank_deposit

	validates :account_owner, :presence => true
	validates :bank_name, :presence => true
	validates :check_number, :presence => true

	STATUS = {-1 => "Rejected", 0 => "Uncashed", 1 => "Cashed"}

	scope :unchashed, -> { where(status: 0) }
	scope :pending, -> { PaymentBankCheck.joins('JOIN payments ON payments.id = payment_bank_checks.payment_id').where(status: 0).where("payments.paid_at < ?", Time.now).select("payment_bank_checks.*, payments.paid_at") }

end
