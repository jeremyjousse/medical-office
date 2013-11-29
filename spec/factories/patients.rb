FactoryGirl.define do
  factory :patient do
    first_name 'Sidney'
    last_name 'Johns'
    created_at Time.now
    updated_at Time.now
  end
end
