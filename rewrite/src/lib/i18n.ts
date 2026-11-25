export type Locale = 'en' | 'fr';

const en = {
  // Brand & Nav
  appTitle: 'Medical Office',
  navHome: 'Home',
  navPatients: 'Patients',
  navPractitioners: 'Practitioners',
  navMedicalTreatments: 'Medical Treatments',
  navPayments: 'Payments',
  navBankDeposits: 'Bank Deposits',
  navSettings: 'Settings',
  navMyOffice: 'My Office',
  navBankAccounts: 'Bank Accounts',
  navTreatmentTypes: 'Medical Treatment Types',
  navImportDb: 'Import SQLite DB',
  navExportDb: 'Export SQLite DB',
  navProfile: 'Active Profile',
  navLanguage: 'Language',

  // Common
  back: 'Back',
  save: 'Save',
  saving: 'Saving...',
  edit: 'Edit',
  show: 'Show',
  detail: 'Detail',
  destroy: 'Destroy',
  filter: 'Filter',
  reset: 'Reset',
  cancel: 'Cancel',
  confirm: 'Confirm',
  previous: 'Previous',
  next: 'Next',
  loading: 'Loading...',
  noRecords: 'No records found.',
  selectedOfTotal: '{selected} {label} selected / {total}',
  searchPlaceholder: 'Search...',
  noMatches: 'No matches',
  clearSelection: 'Clear',
  actions: 'Actions',
  close: 'Close',

  // Setup DB
  setupTitle: 'Database Setup',
  setupSubtitle:
    'Choose an existing legacy Rails SQLite database or initialize a fresh empty database to get started.',
  setupImportTitle: 'Import existing SQLite DB',
  setupImportDesc:
    'Select a `.sqlite3` file. Your database will be copied into the app data folder and validated.',
  setupImportBtn: 'Choose SQLite file',
  setupCreateTitle: 'Create empty DB',
  setupCreateDesc:
    'Create a new compatible SQLite database seeded with 251 countries, a default user, and an empty office.',
  setupCreateBtn: 'Create empty database',
  setupImportedSuccess: 'SQLite database imported successfully.',
  setupCreatedSuccess: 'Empty database created successfully.',
  setupExportSuccess: 'Database exported to {path}.',
  setupImportConfirmTitle: 'Replace active database?',
  setupImportConfirmBody:
    'Importing a new SQLite file will replace the currently active database in the application data folder. Make sure you have exported a backup if needed.',
  setupUserSwitched: 'Switched active profile to {email}.',

  // Home
  homeWelcome: 'Welcome {email}',
  homeWelcomeOffice: 'Welcome {email} — {office}',
  homeUnpaidSingular: '1 Medical Treatment unpaid',
  homeUnpaidPlural: '{count} Medical Treatments unpaid',
  homeAllPaid: 'All medical treatments are paid.',
  homeOfficeRequiredNotice: 'Please complete your office details before continuing.',

  // Office
  officeTitle: 'My Office',
  officeName: 'Name',
  officeAddress: 'Address',
  officePostalCode: 'Postal code',
  officeCity: 'City',
  officeCountry: 'Country',
  officePhone: 'Phone',
  officeWebsite: 'Website',
  officeEmail: 'Email',
  officeUpdatedSuccess: 'Office was successfully updated.',

  // Practitioners
  practitionersTitle: 'Practitioners',
  practitionerSingular: 'Practitioner',
  practitionerPlural: 'Practitioners',
  practitionerDetailTitle: 'Practitioner Detail',
  practitionerNewTitle: 'New Practitioner',
  practitionerEditTitle: 'Edit Practitioner',
  practitionerNewBtn: 'New Practitioner',
  practitionerSpeciality: 'Speciality',
  practitionerFirstName: 'First name',
  practitionerLastName: 'Last name',
  practitionerPhone: 'Phone',
  practitionerMobilePhone: 'Mobile phone',
  practitionerEmail: 'Email',
  practitionerAddress: 'Address',
  practitionerPostalCode: 'Postal code',
  practitionerCity: 'City',
  practitionerCountry: 'Country',
  practitionerNote: 'Note',
  practitionerAllSpecialities: 'All specialities',
  practitionerCreatedSuccess: 'Practitioner was successfully created.',
  practitionerUpdatedSuccess: 'Practitioner was successfully updated.',
  speciality1: 'Family doctor',
  speciality2: 'Physical Therapists',
  speciality3: 'Osteopath',

  // Medical Treatment Types
  treatmentTypesTitle: 'Medical Treatment Types',
  treatmentTypeSingular: 'Medical Treatment Type',
  treatmentTypePlural: 'Medical Treatment Types',
  treatmentTypeDetailTitle: 'Medical Treatment Type Detail',
  treatmentTypeNewTitle: 'New Medical Treatment Type',
  treatmentTypeEditTitle: 'Edit Medical Treatment Type',
  treatmentTypeNewBtn: 'New Medical Treatment Type',
  treatmentTypeName: 'Name',
  treatmentTypePrice: 'Price',
  treatmentTypeCreatedSuccess: 'Medical treatment type was successfully created.',
  treatmentTypeUpdatedSuccess: 'Medical treatment type was successfully updated.',
  treatmentTypeDeletedSuccess: 'Medical treatment type was successfully deleted.',
  treatmentTypeDeleteConfirmTitle: 'Delete medical treatment type?',
  treatmentTypeDeleteConfirmBody:
    'Are you sure you want to delete "{name}"? This action cannot be undone.',

  // Bank Accounts
  bankAccountsTitle: 'Bank Accounts',
  bankAccountSingular: 'Bank Account',
  bankAccountPlural: 'Bank Accounts',
  bankAccountDetailTitle: 'Bank Account',
  bankAccountNewTitle: 'New Bank Account',
  bankAccountEditTitle: 'Edit Bank Account',
  bankAccountNewBtn: 'New Bank Account',
  bankAccountName: 'Bank name',
  bankAccountNumber: 'Account number',
  bankAccountDepositNumber: 'Last deposit number',
  bankAccountCreatedSuccess: 'Bank account was successfully created.',
  bankAccountUpdatedSuccess: 'Bank account was successfully updated.',

  // Deferred routes
  deferredTitle: 'Module Scheduled in Next Lot',
  deferredDesc: 'The "{module}" module is outside the current scope and will be implemented in an upcoming lot.',
  deferredFilterInfo: 'Active filter parameter: {filter}',

  // Validation codes
  err_required: 'This field is required.',
  err_too_short: 'This field is too short.',
  err_too_long: 'This field is too long.',
  err_invalid_email: 'Please enter a valid email address.',
  err_invalid_url: 'URL must start with http:// or https://.',
  err_invalid_speciality: 'Please select a valid speciality.',
  err_invalid_country: 'Please select a valid country.',
  err_invalid_price: 'Please enter a valid non-negative price.',
  err_invalid_integer: 'Please enter a valid non-negative integer.',
  err_referenced_by_treatments:
    'Cannot delete medical treatment type because it is referenced by existing medical treatments.',
} as const;

export type TranslationKey = keyof typeof en;

const fr: Record<TranslationKey, string> = {
  // Brand & Nav
  appTitle: 'Cabinet Médical',
  navHome: 'Accueil',
  navPatients: 'Patients',
  navPractitioners: 'Praticiens',
  navMedicalTreatments: 'Soins médicaux',
  navPayments: 'Paiements',
  navBankDeposits: 'Remises en banque',
  navSettings: 'Paramètres',
  navMyOffice: 'Mon cabinet',
  navBankAccounts: 'Comptes bancaires',
  navTreatmentTypes: 'Types de soins',
  navImportDb: 'Importer une base SQLite',
  navExportDb: 'Exporter la base SQLite',
  navProfile: 'Profil actif',
  navLanguage: 'Langue',

  // Common
  back: 'Retour',
  save: 'Enregistrer',
  saving: 'Enregistrement...',
  edit: 'Modifier',
  show: 'Voir',
  detail: 'Détail',
  destroy: 'Supprimer',
  filter: 'Filtrer',
  reset: 'Réinitialiser',
  cancel: 'Annuler',
  confirm: 'Confirmer',
  previous: 'Précédent',
  next: 'Suivant',
  loading: 'Chargement...',
  noRecords: 'Aucun enregistrement trouvé.',
  selectedOfTotal: '{selected} {label} sélectionné(s) / {total}',
  searchPlaceholder: 'Rechercher...',
  noMatches: 'Aucun résultat',
  clearSelection: 'Effacer',
  actions: 'Actions',
  close: 'Fermer',

  // Setup DB
  setupTitle: 'Configuration de la base de données',
  setupSubtitle:
    'Importez une base SQLite Rails existante ou créez une nouvelle base vide pour démarrer.',
  setupImportTitle: 'Importer une base SQLite existante',
  setupImportDesc:
    'Sélectionnez un fichier `.sqlite3`. La base sera copiée dans le dossier de données de l’application et validée.',
  setupImportBtn: 'Choisir un fichier SQLite',
  setupCreateTitle: 'Créer une base vide',
  setupCreateDesc:
    'Crée une nouvelle base SQLite compatible avec les 251 pays, un utilisateur par défaut et un cabinet vide.',
  setupCreateBtn: 'Créer une base vide',
  setupImportedSuccess: 'Base SQLite importée avec succès.',
  setupCreatedSuccess: 'Base vide créée avec succès.',
  setupExportSuccess: 'Base exportée vers {path}.',
  setupImportConfirmTitle: 'Remplacer la base active ?',
  setupImportConfirmBody:
    'L’importation d’un nouveau fichier SQLite remplacera la base actuellement active dans le dossier de l’application. Pensez à exporter une sauvegarde si nécessaire.',
  setupUserSwitched: 'Profil actif basculé sur {email}.',

  // Home
  homeWelcome: 'Bienvenue {email}',
  homeWelcomeOffice: 'Bienvenue {email} — {office}',
  homeUnpaidSingular: '1 soin médical impayé',
  homeUnpaidPlural: '{count} soins médicaux impayés',
  homeAllPaid: 'Tous les soins médicaux sont payés.',
  homeOfficeRequiredNotice: 'Veuillez compléter les informations du cabinet avant de continuer.',

  // Office
  officeTitle: 'Mon cabinet',
  officeName: 'Nom',
  officeAddress: 'Adresse',
  officePostalCode: 'Code postal',
  officeCity: 'Ville',
  officeCountry: 'Pays',
  officePhone: 'Téléphone',
  officeWebsite: 'Site web',
  officeEmail: 'E-mail',
  officeUpdatedSuccess: 'Le cabinet a été mis à jour avec succès.',

  // Practitioners
  practitionersTitle: 'Praticiens',
  practitionerSingular: 'Praticien',
  practitionerPlural: 'Praticiens',
  practitionerDetailTitle: 'Détail du praticien',
  practitionerNewTitle: 'Nouveau praticien',
  practitionerEditTitle: 'Modifier le praticien',
  practitionerNewBtn: 'Nouveau praticien',
  practitionerSpeciality: 'Spécialité',
  practitionerFirstName: 'Prénom',
  practitionerLastName: 'Nom',
  practitionerPhone: 'Téléphone',
  practitionerMobilePhone: 'Téléphone portable',
  practitionerEmail: 'E-mail',
  practitionerAddress: 'Adresse',
  practitionerPostalCode: 'Code postal',
  practitionerCity: 'Ville',
  practitionerCountry: 'Pays',
  practitionerNote: 'Note',
  practitionerAllSpecialities: 'Toutes les spécialités',
  practitionerCreatedSuccess: 'Le praticien a été créé avec succès.',
  practitionerUpdatedSuccess: 'Le praticien a été mis à jour avec succès.',
  speciality1: 'Médecin traitant',
  speciality2: 'Kinésithérapeute',
  speciality3: 'Ostéopathe',

  // Medical Treatment Types
  treatmentTypesTitle: 'Types de soins médicaux',
  treatmentTypeSingular: 'Type de soin',
  treatmentTypePlural: 'Types de soins',
  treatmentTypeDetailTitle: 'Détail du type de soin',
  treatmentTypeNewTitle: 'Nouveau type de soin',
  treatmentTypeEditTitle: 'Modifier le type de soin',
  treatmentTypeNewBtn: 'Nouveau type de soin',
  treatmentTypeName: 'Nom',
  treatmentTypePrice: 'Tarif',
  treatmentTypeCreatedSuccess: 'Le type de soin a été créé avec succès.',
  treatmentTypeUpdatedSuccess: 'Le type de soin a été mis à jour avec succès.',
  treatmentTypeDeletedSuccess: 'Le type de soin a été supprimé avec succès.',
  treatmentTypeDeleteConfirmTitle: 'Supprimer le type de soin ?',
  treatmentTypeDeleteConfirmBody:
    'Voulez-vous vraiment supprimer « {name} » ? Cette action est irréversible.',

  // Bank Accounts
  bankAccountsTitle: 'Comptes bancaires',
  bankAccountSingular: 'Compte bancaire',
  bankAccountPlural: 'Comptes bancaires',
  bankAccountDetailTitle: 'Compte bancaire',
  bankAccountNewTitle: 'Nouveau compte bancaire',
  bankAccountEditTitle: 'Modifier le compte bancaire',
  bankAccountNewBtn: 'Nouveau compte bancaire',
  bankAccountName: 'Nom de la banque',
  bankAccountNumber: 'Numéro de compte',
  bankAccountDepositNumber: 'Dernier numéro de remise',
  bankAccountCreatedSuccess: 'Le compte bancaire a été créé avec succès.',
  bankAccountUpdatedSuccess: 'Le compte bancaire a été mis à jour avec succès.',

  // Deferred routes
  deferredTitle: 'Module prévu dans le lot suivant',
  deferredDesc: 'Le module « {module} » est hors périmètre actuel et sera implémenté dans un prochain lot.',
  deferredFilterInfo: 'Paramètre de filtre actif : {filter}',

  // Validation codes
  err_required: 'Ce champ est obligatoire.',
  err_too_short: 'Ce champ est trop court.',
  err_too_long: 'Ce champ est trop long.',
  err_invalid_email: 'Veuillez saisir une adresse e-mail valide.',
  err_invalid_url: 'L’URL doit commencer par http:// ou https://.',
  err_invalid_speciality: 'Veuillez sélectionner une spécialité valide.',
  err_invalid_country: 'Veuillez sélectionner un pays valide.',
  err_invalid_price: 'Veuillez saisir un tarif positif ou nul valide.',
  err_invalid_integer: 'Veuillez saisir un entier positif ou nul valide.',
  err_referenced_by_treatments:
    'Impossible de supprimer ce type de soin car il est utilisé par des soins médicaux existants.',
};

const dictionaries: Record<Locale, Record<TranslationKey, string>> = { en, fr };

export function t(
  locale: Locale,
  key: TranslationKey,
  params?: Record<string, string | number>,
): string {
  let text = dictionaries[locale][key] ?? en[key] ?? key;
  if (params) {
    for (const [k, v] of Object.entries(params)) {
      text = text.replaceAll(`{${k}}`, String(v));
    }
  }
  return text;
}

export function specialityLabel(locale: Locale, specialityId: number | null | undefined): string {
  switch (specialityId) {
    case 1:
      return t(locale, 'speciality1');
    case 2:
      return t(locale, 'speciality2');
    case 3:
      return t(locale, 'speciality3');
    default:
      return '—';
  }
}

export function specialityOptions(locale: Locale): { value: number; label: string }[] {
  return [
    { value: 1, label: t(locale, 'speciality1') },
    { value: 2, label: t(locale, 'speciality2') },
    { value: 3, label: t(locale, 'speciality3') },
  ];
}

export function translateErrorCode(locale: Locale, code: string, fallback: string): string {
  const key = `err_${code}` as TranslationKey;
  if (key in dictionaries[locale]) {
    return dictionaries[locale][key];
  }
  return fallback;
}
