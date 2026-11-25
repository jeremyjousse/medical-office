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
