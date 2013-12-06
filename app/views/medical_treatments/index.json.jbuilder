json.array!(@medical_treatments) do |medical_treatment|
  json.extract! medical_treatment, :user_id, :patient_id, :date, :location_id, :treatment_type_id, :price, :note
  json.url medical_treatment_url(medical_treatment, format: :json)
end
