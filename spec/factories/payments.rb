# Read about factories at https://github.com/thoughtbot/factory_girl

FactoryGirl.define do
  factory :payment do
    medical_treatment nil
    paid_at "2013-12-16 12:57:31"
    type 1
    amount ""
  end
end
