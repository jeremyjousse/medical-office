FactoryGirl.define do
  factory :patient do
    first_name 'Sidney'
    last_name 'Johns'
    city 'Lille'
    country_id 1
    created_at Time.now
    updated_at Time.now
  end
end
