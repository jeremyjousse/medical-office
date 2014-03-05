include Devise::TestHelpers

step "I am signed in as :email :password" do |email, password|
  visit '/en/users/sign_in'
  fill_in "user_email", :with => email
  fill_in "user_password", :with => password
  click_button "Sign in"
end