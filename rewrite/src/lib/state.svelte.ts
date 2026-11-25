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

export type ParsedRoute =
  | { view: 'home' }
  | { view: 'setup' }
  | { view: 'office-edit' }
  | { view: 'practitioners'; mode: ResourceMode; id?: number }
  | { view: 'treatment-types'; mode: ResourceMode; id?: number }
  | { view: 'bank-accounts'; mode: ResourceMode; id?: number }
  | {
      view: 'deferred';
      module: 'patients' | 'medical-treatments' | 'payments' | 'bank-deposits';
      query: Record<string, string>;
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

  if (
    head === 'patients' ||
    head === 'medical-treatments' ||
    head === 'payments' ||
    head === 'bank-deposits'
  ) {
    return { view: 'deferred', module: head, query };
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
    this.dbStatus = status;
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
