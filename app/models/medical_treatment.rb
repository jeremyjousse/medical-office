class MedicalTreatment < ActiveRecord::Base

	belongs_to :user
	belongs_to :patient
	belongs_to :medical_treatment_type
	has_many :payments, :dependent => :destroy

	accepts_nested_attributes_for :payments

	validates :patient_id, :presence => true
	validates :date, :presence => true
	validates :location_id, :presence => true
	validates :medical_treatment_type_id, :presence => true
	validates :price, :presence => true
	validate :chek_total_payments_do_not_exceed_price


  LOCATIONS = {1 => "Office", 2 => "Home"}
  STATUS = {0 => "unpaid" ,1 => "paied"}

  before_save :change_status_if_paied

  private

  def chek_total_payments_do_not_exceed_price
  	total_payment = BigDecimal.new("0")
  	self.payments.each do |payment|
  			total_payment = total_payment + payment.amount
  	end

  	if !self.price.nil? && total_payment > self.price
  		errors.add(:price, "Max amount exceeded for payments")
  		return false
  	end
  	return true
  end

  def change_status_if_paied
	    total_payment = BigDecimal.new("0")
	    self.payments.each do |payment|
	        total_payment = total_payment + payment.amount
	    end

	    if !self.price.nil? && total_payment == self.price
	      self.status = 1
	      #self.save
	    else
	      self.status = 0
	      #self.save
	    end
  end

end
