json.array!(@payment_bank_checks) do |payment_bank_check|
  json.extract! payment_bank_check, :id, :account_owner, :bank_name, :check_number, :amount, :status, :check_deposit_id, :paiment_id
  json.url payment_bank_check_url(payment_bank_check, format: :json)
end
