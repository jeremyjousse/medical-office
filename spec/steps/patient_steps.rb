require 'spec_helper'
include Devise::TestHelpers
steps_for :patient do
  def build_valid_patient
    @patient = FactoryGirl.build(:patient)
  end

  step :build_valid_patient, 'build a valid patient'

  step 'I add a patient with :validness informations' do |validness|
    build_valid_patient if validness == 'valid'

    visit '/en/patients/new'
    fill_in 'patient_first_name', with: @patient.first_name
    fill_in 'patient_last_name', with: @patient.last_name
    fill_in 'patient_city', with: @patient.city
    # fill_in 'patient_country_id', with: @patient.country_id

    click_button 'Create Patient'
  end

  step 'I should see patient show page' do
    page.should have_content 'Patient detail'
  end
end
