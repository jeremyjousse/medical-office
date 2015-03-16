MedicalOffice::Application.routes.draw do
  root :to => 'passthrough#index'

  scope "/:locale" do
    match '/', :to=>'home#index', via: :get, :as=>:index

      devise_for :users
      resources :offices, only: [:new, :create, :show, :edit, :update]

      resources :patients do
        get 'finder', on: :collection
        get 'new_import', to: 'patients#new_import', as: 'new_import', on: :collection
        collection { post :import }
        resources :medical_treatments, controller: 'patients/medical_treatments' do
          get 'receipt'
          resources :payments, controller: 'patients/medical_treatments/payments'
        end
        resources :postural_analyses,  controller: 'patients/postural_analyses' do
        end
        resources :document_scans,  controller: 'patients/document_scans' do
        end
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

      resources :postural_analyses do
        resources :postural_analysis_scans
      end

      resources :payments do
        get 'listing', on: :collection
      end

      resources :payment_bank_checks

  end
end
