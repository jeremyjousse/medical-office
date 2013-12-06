json.array!(@medical_treatment_types) do |medical_treatment_type|
  json.extract! medical_treatment_type, :name, :price, :user_id
  json.url medical_treatment_type_url(medical_treatment_type, format: :json)
end
