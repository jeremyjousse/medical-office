import { invoke } from '@tauri-apps/api/core';
import { open, save } from '@tauri-apps/plugin-dialog';
import { translateErrorCode, type Locale } from './i18n';

export interface User {
  id: number;
  email: string;
  office_id: number | null;
  created_at: string | null;
  updated_at: string | null;
}

export interface Office {
  id: number;
  name: string | null;
  address: string | null;
  city: string | null;
  postal_code: string | null;
  country_id: number | null;
  phone: string | null;
  website: string | null;
  email: string | null;
  created_at: string | null;
  updated_at: string | null;
}

export interface DbStatus {
  initialized: boolean;
  db_path: string | null;
  active_user: User | null;
  office: Office | null;
  users_count: number;
  needs_office_setup: boolean;
}

export interface Country {
  id: number;
  name: string | null;
  iso: string | null;
}

export interface HomeSummary {
  unpaid_medical_treatments_count: number;
  office: Office;
  needs_office_setup: boolean;
}

export interface OfficeInput {
  name: string;
  address: string;
  postal_code: string;
  city: string;
  country_id: number | null;
  phone: string;
  website: string | null;
  email: string;
}

export interface PractitionerView {
  id: number;
  first_name: string | null;
  last_name: string | null;
  phone: string | null;
  mobile_phone: string | null;
  email: string | null;
  address: string | null;
  postal_code: string | null;
  city: string | null;
  country_id: number | null;
  country_name: string | null;
  note: string | null;
  speciality_id: number | null;
  created_at: string | null;
  updated_at: string | null;
  user_id: number | null;
}

export interface PractitionerFilter {
  page?: number;
  per_page?: number;
  speciality_id?: number | null;
  first_name?: string;
  last_name?: string;
  city?: string;
}

export interface PractitionerInput {
  speciality_id: number | null;
  first_name: string;
  last_name: string;
  phone: string | null;
  mobile_phone: string | null;
  email: string | null;
  address: string;
  postal_code: string;
  city: string;
  country_id: number | null;
  note: string | null;
}

export interface MedicalTreatmentType {
  id: number;
  name: string | null;
  price: number | null;
  user_id: number | null;
  created_at: string | null;
  updated_at: string | null;
}

export interface TreatmentTypeFilter {
  page?: number;
  per_page?: number;
  name?: string;
}

export interface TreatmentTypeInput {
  name: string;
  price: number | null;
}

export interface BankAccount {
  id: number;
  bank_name: string | null;
  bank_check_deposit_number: number | null;
  account_number: number | null;
  user_id: number | null;
  created_at: string | null;
  updated_at: string | null;
}

export interface BankAccountFilter {
  page?: number;
  per_page?: number;
  bank_name?: string;
}

export interface BankAccountInput {
  bank_name: string;
  account_number: number | null;
  bank_check_deposit_number: number | null;
}

export interface PatientView {
  id: number;
  first_name: string | null;
  last_name: string | null;
  birthdate: string | null;
  profession: string | null;
  phone: string | null;
  mobile_phone: string | null;
  email: string | null;
  address: string | null;
  postal_code: string | null;
  city: string | null;
  country_id: number | null;
  country_name: string | null;
  national_insurance_number: string | null;
  family_doctor_id: number | null;
  family_doctor_label: string | null;
  physical_therapists_id: number | null;
  physical_therapists_label: string | null;
  osteopath_id: number | null;
  osteopath_label: string | null;
  legal_guardian_id: number | null;
  note: string | null;
  diabetic: boolean | null;
  universal_healthcare_coverage: boolean | null;
  long_duration_disease: boolean | null;
  diabete_network_id: number | null;
  user_id: number | null;
  created_at: string | null;
  updated_at: string | null;
  treatments_count: number;
}

export interface PatientChoice {
  id: number;
  first_name: string | null;
  last_name: string | null;
  city: string | null;
  label: string;
}

export interface PractitionerOption {
  id: number;
  first_name: string | null;
  last_name: string | null;
  city: string | null;
  speciality_id: number | null;
}

export interface PatientFilter {
  page?: number;
  per_page?: number;
  first_name?: string;
  last_name?: string;
  phone?: string;
  diabetic?: boolean;
}

export interface PatientInput {
  first_name: string | null;
  last_name: string;
  birthdate: string | null;
  profession: string | null;
  phone: string | null;
  mobile_phone: string | null;
  email: string | null;
  address: string | null;
  postal_code: string | null;
  city: string;
  country_id: number | null;
  national_insurance_number: string | null;
  family_doctor_id: number | null;
  physical_therapists_id: number | null;
  osteopath_id: number | null;
  legal_guardian_id: number | null;
  note: string | null;
  diabetic: boolean;
  universal_healthcare_coverage: boolean;
  long_duration_disease: boolean;
  diabete_network_id: number | null;
}

export interface ImportPatientsResult {
  imported_count: number;
  skipped_count: number;
  total_rows: number;
}

export interface Payment {
  id: number;
  medical_treatment_id: number | null;
  paid_at: string | null;
  payment_type: number | null;
  amount: number | null;
  user_id: number | null;
  bank_deposit_id: number | null;
  created_at: string | null;
  updated_at: string | null;
}

export interface PaymentBankCheck {
  id: number;
  account_owner: string | null;
  bank_name: string | null;
  check_number: string | null;
  amount: number | null;
  status: number | null;
  bank_deposit_id: number | null;
  payment_id: number | null;
  user_id: number | null;
  created_at: string | null;
  updated_at: string | null;
}

export interface PaymentView extends Payment {
  patient_id: number | null;
  patient_first_name: string | null;
  patient_last_name: string | null;
  treatment_type_name: string | null;
  treatment_date: string | null;
  treatment_price: number | null;
  payment_bank_check: PaymentBankCheck | null;
}

export interface PaymentFilter {
  page?: number;
  per_page?: number;
  paid_at?: string;
  payment_type?: number | null;
}

export interface PaymentListingFilter {
  paid_at_gteq?: string;
  paid_at_lteq?: string;
  payment_type?: number | null;
}

export interface PaymentTypeTotal {
  payment_type: number;
  amount: number;
}

export interface PaymentListingDayGroup {
  date: string;
  payments: PaymentView[];
  total_amount: number;
  totals_by_type: PaymentTypeTotal[];
}

export interface PaymentListingResult {
  days: PaymentListingDayGroup[];
  grand_total_amount: number;
  grand_totals_by_type: PaymentTypeTotal[];
  filtered_total: number;
  total: number;
}

export interface PaymentBankCheckInput {
  account_owner: string;
  bank_name: string;
  check_number: string;
}

export interface PaymentInput {
  medical_treatment_id: number | null;
  paid_at: string;
  payment_type: number | null;
  amount: number | null;
  payment_bank_check: PaymentBankCheckInput | null;
}

export interface MedicalTreatmentOption {
  id: number;
  patient_id: number | null;
  patient_first_name: string | null;
  patient_last_name: string | null;
  treatment_type_name: string | null;
  date: string | null;
  price: number | null;
  status: number | null;
}

export interface BankDepositView {
  id: number;
  deposit_type: number | null;
  number: number | null;
  amount: number | null;
  user_id: number | null;
  deposit_date: string | null;
  bank_account_id: number | null;
  bank_account_name: string | null;
  bank_account_number: number | null;
  checks_count: number;
  created_at: string | null;
  updated_at: string | null;
}

export interface BankCheckDepositRow {
  id: number;
  account_owner: string | null;
  bank_name: string | null;
  check_number: string | null;
  amount: number | null;
  status: number | null;
  bank_deposit_id: number | null;
  payment_id: number | null;
  paid_at: string | null;
  patient_first_name: string | null;
  patient_last_name: string | null;
}

export interface BankDepositDetail {
  deposit: BankDepositView;
  office: Office;
  checks: BankCheckDepositRow[];
}

export interface BankDepositFilter {
  page?: number;
  per_page?: number;
  deposit_type?: number | null;
  number?: number | null;
}

export interface BankDepositInput {
  deposit_type: number | null;
  bank_account_id: number | null;
  payment_bank_check_ids: number[];
}

export interface MedicalTreatmentView {
  id: number;
  user_id: number | null;
  patient_id: number | null;
  patient_first_name: string | null;
  patient_last_name: string | null;
  patient_city: string | null;
  date: string | null;
  location_id: number | null;
  medical_treatment_type_id: number | null;
  treatment_type_name: string | null;
  treatment_type_price: number | null;
  price: number | null;
  note: string | null;
  status: number | null;
  payments_count: number;
  payments_total: number;
  created_at: string | null;
  updated_at: string | null;
}

export interface MedicalTreatmentDetail {
  treatment: MedicalTreatmentView;
  payments: Payment[];
}

export interface TreatmentFilter {
  page?: number;
  per_page?: number;
  patient_id?: number | null;
  medical_treatment_type_id?: number | null;
  date_gteq?: string;
  date_lteq?: string;
  location_id?: number | null;
  status?: number | null;
}

export interface MedicalTreatmentInput {
  patient_id: number | null;
  date: string;
  location_id: number | null;
  medical_treatment_type_id: number | null;
  price: number | null;
  note: string | null;
}

export interface PaginatedResult<T> {
  items: T[];
  page: number;
  per_page: number;
  filtered_total: number;
  total: number;
}

export interface FieldError {
  field: string;
  code: string;
  message: string;
}

export interface ParsedApiError {
  message: string;
  fieldErrors: Record<string, string>;
}

type RawAppError =
  | { kind: 'validation'; errors: FieldError[]; message: string }
  | { kind: 'blocked'; code: string; message: string }
  | { kind: 'not_found'; message: string }
  | { kind: 'message'; message: string };

export function parseApiError(err: unknown, locale: Locale): ParsedApiError {
  if (typeof err === 'string') {
    return { message: err, fieldErrors: {} };
  }
  if (err && typeof err === 'object' && 'kind' in err) {
    const appErr = err as RawAppError;
    if (appErr.kind === 'validation') {
      const fieldErrors: Record<string, string> = {};
      const messages: string[] = [];
      for (const f of appErr.errors) {
        const msg = translateErrorCode(locale, f.code, f.message);
        if (!fieldErrors[f.field]) {
          fieldErrors[f.field] = msg;
        }
        messages.push(`${f.field}: ${msg}`);
      }
      return {
        message: messages.join(' '),
        fieldErrors,
      };
    }
    if (appErr.kind === 'blocked') {
      return {
        message: translateErrorCode(locale, appErr.code, appErr.message),
        fieldErrors: {},
      };
    }
    return { message: appErr.message, fieldErrors: {} };
  }
  if (err instanceof Error) {
    return { message: err.message, fieldErrors: {} };
  }
  return { message: String(err), fieldErrors: {} };
}

// DB Setup & Profile
export const getDbStatus = () => invoke<DbStatus>('get_db_status');
export const importSqliteDb = (sourcePath: string) =>
  invoke<DbStatus>('import_sqlite_db', { sourcePath });
export const createEmptyDb = () => invoke<DbStatus>('create_empty_db');
export const exportSqliteDb = (destPath: string) =>
  invoke<void>('export_sqlite_db', { destPath });
export const listUsers = () => invoke<User[]>('list_users');
export const switchUser = (userId: number) => invoke<DbStatus>('switch_user', { userId });

// Dialog helpers
export async function pickSqliteFile(): Promise<string | null> {
  const selected = await open({
    multiple: false,
    filters: [{ name: 'SQLite Database', extensions: ['sqlite3', 'sqlite', 'db'] }],
  });
  return typeof selected === 'string' ? selected : null;
}

export async function pickSaveSqliteFile(): Promise<string | null> {
  const selected = await save({
    defaultPath: 'medical-office-backup.sqlite3',
    filters: [{ name: 'SQLite Database', extensions: ['sqlite3', 'sqlite', 'db'] }],
  });
  return selected ?? null;
}

export async function pickCsvFile(): Promise<string | null> {
  const selected = await open({
    multiple: false,
    filters: [{ name: 'CSV', extensions: ['csv', 'txt'] }],
  });
  return typeof selected === 'string' ? selected : null;
}

// Home & Office & Countries
export const getHomeSummary = () => invoke<HomeSummary>('get_home_summary');
export const listCountries = () => invoke<Country[]>('list_countries');
export const getOffice = () => invoke<Office>('get_office');
export const updateOffice = (input: OfficeInput) => invoke<Office>('update_office', { input });

// Practitioners
export const listPractitioners = (filter: PractitionerFilter) =>
  invoke<PaginatedResult<PractitionerView>>('list_practitioners', { filter });
export const getPractitioner = (id: number) =>
  invoke<PractitionerView>('get_practitioner', { id });
export const createPractitioner = (input: PractitionerInput) =>
  invoke<PractitionerView>('create_practitioner', { input });
export const updatePractitioner = (id: number, input: PractitionerInput) =>
  invoke<PractitionerView>('update_practitioner', { id, input });

// Medical Treatment Types
export const listTreatmentTypes = (filter: TreatmentTypeFilter) =>
  invoke<PaginatedResult<MedicalTreatmentType>>('list_treatment_types', { filter });
export const getTreatmentType = (id: number) =>
  invoke<MedicalTreatmentType>('get_treatment_type', { id });
export const createTreatmentType = (input: TreatmentTypeInput) =>
  invoke<MedicalTreatmentType>('create_treatment_type', { input });
export const updateTreatmentType = (id: number, input: TreatmentTypeInput) =>
  invoke<MedicalTreatmentType>('update_treatment_type', { id, input });
export const deleteTreatmentType = (id: number) => invoke<void>('delete_treatment_type', { id });

// Bank Accounts
export const listBankAccounts = (filter: BankAccountFilter) =>
  invoke<PaginatedResult<BankAccount>>('list_bank_accounts', { filter });
export const getBankAccount = (id: number) => invoke<BankAccount>('get_bank_account', { id });
export const createBankAccount = (input: BankAccountInput) =>
  invoke<BankAccount>('create_bank_account', { input });
export const updateBankAccount = (id: number, input: BankAccountInput) =>
  invoke<BankAccount>('update_bank_account', { id, input });

// Patients
export const listPatients = (filter: PatientFilter) =>
  invoke<PaginatedResult<PatientView>>('list_patients', { filter });
export const getPatient = (id: number) => invoke<PatientView>('get_patient', { id });
export const createPatient = (input: PatientInput) => invoke<PatientView>('create_patient', { input });
export const updatePatient = (id: number, input: PatientInput) =>
  invoke<PatientView>('update_patient', { id, input });
export const deletePatient = (id: number) => invoke<void>('delete_patient', { id });
export const finderPatients = (query: string, limit?: number) =>
  invoke<PatientChoice[]>('finder_patients', { query, limit });
export const listPractitionerOptions = () =>
  invoke<PractitionerOption[]>('list_practitioner_options');
export const importPatientsCsv = (path: string) =>
  invoke<ImportPatientsResult>('import_patients_csv', { path });

// Medical Treatments
export const listTreatments = (filter: TreatmentFilter) =>
  invoke<PaginatedResult<MedicalTreatmentView>>('list_treatments', { filter });
export const getTreatment = (id: number) => invoke<MedicalTreatmentDetail>('get_treatment', { id });
export const createTreatment = (input: MedicalTreatmentInput) =>
  invoke<MedicalTreatmentView>('create_treatment', { input });
export const updateTreatment = (id: number, input: MedicalTreatmentInput) =>
  invoke<MedicalTreatmentView>('update_treatment', { id, input });
export const deleteTreatment = (id: number) => invoke<void>('delete_treatment', { id });
export const priceFinder = (medicalTreatmentTypeId: number) =>
  invoke<number | null>('price_finder', { medicalTreatmentTypeId });
export const listTreatmentTypeOptions = () =>
  invoke<MedicalTreatmentType[]>('list_treatment_type_options');

// Payments
export const listPayments = (filter: PaymentFilter) =>
  invoke<PaginatedResult<PaymentView>>('list_payments', { filter });
export const listPaymentListing = (filter: PaymentListingFilter) =>
  invoke<PaymentListingResult>('list_payment_listing', { filter });
export const getPayment = (id: number) => invoke<PaymentView>('get_payment', { id });
export const createPayment = (input: PaymentInput) =>
  invoke<PaymentView>('create_payment', { input });
export const updatePayment = (id: number, input: PaymentInput) =>
  invoke<PaymentView>('update_payment', { id, input });
export const deletePayment = (id: number) => invoke<void>('delete_payment', { id });
export const listTreatmentOptionsForPayment = () =>
  invoke<MedicalTreatmentOption[]>('list_treatment_options_for_payment');

// Bank Deposits
export const listBankDeposits = (filter: BankDepositFilter) =>
  invoke<PaginatedResult<BankDepositView>>('list_bank_deposits', { filter });
export const getBankDeposit = (id: number) =>
  invoke<BankDepositDetail>('get_bank_deposit', { id });
export const listPendingBankChecks = () =>
  invoke<BankCheckDepositRow[]>('list_pending_bank_checks');
export const listBankAccountOptions = () =>
  invoke<BankAccount[]>('list_bank_account_options');
export const createBankDeposit = (input: BankDepositInput) =>
  invoke<BankDepositDetail>('create_bank_deposit', { input });
export const deleteBankDeposit = (id: number) => invoke<void>('delete_bank_deposit', { id });
