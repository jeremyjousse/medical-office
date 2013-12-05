json.array!(@practitioners) do |practitioner|
  json.extract! practitioner, :first_name, :last_name, :phone, :mobile_phone, :email, :address, :postal_code, :city, :country_id, :note, :speciality_id
  json.url practitioner_url(practitioner, format: :json)
end
