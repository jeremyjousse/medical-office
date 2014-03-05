# Read about factories at https://github.com/thoughtbot/factory_girl

FactoryGirl.define do
  factory :payment_bank_check do
    account_owner "MyString"
    bank_name "MyString"
    check_number "MyString"
    amount "9.99"
    status 1
    check_deposit_id 1
    paiment_id 1
  end
end
