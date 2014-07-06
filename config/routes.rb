MedicalOffice::Application.routes.draw do

  root :to => 'passthrough#index'

  scope "/:locale" do
    match '/', :to=>'home#index', via: :get, :as=>:index

      devise_for :users
      resources :offices, only: [:new, :create, :show, :edit, :update]

      resources :patients do
        get 'finder', on: :collection
        get 'new_import', to: 'patients#new_import', as: 'new_import', on: :collection
        # post 'import', to: 'patients#import', as: 'import', on: :collection
        collection { post :import }
      end


      resources :practitioners do
        get 'finder', on: :collection
      end

      resources :payment_bank_checks do
        get 'pending', on: :collection
      end

      resources :medical_treatment_types do
        member do
          get 'price_finder'
        end

      end
      resources :medical_treatments do
        get "new_payment" => 'medical_treatment_type#new_payment', :as => :new_payment

      end

      resources :bank_accounts

      resources :bank_deposits

      resources :postural_analyses

      resources :payments do
        get 'listing', on: :collection
      end

      resources :payment_bank_checks

  end
end
