# Read about factories at https://github.com/thoughtbot/factory_girl

FactoryGirl.define do
  factory :bank_account do
    bank_name "MyString"
    bank_check_deposit_number 1
    user_id 1
  end
end
