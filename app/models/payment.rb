class Payment < ActiveRecord::Base
  belongs_to :medical_treatment
  belongs_to :user
  belongs_to :bank_deposit
  has_one :payment_bank_check, :dependent => :destroy

  accepts_nested_attributes_for :payment_bank_check

  #validates :medical_treatment_id, :presence => true
  validates :paid_at, :presence => true
  validates :payment_type, :presence => true
  validates :amount, :presence => true

	#validate :chek_total_payments_do_not_exceed_medical_treatment_amount

  after_save :change_medical_treatment_id_total_payments_amount_equal_medical_treatment_price



  TYPE = {1 => "Check", 2 => "Credit card", 3 => "Cash"}




  def chek_total_payments_do_not_exceed_medical_treatment_amount
  	mt = self.medical_treatment
  	total_payment = BigDecimal.new("0")
  	total_payment = self.amount
  	mt.payments.each do |payment|
  		if payment.id != self.id
  			total_payment = total_payment + payment.amount
  		end
  	end

  	if total_payment > self.medical_treatment.price
  		errors.add(:amount, "Max amount exceeded")
  		return false
  	end
  	return true
  end

  def change_medical_treatment_id_total_payments_amount_equal_medical_treatment_price

    Rails.logger.info '--------- payment.destroy'

    mt = self.medical_treatment
    total_payment = BigDecimal.new("0")
    mt.payments.each do |payment|
        total_payment = total_payment + payment.amount
    end

    if total_payment == self.medical_treatment.price
      mt.status = 1
      mt.save
    end
  end


  def safe_destroy

    if !self.bank_deposit_id.nil?

      return false

    end


    medical_treatment = self.medical_treatment

    self.destroy

    medical_treatment.save

    # check bank_deposit_id

    # check medical_treatment





    #

  end

end
