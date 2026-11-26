import {
  getDbStatus,
  listUsers,
  parseApiError,
  type DbStatus,
  type Office,
  type User,
} from './api';
import { t, type Locale, type TranslationKey } from './i18n';

export type FlashType = 'success' | 'error' | 'info' | 'warning';

export interface FlashMessage {
  id: number;
  type: FlashType;
  message: string;
}

export type ResourceMode = 'index' | 'new' | 'show' | 'edit';
export type PatientRouteMode = ResourceMode | 'import';
export type PaymentRouteMode = ResourceMode | 'listing';
export type BankDepositRouteMode = 'index' | 'new' | 'show';

export interface PatientFiltersState {
  first_name: string;
  last_name: string;
  phone: string;
  diabetic: boolean;
  page: number;
  per_page: number;
}

export interface TreatmentFiltersState {
  patient_id: number | null;
  medical_treatment_type_id: number | null;
  date_gteq: string;
  date_lteq: string;
  location_id: number | null;
  status: number | null;
  page: number;
  per_page: number;
}

export interface PaymentFiltersState {
  paid_at: string;
  payment_type: number | null;
  page: number;
  per_page: number;
}

export interface PaymentListingFiltersState {
  paid_at_gteq: string;
  paid_at_lteq: string;
  payment_type: number | null;
}

export interface BankDepositFiltersState {
  deposit_type: number | null;
  number: number | null;
  page: number;
}

export const PER_PAGE_OPTIONS: readonly {
  value: number;
  label?: string;
  labelKey?: 'allOption';
}[] = [
  { value: 2, label: '2' },
  { value: 10, label: '10' },
  { value: 20, label: '20' },
  { value: 50, label: '50' },
  { value: 100, label: '100' },
  { value: 10000000, labelKey: 'allOption' },
];

export function defaultPatientFilters(): PatientFiltersState {
  return {
    first_name: '',
    last_name: '',
    phone: '',
    diabetic: false,
    page: 1,
    per_page: 10,
  };
}

export function defaultTreatmentFilters(): TreatmentFiltersState {
  return {
    patient_id: null,
    medical_treatment_type_id: null,
    date_gteq: '',
    date_lteq: '',
    location_id: null,
    status: null,
    page: 1,
    per_page: 10,
  };
}

export function defaultPaymentFilters(): PaymentFiltersState {
  return {
    paid_at: '',
    payment_type: null,
    page: 1,
    per_page: 10,
  };
}

export function defaultPaymentListingFilters(): PaymentListingFiltersState {
  return {
    paid_at_gteq: '',
    paid_at_lteq: '',
    payment_type: null,
  };
}

export function defaultBankDepositFilters(): BankDepositFiltersState {
  return {
    deposit_type: null,
    number: null,
    page: 1,
  };
}

export type ParsedRoute =
  | { view: 'home' }
  | { view: 'setup' }
  | { view: 'office-edit' }
  | { view: 'patients'; mode: PatientRouteMode; id?: number; query?: Record<string, string> }
  | { view: 'practitioners'; mode: ResourceMode; id?: number }
  | { view: 'treatment-types'; mode: ResourceMode; id?: number }
  | { view: 'bank-accounts'; mode: ResourceMode; id?: number }
  | {
      view: 'medical-treatments';
      mode: ResourceMode;
      id?: number;
      query?: Record<string, string>;
    }
  | {
      view: 'payments';
      mode: PaymentRouteMode;
      id?: number;
      query?: Record<string, string>;
    }
  | {
      view: 'bank-deposits';
      mode: BankDepositRouteMode;
      id?: number;
    };

const LOCALE_STORAGE_KEY = 'medical-office.locale';

function detectInitialLocale(): Locale {
  if (typeof window !== 'undefined') {
    const saved = window.localStorage.getItem(LOCALE_STORAGE_KEY);
    if (saved === 'en' || saved === 'fr') return saved;
    if (window.navigator.language?.toLowerCase().startsWith('fr')) return 'fr';
  }
  return 'en';
}

export function parseHashRoute(hash: string): ParsedRoute {
  const cleaned = hash.replace(/^#/, '') || '/';
  const [rawPath, rawQuery = ''] = cleaned.split('?');
  const path = rawPath.startsWith('/') ? rawPath : `/${rawPath}`;
  const segments = path.split('/').filter(Boolean);
  const query: Record<string, string> = {};
  if (rawQuery) {
    const params = new URLSearchParams(rawQuery);
    params.forEach((v, k) => {
      query[k] = v;
    });
  }

  if (segments.length === 0) return { view: 'home' };

  const [head, second, third] = segments;

  if (head === 'setup') return { view: 'setup' };
  if (head === 'office' && second === 'edit') return { view: 'office-edit' };

  if (head === 'patients') {
    if (!second) return { view: 'patients', mode: 'index', query };
    if (second === 'new') return { view: 'patients', mode: 'new' };
    if (second === 'import' || second === 'new_import') return { view: 'patients', mode: 'import' };
    const id = Number(second);
    if (Number.isInteger(id) && id > 0) {
      if (third === 'edit') return { view: 'patients', mode: 'edit', id };
      return { view: 'patients', mode: 'show', id };
    }
    return { view: 'patients', mode: 'index', query };
  }

  if (head === 'medical-treatments') {
    if (!second) return { view: 'medical-treatments', mode: 'index', query };
    if (second === 'new') return { view: 'medical-treatments', mode: 'new', query };
    const id = Number(second);
    if (Number.isInteger(id) && id > 0) {
      if (third === 'edit') return { view: 'medical-treatments', mode: 'edit', id };
      return { view: 'medical-treatments', mode: 'show', id };
    }
    return { view: 'medical-treatments', mode: 'index', query };
  }

  if (head === 'practitioners' || head === 'treatment-types' || head === 'bank-accounts') {
    if (!second) return { view: head, mode: 'index' };
    if (second === 'new') return { view: head, mode: 'new' };
    const id = Number(second);
    if (Number.isInteger(id) && id > 0) {
      if (third === 'edit') return { view: head, mode: 'edit', id };
      return { view: head, mode: 'show', id };
    }
    return { view: head, mode: 'index' };
  }

  if (head === 'payments') {
    if (!second) return { view: 'payments', mode: 'index', query };
    if (second === 'listing') return { view: 'payments', mode: 'listing', query };
    if (second === 'new') return { view: 'payments', mode: 'new', query };
    const id = Number(second);
    if (Number.isInteger(id) && id > 0) {
      if (third === 'edit') return { view: 'payments', mode: 'edit', id };
      return { view: 'payments', mode: 'show', id };
    }
    return { view: 'payments', mode: 'index', query };
  }

  if (head === 'bank-deposits') {
    if (!second) return { view: 'bank-deposits', mode: 'index' };
    if (second === 'new') return { view: 'bank-deposits', mode: 'new' };
    const id = Number(second);
    if (Number.isInteger(id) && id > 0) {
      return { view: 'bank-deposits', mode: 'show', id };
    }
    return { view: 'bank-deposits', mode: 'index' };
  }

  return { view: 'home' };
}

class AppState {
  dbStatus = $state<DbStatus | null>(null);
  users = $state<User[]>([]);
  locale = $state<Locale>(detectInitialLocale());
  flashes = $state<FlashMessage[]>([]);
  hash = $state<string>(typeof window !== 'undefined' ? window.location.hash || '#/' : '#/');
  initializing = $state<boolean>(true);
  patientFilters = $state<PatientFiltersState>(defaultPatientFilters());
  treatmentFilters = $state<TreatmentFiltersState>(defaultTreatmentFilters());
  paymentFilters = $state<PaymentFiltersState>(defaultPaymentFilters());
  paymentListingFilters = $state<PaymentListingFiltersState>(defaultPaymentListingFilters());
  bankDepositFilters = $state<BankDepositFiltersState>(defaultBankDepositFilters());
  private nextFlashId = 1;

  currentUser = $derived<User | null>(this.dbStatus?.active_user ?? null);
  currentOffice = $derived<Office | null>(this.dbStatus?.office ?? null);
  isInitialized = $derived<boolean>(Boolean(this.dbStatus?.initialized));
  needsOfficeSetup = $derived<boolean>(Boolean(this.dbStatus?.needs_office_setup));
  route = $derived<ParsedRoute>(parseHashRoute(this.hash));

  tr(key: TranslationKey, params?: Record<string, string | number>): string {
    return t(this.locale, key, params);
  }

  setLocale(next: Locale) {
    this.locale = next;
    if (typeof window !== 'undefined') {
      window.localStorage.setItem(LOCALE_STORAGE_KEY, next);
    }
  }

  navigate(path: string) {
    const target = path.startsWith('#') ? path : `#${path.startsWith('/') ? path : `/${path}`}`;
    if (typeof window !== 'undefined') {
      if (window.location.hash === target) {
        this.hash = target;
      } else {
        window.location.hash = target;
      }
    } else {
      this.hash = target;
    }
  }

  addFlash(type: FlashType, message: string, autoDismissMs = 5000) {
    const id = this.nextFlashId++;
    this.flashes = [...this.flashes, { id, type, message }];
    if (autoDismissMs > 0 && typeof window !== 'undefined') {
      window.setTimeout(() => {
        this.dismissFlash(id);
      }, autoDismissMs);
    }
  }

  dismissFlash(id: number) {
    this.flashes = this.flashes.filter((f) => f.id !== id);
  }

  clearFlashes() {
    this.flashes = [];
  }

  async applyDbStatus(status: DbStatus) {
    const prevUserId = this.dbStatus?.active_user?.id ?? null;
    const prevDbPath = this.dbStatus?.db_path ?? null;
    this.dbStatus = status;
    const nextUserId = status.active_user?.id ?? null;
    const nextDbPath = status.db_path ?? null;
    if (prevUserId !== nextUserId || prevDbPath !== nextDbPath) {
      this.patientFilters = defaultPatientFilters();
      this.treatmentFilters = defaultTreatmentFilters();
      this.paymentFilters = defaultPaymentFilters();
      this.paymentListingFilters = defaultPaymentListingFilters();
      this.bankDepositFilters = defaultBankDepositFilters();
    }
    if (status.initialized && status.users_count > 1) {
      try {
        this.users = await listUsers();
      } catch {
        this.users = status.active_user ? [status.active_user] : [];
      }
    } else {
      this.users = status.active_user ? [status.active_user] : [];
    }
  }

  async refreshDbStatus() {
    const status = await getDbStatus();
    await this.applyDbStatus(status);
    return status;
  }

  async init() {
    this.initializing = true;
    try {
      const status = await getDbStatus();
      await this.applyDbStatus(status);
      if (!status.initialized) {
        this.navigate('/setup');
      } else if (this.route.view === 'setup') {
        this.navigate('/');
      }
    } catch (err) {
      this.addFlash('error', parseApiError(err, this.locale).message, 0);
    } finally {
      this.initializing = false;
    }
  }
}

export const appState = new AppState();
