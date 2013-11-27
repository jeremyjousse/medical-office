class Office < ActiveRecord::Base
	belongs_to :country
	has_many :users

	validates :name, :presence => true, :length => 2..255, :on => :update
	validates :address, :presence => true, :length => 2..255, :on => :update
  validates :postal_code, :presence => true, :length => 2..100, :on => :update
  validates :city, :presence => true, :length => 2..255, :on => :update
  validates :country_id, :presence => true, :on => :update
  validates :phone, :presence => true, :length => 5..100, :on => :update
  validates :email, :presence => true, :format => { :with => /\A([^@\s]+)@((?:[-a-z0-9]+\.)+[a-z]{2,})\z/i } , :length => 5..255, :on => :update
  validates :website, :length => { :maximum => 255 }, :on => :update
  validates :website, :format => { :with => URI::regexp(%w(http https))}, :allow_blank => true, :on => :update
  
end