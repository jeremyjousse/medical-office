MedicalOffice::Application.routes.draw do

  resources :payment_bank_checks

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

      resources :medical_treatment_types do
        member do
          get 'price_finder'
        end
      end
      resources :medical_treatments do
        get "new_payment" => 'medical_treatment_type#new_payment', :as => :new_payment
      end
      
      resources :payments

      resources :payment_bank_checks

  end
end
