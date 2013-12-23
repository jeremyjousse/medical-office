MedicalOffice::Application.routes.draw do

  root :to => 'passthrough#index'

  scope "/:locale" do
    match '/', :to=>'home#index', via: :get, :as=>:index
    
      devise_for :users
      resources :offices, only: [:new, :create, :show, :edit, :update]

      resources :patients do
        get 'finder', on: :collection
      end

      resources :practitioners do
        get 'finder', on: :collection
      end

      resources :medical_treatment_types
      resources :medical_treatments
      resources :payments
  end
end
