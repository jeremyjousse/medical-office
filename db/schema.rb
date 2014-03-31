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

ActiveRecord::Schema.define(version: 20140331183526) do

  create_table "bank_accounts", force: true do |t|
    t.string   "bank_name"
    t.integer  "bank_check_deposit_number"
    t.integer  "user_id"
    t.datetime "created_at"
    t.datetime "updated_at"
    t.integer  "account_number"
  end

  create_table "bank_deposits", force: true do |t|
    t.integer  "deposit_type"
    t.integer  "number"
    t.decimal  "amount"
    t.datetime "created_at"
    t.datetime "updated_at"
    t.integer  "user_id"
    t.date     "deposit_date"
    t.integer  "bank_account_id"
  end

  create_table "countries", force: true do |t|
    t.string "name"
    t.string "iso"
  end

  create_table "medical_treatment_types", force: true do |t|
    t.string   "name"
    t.decimal  "price"
    t.integer  "user_id"
    t.datetime "created_at"
    t.datetime "updated_at"
  end

  create_table "medical_treatments", force: true do |t|
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

  create_table "models", force: true do |t|
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

  create_table "offices", force: true do |t|
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

  create_table "patients", force: true do |t|
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

  create_table "payment_bank_checks", force: true do |t|
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

  create_table "payments", force: true do |t|
    t.integer  "medical_treatment_id"
    t.datetime "paid_at"
    t.integer  "payment_type"
    t.decimal  "amount"
    t.datetime "created_at"
    t.datetime "updated_at"
    t.integer  "user_id"
    t.integer  "bank_deposit_id"
  end

  add_index "payments", ["medical_treatment_id"], name: "index_payments_on_medical_treatment_id"

  create_table "practitioners", force: true do |t|
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

  create_table "users", force: true do |t|
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
