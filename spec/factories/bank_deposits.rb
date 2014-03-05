# Read about factories at https://github.com/thoughtbot/factory_girl

FactoryGirl.define do
  factory :bank_deposit do
    type 1
    number 1
    amount "9.99"
  end
end
