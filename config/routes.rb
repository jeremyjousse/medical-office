MedicalOffice::Application.routes.draw do

  #devise_for :models

  root :to => 'passthrough#index'

#  devise_for :users

  #root to: redirect("/#{I18n.default_locale}")

  scope "/:locale" do
    # match '/', :to=>'home#index', :as=>:root

    #get '/:locale' => 'dashboard#index'
    match '/', :to=>'home#index', via: :get, :as=>:index
    
      devise_for :users
      resources :offices

      resources :patients

      resources :practitioners

      resources :medical_treatment_types
      resources :medical_treatments
    # namespace :admin do

    #   resources :users

    #   #get "home" => "statistics#home", :as=> :home






    #   #root :to => ':locale/statistics#home'
    # end

  end
end
