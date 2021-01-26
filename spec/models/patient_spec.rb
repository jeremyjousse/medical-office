require 'rails_helper'

describe Patient do
  it 'has a valid factory' do
    patient = FactoryGirl.build(:patient)
    patient.should be_valid
  end
end
