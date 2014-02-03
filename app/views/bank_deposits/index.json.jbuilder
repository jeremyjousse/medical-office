json.array!(@bank_deposits) do |bank_deposit|
  json.extract! bank_deposit, :id, :type, :number, :amount
  json.url bank_deposit_url(bank_deposit, format: :json)
end
