# Read about factories at https://github.com/thoughtbot/factory_girl

FactoryGirl.define do
  factory :practitioner do
    first_name "MyString"
    last_name "MyString"
    phone "MyString"
    mobile_phone "MyString"
    email "MyString"
    address "MyString"
    postal_code "MyString"
    city "MyString"
    country_id 1
    note "MyText"
    speciality_id 1
  end
end
