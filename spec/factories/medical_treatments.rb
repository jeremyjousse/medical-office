# Read about factories at https://github.com/thoughtbot/factory_girl

FactoryGirl.define do
  factory :medical_treatment do
    user_id 1
    patient_id 1
    date "2013-12-05"
    location_id 1
    treatment_type_id 1
    price "9.99"
    note "MyText"
  end
end
