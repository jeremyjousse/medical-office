# Read about factories at https://github.com/thoughtbot/factory_girl

FactoryGirl.define do
  factory :medical_treatment_type do
    name "MyString"
    price "9.99"
    user_id 1
  end
end
