json.array!(@payments) do |payment|
  json.extract! payment, :id, :medical_treatment_id, :paid_at, :type, :amount
  json.url payment_url(payment, format: :json)
end
