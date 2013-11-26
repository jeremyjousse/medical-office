class Office < ActiveRecord::Base
	belongs_to :country
	has_many :users
end