# encoding: UTF-8
# This file is auto-generated from the current state of the database. Instead
# of editing this file, please use the migrations feature of Active Record to
# incrementally modify your database, and then regenerate this schema definition.
#
# Note that this schema.rb definition is the authoritative source for your
# database schema. If you need to create the application database on another
# system, you should be using db:schema:load, not running all the migrations
# from scratch. The latter is a flawed and unsustainable approach (the more migrations
# you'll amass, the slower it'll run and the greater likelihood for issues).
#
# It's strongly recommended that you check this file into your version control system.

ActiveRecord::Schema.define(version: 20150130165215) do

  create_table "bank_accounts", force: :cascade do |t|
    t.string   "bank_name"
    t.integer  "bank_check_deposit_number"
    t.integer  "user_id"
    t.datetime "created_at"
    t.datetime "updated_at"
    t.integer  "account_number"
  end

  create_table "bank_deposits", force: :cascade do |t|
    t.integer  "deposit_type"
    t.integer  "number"
    t.decimal  "amount"
    t.datetime "created_at"
    t.datetime "updated_at"
    t.integer  "user_id"
    t.date     "deposit_date"
    t.integer  "bank_account_id"
  end

  create_table "countries", force: :cascade do |t|
    t.string "name"
    t.string "iso"
  end

  create_table "document_scans", force: :cascade do |t|
    t.integer  "patient_id"
    t.integer  "user_id"
    t.string   "file"
    t.datetime "created_at"
    t.datetime "updated_at"
    t.string   "title"
  end

  add_index "document_scans", ["patient_id"], name: "index_document_scans_on_patient_id"
  add_index "document_scans", ["user_id"], name: "index_document_scans_on_user_id"

  create_table "medical_treatment_types", force: :cascade do |t|
    t.string   "name"
    t.decimal  "price"
    t.integer  "user_id"
    t.datetime "created_at"
    t.datetime "updated_at"
  end

  create_table "medical_treatments", force: :cascade do |t|
    t.integer  "user_id"
    t.integer  "patient_id"
    t.date     "date"
    t.integer  "location_id"
    t.integer  "treatment_type_id"
    t.decimal  "price"
    t.text     "note"
    t.datetime "created_at"
    t.datetime "updated_at"
    t.integer  "medical_treatment_type_id"
    t.integer  "status",                    default: 0
  end

  create_table "models", force: :cascade do |t|
    t.string   "email",                  default: "", null: false
    t.string   "encrypted_password",     default: "", null: false
    t.string   "reset_password_token"
    t.datetime "reset_password_sent_at"
    t.datetime "remember_created_at"
    t.integer  "sign_in_count",          default: 0,  null: false
    t.datetime "current_sign_in_at"
    t.datetime "last_sign_in_at"
    t.string   "current_sign_in_ip"
    t.string   "last_sign_in_ip"
    t.datetime "created_at"
    t.datetime "updated_at"
  end

  add_index "models", ["email"], name: "index_models_on_email", unique: true
  add_index "models", ["reset_password_token"], name: "index_models_on_reset_password_token", unique: true

  create_table "offices", force: :cascade do |t|
    t.string   "name"
    t.string   "address"
    t.string   "city"
    t.string   "postal_code"
    t.integer  "country_id"
    t.string   "phone"
    t.string   "website"
    t.string   "email"
    t.datetime "created_at"
    t.datetime "updated_at"
  end

  create_table "patients", force: :cascade do |t|
    t.string   "first_name"
    t.string   "last_name"
    t.date     "birthdate"
    t.string   "profession"
    t.string   "phone"
    t.string   "mobile_phone"
    t.string   "email"
    t.string   "address"
    t.string   "postal_code"
    t.string   "city"
    t.integer  "country_id"
    t.string   "national_insurance_number"
    t.integer  "family_doctor_id"
    t.integer  "physical_therapists_id"
    t.integer  "osteopath_id"
    t.integer  "legal_guardian_id"
    t.text     "note"
    t.boolean  "diabetic"
    t.boolean  "universal_healthcare_coverage"
    t.boolean  "long_duration_disease"
    t.integer  "diabete_network_id"
    t.integer  "user_id"
    t.datetime "created_at"
    t.datetime "updated_at"
  end

  create_table "payment_bank_checks", force: :cascade do |t|
    t.string   "account_owner"
    t.string   "bank_name"
    t.string   "check_number"
    t.decimal  "amount"
    t.integer  "status"
    t.integer  "bank_deposit_id"
    t.integer  "payment_id"
    t.datetime "created_at"
    t.datetime "updated_at"
    t.integer  "user_id"
  end

  create_table "payments", force: :cascade do |t|
    t.integer  "medical_treatment_id"
    t.date     "paid_at"
    t.integer  "payment_type"
    t.decimal  "amount"
    t.datetime "created_at"
    t.datetime "updated_at"
    t.integer  "user_id"
    t.integer  "bank_deposit_id"
  end

  add_index "payments", ["medical_treatment_id"], name: "index_payments_on_medical_treatment_id"

  create_table "postural_analyses", force: :cascade do |t|
    t.integer  "user_id"
    t.integer  "patient_id"
    t.integer  "shoe_size"
    t.datetime "created_at"
    t.datetime "updated_at"
    t.string   "sole_pattern"
    t.text     "pain"
    t.text     "medical_history"
    t.text     "radiologic_examinations"
    t.string   "headache"
    t.string   "giddiness"
    t.string   "falls"
    t.string   "lying_down_eo_mo"
    t.string   "lying_down_ec_mo"
    t.string   "lying_down_eo_mc"
    t.string   "jaw_centring"
    t.string   "tooth_fillings"
    t.string   "popping_jaw"
    t.string   "jaw_opening"
    t.string   "temporal_muscles"
    t.string   "micro_current"
    t.string   "podoscope"
    t.string   "stork_balance_test"
    t.integer  "left_right_handed"
    t.string   "barre_vertical_test"
    t.string   "ilium"
    t.string   "scapulum"
    t.string   "plumb_line"
    t.text     "walking"
    t.string   "cervical_rotation"
    t.string   "thumb_test"
    t.string   "dynamic_posture_test"
    t.string   "romberg_test"
    t.string   "trampling"
    t.string   "extensors_test"
    t.string   "eye_convergence"
    t.string   "master_eye"
    t.string   "maddox_test"
    t.string   "visual_correction"
    t.string   "atlas"
    t.string   "jammed"
  end

  create_table "postural_analysis_scans", force: :cascade do |t|
    t.integer "postural_analysis_id"
    t.string  "scan"
    t.integer "user_id"
  end

  add_index "postural_analysis_scans", ["postural_analysis_id"], name: "index_postural_analysis_scans_on_postural_analysis_id"
  add_index "postural_analysis_scans", ["user_id"], name: "index_postural_analysis_scans_on_user_id"

  create_table "practitioners", force: :cascade do |t|
    t.string   "first_name"
    t.string   "last_name"
    t.string   "phone"
    t.string   "mobile_phone"
    t.string   "email"
    t.string   "address"
    t.string   "postal_code"
    t.string   "city"
    t.integer  "country_id"
    t.text     "note"
    t.integer  "speciality_id"
    t.datetime "created_at"
    t.datetime "updated_at"
    t.integer  "user_id"
  end

  create_table "users", force: :cascade do |t|
    t.string   "email",                  default: "", null: false
    t.string   "encrypted_password",     default: "", null: false
    t.string   "reset_password_token"
    t.datetime "reset_password_sent_at"
    t.datetime "remember_created_at"
    t.integer  "sign_in_count",          default: 0,  null: false
    t.datetime "current_sign_in_at"
    t.datetime "last_sign_in_at"
    t.string   "current_sign_in_ip"
    t.string   "last_sign_in_ip"
    t.datetime "created_at"
    t.datetime "updated_at"
    t.integer  "office_id"
  end

  add_index "users", ["email"], name: "index_users_on_email", unique: true
  add_index "users", ["reset_password_token"], name: "index_users_on_reset_password_token", unique: true

end
