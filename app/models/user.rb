class User < ActiveRecord::Base
  # Include default devise modules. Others available are:
  # :confirmable, :lockable, :timeoutable and :omniauthable
  devise :database_authenticatable, :registerable,
         :recoverable, :rememberable, :trackable, :validatable

  belongs_to :office
  has_many :patients
  has_many :practitioners
  has_many :medical_treatments
  has_many :medical_treatment_types
  has_many :payments
  has_many :payment_bank_checks
  has_many :bank_deposits

end