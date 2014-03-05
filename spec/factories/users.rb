# Read about factories at https://github.com/thoughtbot/factory_girl

FactoryGirl.define do
  factory :user do
    email 'jean.dupond@test.com'
    password '1234qerty'
    password_confirmation '1234qerty'
    created_at Time.now
    updated_at Time.now
    #confirmed_at Time.now
  end

  factory :valid_user, class: User do
    email 'jean.dupond@test.com'
    password '1234qerty'
    password_confirmation '1234qerty'
    created_at Time.now
    updated_at Time.now
    #confirmed_at Time.now
  end
end
