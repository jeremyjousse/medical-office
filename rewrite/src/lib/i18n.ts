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
  yes: 'Yes',
  no: 'No',
  showCountPrefix: 'Show',
  perPageSuffix: 'per page',
  allOption: 'All',
  toRange: 'to',

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

  // Patients
  patientsTitle: 'My Patients',
  patientSingular: 'Patient',
  patientPlural: 'Patients',
  patientDetailTitle: 'Patient Detail',
  patientNewTitle: 'New Patient',
  patientEditTitle: 'Edit Patient',
  patientImportTitle: 'Import Patients',
  patientNewBtn: 'New Patient',
  patientImportBtn: 'Import Patients',
  patientFirstName: 'First name',
  patientLastName: 'Last name',
  patientBirthdate: 'Birthday',
  patientProfession: 'Profession',
  patientPhone: 'Phone',
  patientMobilePhone: 'Mobile phone',
  patientEmail: 'Email',
  patientAddress: 'Address',
  patientPostalCode: 'Postal code',
  patientCity: 'City',
  patientCountry: 'Country',
  patientNationalInsuranceNumber: 'National insurance number',
  patientFamilyDoctor: 'Family doctor',
  patientPhysicalTherapist: 'Physical therapist',
  patientOsteopath: 'Osteopath',
  patientLegalGuardian: 'Legal guardian',
  patientNote: 'Note',
  patientDiabetic: 'Diabetic',
  patientUniversalHealthcare: 'Universal healthcare coverage',
  patientLongDurationDisease: 'Long duration disease',
  patientDiabeteNetwork: 'Diabetes network',
  patientTreatmentsCount: 'Nb. treatments',
  patientAddTreatment: 'Add Medical Treatment',
  patientAddDocumentScan: 'Add Document Scan',
  patientCreatedSuccess: 'Patient was successfully created.',
  patientUpdatedSuccess: 'Patient was successfully updated.',
  patientDeletedSuccess: 'Patient was successfully deleted.',
  patientDeleteConfirmTitle: 'Delete patient?',
  patientDeleteConfirmBody: 'Are you sure you want to delete "{name}"? This action cannot be undone.',
  patientImportSubtitle: 'Select a Google Contacts CSV export (UTF-16LE with BOM). Existing patients with the same last name, first name, and phone are skipped automatically.',
  patientImportChooseFile: 'Choose Google Contacts CSV',
  patientImportSelectedFile: 'Selected file',
  patientImportSubmit: 'Import',
  patientImportSummary: 'Imported {imported} patient(s), skipped {skipped} of {total} row(s).',
  patientImportSuccess: '{count} patient(s) imported successfully.',
  patientSearchPlaceholder: 'Search patient by name...',

  // Document Scans
  documentScansTitle: 'Document Scans',
  documentScanColumn: 'Document Scan',
  documentScanDate: 'Date',
  documentScanNewTitle: 'Add Document Scan',
  documentScanDetailTitle: 'Document Scan',
  documentScanPatient: 'Patient',
  documentScanTitle: 'Title',
  documentScanFile: 'File',
  documentScanChooseFile: 'Choose file (JPG, PNG, GIF, PDF)',
  documentScanSelectedFile: 'Selected file',
  documentScanAddBtn: 'Add',
  documentScanFileRequired: 'Please select a file to upload.',
  documentScanCreatedSuccess: 'Document scan was successfully created.',
  documentScanMissingFile:
    'The scan file is not present on disk (legacy database record or moved file).',
  documentScanOpenViewer: 'Open in macOS Viewer',
  documentScanNoRecords: 'No document scans.',

  // Medical Treatments
  treatmentsTitle: 'Medical Treatments',
  treatmentSingular: 'Medical Treatment',
  treatmentPlural: 'Medical Treatments',
  treatmentDetailTitle: 'Medical Treatment Detail',
  treatmentNewTitle: 'New Medical Treatment',
  treatmentEditTitle: 'Edit Medical Treatment',
  treatmentNewBtn: 'New Medical Treatment',
  treatmentPatient: 'Patient',
  treatmentType: 'Treatment type',
  treatmentDate: 'Date',
  treatmentLocation: 'Location',
  treatmentPrice: 'Price',
  treatmentNote: 'Note',
  treatmentStatus: 'Status',
  treatmentAllTypes: 'All types',
  treatmentAllLocations: 'All locations',
  treatmentAllStatuses: 'All statuses',
  treatmentLocationOffice: 'Office',
  treatmentLocationHome: 'Home',
  treatmentStatusUnpaid: 'unpaid',
  treatmentStatusPaid: 'paid',
  treatmentCreatedSuccess: 'Medical treatment was successfully created.',
  treatmentUpdatedSuccess: 'Medical treatment was successfully updated.',
  treatmentDeletedSuccess: 'Medical treatment was successfully deleted.',
  treatmentDeleteConfirmTitle: 'Delete medical treatment?',
  treatmentDeleteConfirmBody: 'Are you sure you want to delete this medical treatment from {date}? This action cannot be undone.',
  treatmentPaymentsHeading: 'Payments',
  treatmentPaymentPaidAt: 'Paid at',
  treatmentPaymentType: 'Type',
  treatmentPaymentAmount: 'Amount',
  treatmentAddPayment: 'Add payment',

  // Payments
  paymentsTitle: 'Payments',
  paymentSingular: 'Payment',
  paymentPlural: 'Payments',
  paymentDetailTitle: 'Payment Detail',
  paymentNewTitle: 'New Payment',
  paymentEditTitle: 'Edit Payment',
  paymentNewBtn: 'New Payment',
  paymentPrintListingBtn: 'Print listing',
  paymentMedicalTreatment: 'Medical treatment',
  paymentPaidAt: 'Paid at',
  paymentType: 'Type',
  paymentAmount: 'Amount',
  paymentBankDeposit: 'Bank deposit',
  paymentAllTypes: 'All types',
  paymentType1: 'Check',
  paymentType2: 'Credit card',
  paymentType3: 'Cash',
  paymentType4: 'Bank account transfer',
  paymentCheckSection: 'Bank Check Details',
  paymentCheckOwner: 'Account owner',
  paymentCheckBankName: 'Bank name',
  paymentCheckNumber: 'Check number',
  paymentCheckStatus: 'Check status',
  paymentCheckStatusNeg1: 'Rejected',
  paymentCheckStatus0: 'Uncashed',
  paymentCheckStatus1: 'Cashed',
  paymentCreatedSuccess: 'Payment was successfully created.',
  paymentUpdatedSuccess: 'Payment was successfully updated.',
  paymentDeletedSuccess: 'Payment was successfully deleted.',
  paymentDeleteConfirmTitle: 'Delete payment?',
  paymentDeleteConfirmBody: 'Are you sure you want to delete this payment? This action cannot be undone.',
  paymentDepositedReadOnlyNotice:
    'This payment is included in a bank deposit; payment type, amount, and medical treatment cannot be modified.',

  // Payment Listing
  paymentListingTitle: 'Payment listing',
  print: 'Print',
  paymentListingDayTotal: 'Day total',
  paymentListingGrandTotal: 'Grand total',
  paymentListingAll: 'All',

  // Bank Deposits
  bankDepositsTitle: 'My Bank Deposits',
  bankDepositSingular: 'Bank Deposit',
  bankDepositPlural: 'Bank Deposits',
  bankDepositNewTitle: 'New Bank Deposit',
  bankDepositDetailTitle: 'Bank Deposit Slip',
  bankDepositNewBtn: 'New Bank Deposit',
  bankDepositDate: 'Date',
  bankDepositType: 'Type',
  bankDepositNumber: 'Number',
  bankDepositAmount: 'Amount',
  bankDepositBankAccount: 'Bank account',
  bankDepositAllTypes: 'All types',
  bankDepositType1: 'Check',
  bankDepositType2: 'Cash',
  bankDepositPendingChecks: 'Pending checks',
  bankDepositNoPendingChecks: 'No pending checks available for deposit.',
  bankDepositSelectAll: 'Select all',
  bankDepositDeselectAll: 'Deselect all',
  bankDepositSelectedSummary: '{count} check(s) selected — Total: {total}',
  bankDepositNoAccountsWarning: 'Please create a bank account in Settings before creating a bank deposit.',
  bankDepositCreatedSuccess: 'Bank deposit was successfully created.',
  bankDepositDeletedSuccess: 'Bank deposit was successfully deleted.',
  bankDepositDeleteConfirmTitle: 'Delete bank deposit?',
  bankDepositDeleteConfirmBody:
    'Are you sure you want to delete deposit #{number}? Linked checks will return to uncashed status.',
  bankDepositSlipAccountNo: 'Account no.:',
  bankDepositSlipDatePrefix: '{city}, {date}',
  bankDepositSlipNumberLabel: 'Check deposit no. {number}',
  bankDepositSlipCheckCount: 'No. of checks: {count}',
  bankDepositSlipTotal: 'Total {total}',

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
  err_invalid_practitioner: 'Please select a valid practitioner.',
  err_invalid_patient: 'Please select a valid patient.',
  err_invalid_date: 'Please enter a valid date (YYYY-MM-DD).',
  err_invalid_location: 'Please select a valid location.',
  err_invalid_treatment_type: 'Please select a valid medical treatment type.',
  err_payments_exceed_price: 'Total payments cannot exceed treatment price.',
  err_patient_has_treatments: 'Cannot delete patient because they have existing medical treatments.',
  err_treatment_has_payments: 'Cannot delete medical treatment because it has existing payments.',
  err_payment_in_bank_deposit: 'Cannot modify or delete a payment included in a bank deposit.',
  err_invalid_payment_type: 'Please select a valid payment type.',
  err_invalid_amount: 'Please enter a valid positive amount.',
  err_invalid_treatment: 'Please select a valid medical treatment.',
  err_invalid_bank_account: 'Please select a valid bank account.',
  err_invalid_deposit_type: 'Only check deposits are supported.',
  err_no_checks_selected: 'Please select at least one pending check.',
  err_invalid_bank_check: 'One or more selected checks are invalid or already cashed.',
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
  yes: 'Oui',
  no: 'Non',
  showCountPrefix: 'Afficher',
  perPageSuffix: 'par page',
  allOption: 'Tous',
  toRange: 'à',

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

  // Patients
  patientsTitle: 'Mes patients',
  patientSingular: 'Patient',
  patientPlural: 'Patients',
  patientDetailTitle: 'Détail du patient',
  patientNewTitle: 'Nouveau patient',
  patientEditTitle: 'Modifier le patient',
  patientImportTitle: 'Importer des patients',
  patientNewBtn: 'Nouveau patient',
  patientImportBtn: 'Importer des patients',
  patientFirstName: 'Prénom',
  patientLastName: 'Nom',
  patientBirthdate: 'Date de naissance',
  patientProfession: 'Profession',
  patientPhone: 'Téléphone',
  patientMobilePhone: 'Téléphone portable',
  patientEmail: 'E-mail',
  patientAddress: 'Adresse',
  patientPostalCode: 'Code postal',
  patientCity: 'Ville',
  patientCountry: 'Pays',
  patientNationalInsuranceNumber: 'Numéro de sécurité sociale',
  patientFamilyDoctor: 'Médecin traitant',
  patientPhysicalTherapist: 'Kinésithérapeute',
  patientOsteopath: 'Ostéopathe',
  patientLegalGuardian: 'Représentant légal',
  patientNote: 'Note',
  patientDiabetic: 'Diabétique',
  patientUniversalHealthcare: 'CMU / CSS',
  patientLongDurationDisease: 'ALD',
  patientDiabeteNetwork: 'Réseau diabète',
  patientTreatmentsCount: 'Nb. soins',
  patientAddTreatment: 'Ajouter un soin médical',
  patientAddDocumentScan: 'Ajouter un document scanné',
  patientCreatedSuccess: 'Le patient a été créé avec succès.',
  patientUpdatedSuccess: 'Le patient a été mis à jour avec succès.',
  patientDeletedSuccess: 'Le patient a été supprimé avec succès.',
  patientDeleteConfirmTitle: 'Supprimer le patient ?',
  patientDeleteConfirmBody: 'Voulez-vous vraiment supprimer « {name} » ? Cette action est irréversible.',
  patientImportSubtitle: 'Sélectionnez un fichier CSV Google Contacts (UTF-16LE avec BOM). Les patients existants ayant les mêmes nom, prénom et téléphone sont ignorés automatiquement.',
  patientImportChooseFile: 'Choisir un CSV Google Contacts',
  patientImportSelectedFile: 'Fichier sélectionné',
  patientImportSubmit: 'Importer',
  patientImportSummary: '{imported} patient(s) importé(s), {skipped} ignoré(s) sur {total} ligne(s).',
  patientImportSuccess: '{count} patient(s) importé(s) avec succès.',
  patientSearchPlaceholder: 'Rechercher un patient par nom...',

  // Document Scans
  documentScansTitle: 'Documents scannés',
  documentScanColumn: 'Document scanné',
  documentScanDate: 'Date',
  documentScanNewTitle: 'Ajouter un document scanné',
  documentScanDetailTitle: 'Document scanné',
  documentScanPatient: 'Patient',
  documentScanTitle: 'Titre',
  documentScanFile: 'Fichier',
  documentScanChooseFile: 'Choisir un fichier (JPG, PNG, GIF, PDF)',
  documentScanSelectedFile: 'Fichier sélectionné',
  documentScanAddBtn: 'Ajouter',
  documentScanFileRequired: 'Veuillez sélectionner un fichier à importer.',
  documentScanCreatedSuccess: 'Le document scanné a été ajouté avec succès.',
  documentScanMissingFile:
    'Le fichier de ce document scanné est absent du disque (enregistrement importé ou fichier déplacé).',
  documentScanOpenViewer: 'Ouvrir dans l’aperçu macOS',
  documentScanNoRecords: 'Aucun document scanné.',

  // Medical Treatments
  treatmentsTitle: 'Soins médicaux',
  treatmentSingular: 'Soin médical',
  treatmentPlural: 'Soins médicaux',
  treatmentDetailTitle: 'Détail du soin médical',
  treatmentNewTitle: 'Nouveau soin médical',
  treatmentEditTitle: 'Modifier le soin médical',
  treatmentNewBtn: 'Nouveau soin médical',
  treatmentPatient: 'Patient',
  treatmentType: 'Type de soin',
  treatmentDate: 'Date',
  treatmentLocation: 'Lieu',
  treatmentPrice: 'Tarif',
  treatmentNote: 'Note',
  treatmentStatus: 'Statut',
  treatmentAllTypes: 'Tous les types',
  treatmentAllLocations: 'Tous les lieux',
  treatmentAllStatuses: 'Tous les statuts',
  treatmentLocationOffice: 'Cabinet',
  treatmentLocationHome: 'Domicile',
  treatmentStatusUnpaid: 'Impayé',
  treatmentStatusPaid: 'Payé',
  treatmentCreatedSuccess: 'Le soin médical a été créé avec succès.',
  treatmentUpdatedSuccess: 'Le soin médical a été mis à jour avec succès.',
  treatmentDeletedSuccess: 'Le soin médical a été supprimé avec succès.',
  treatmentDeleteConfirmTitle: 'Supprimer le soin médical ?',
  treatmentDeleteConfirmBody: 'Voulez-vous vraiment supprimer ce soin médical du {date} ? Cette action est irréversible.',
  treatmentPaymentsHeading: 'Paiements',
  treatmentPaymentPaidAt: 'Payé le',
  treatmentPaymentType: 'Type',
  treatmentPaymentAmount: 'Montant',
  treatmentAddPayment: 'Ajouter un paiement',

  // Payments
  paymentsTitle: 'Paiements',
  paymentSingular: 'Paiement',
  paymentPlural: 'Paiements',
  paymentDetailTitle: 'Détail du paiement',
  paymentNewTitle: 'Nouveau paiement',
  paymentEditTitle: 'Modifier le paiement',
  paymentNewBtn: 'Nouveau paiement',
  paymentPrintListingBtn: 'Imprimer le listing',
  paymentMedicalTreatment: 'Soin médical',
  paymentPaidAt: 'Payé le',
  paymentType: 'Type',
  paymentAmount: 'Montant',
  paymentBankDeposit: 'Remise en banque',
  paymentAllTypes: 'Tous les types',
  paymentType1: 'Chèque',
  paymentType2: 'Carte bancaire',
  paymentType3: 'Espèces',
  paymentType4: 'Virement bancaire',
  paymentCheckSection: 'Détails du chèque',
  paymentCheckOwner: 'Titulaire du compte',
  paymentCheckBankName: 'Banque',
  paymentCheckNumber: 'Numéro de chèque',
  paymentCheckStatus: 'Statut du chèque',
  paymentCheckStatusNeg1: 'Rejeté',
  paymentCheckStatus0: 'Non encaissé',
  paymentCheckStatus1: 'Encaissé',
  paymentCreatedSuccess: 'Le paiement a été créé avec succès.',
  paymentUpdatedSuccess: 'Le paiement a été mis à jour avec succès.',
  paymentDeletedSuccess: 'Le paiement a été supprimé avec succès.',
  paymentDeleteConfirmTitle: 'Supprimer le paiement ?',
  paymentDeleteConfirmBody: 'Voulez-vous vraiment supprimer ce paiement ? Cette action est irréversible.',
  paymentDepositedReadOnlyNotice:
    'Ce paiement est inclus dans une remise en banque ; le type, le montant et le soin médical ne peuvent pas être modifiés.',

  // Payment Listing
  paymentListingTitle: 'Listing des paiements',
  print: 'Imprimer',
  paymentListingDayTotal: 'Total du jour',
  paymentListingGrandTotal: 'Total général',
  paymentListingAll: 'Tous',

  // Bank Deposits
  bankDepositsTitle: 'Mes remises en banque',
  bankDepositSingular: 'Remise en banque',
  bankDepositPlural: 'Remises en banque',
  bankDepositNewTitle: 'Nouvelle remise en banque',
  bankDepositDetailTitle: 'Bordereau de remise en banque',
  bankDepositNewBtn: 'Nouvelle remise en banque',
  bankDepositDate: 'Date',
  bankDepositType: 'Type',
  bankDepositNumber: 'Numéro',
  bankDepositAmount: 'Montant',
  bankDepositBankAccount: 'Compte bancaire',
  bankDepositAllTypes: 'Tous les types',
  bankDepositType1: 'Chèque',
  bankDepositType2: 'Espèces',
  bankDepositPendingChecks: 'Chèques en attente',
  bankDepositNoPendingChecks: 'Aucun chèque en attente à remettre en banque.',
  bankDepositSelectAll: 'Tout sélectionner',
  bankDepositDeselectAll: 'Tout désélectionner',
  bankDepositSelectedSummary: '{count} chèque(s) sélectionné(s) — Total : {total}',
  bankDepositNoAccountsWarning:
    'Veuillez créer un compte bancaire dans les Paramètres avant de créer une remise en banque.',
  bankDepositCreatedSuccess: 'La remise en banque a été créée avec succès.',
  bankDepositDeletedSuccess: 'La remise en banque a été supprimée avec succès.',
  bankDepositDeleteConfirmTitle: 'Supprimer la remise en banque ?',
  bankDepositDeleteConfirmBody:
    'Voulez-vous vraiment supprimer la remise n° {number} ? Les chèques associés repasseront au statut non encaissé.',
  bankDepositSlipAccountNo: 'Compte n° :',
  bankDepositSlipDatePrefix: '{city} le {date}',
  bankDepositSlipNumberLabel: 'Remise de chèque n° {number}',
  bankDepositSlipCheckCount: 'Nb. chèques {count}',
  bankDepositSlipTotal: 'Total {total}',

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
  err_invalid_practitioner: 'Veuillez sélectionner un praticien valide.',
  err_invalid_patient: 'Veuillez sélectionner un patient valide.',
  err_invalid_date: 'Veuillez saisir une date valide (AAAA-MM-JJ).',
  err_invalid_location: 'Veuillez sélectionner un lieu valide.',
  err_invalid_treatment_type: 'Veuillez sélectionner un type de soin valide.',
  err_payments_exceed_price: 'Le total des paiements ne peut pas dépasser le tarif du soin.',
  err_patient_has_treatments: 'Impossible de supprimer ce patient car il possède des soins médicaux.',
  err_treatment_has_payments: 'Impossible de supprimer ce soin médical car il possède des paiements.',
  err_payment_in_bank_deposit:
    'Impossible de modifier ou supprimer un paiement inclus dans une remise en banque.',
  err_invalid_payment_type: 'Veuillez sélectionner un type de paiement valide.',
  err_invalid_amount: 'Veuillez saisir un montant strictement positif valide.',
  err_invalid_treatment: 'Veuillez sélectionner un soin médical valide.',
  err_invalid_bank_account: 'Veuillez sélectionner un compte bancaire valide.',
  err_invalid_deposit_type: 'Seules les remises de chèques sont prises en charge.',
  err_no_checks_selected: 'Veuillez sélectionner au moins un chèque en attente.',
  err_invalid_bank_check: 'Un ou plusieurs chèques sélectionnés sont invalides ou déjà encaissés.',
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

export function locationLabel(locale: Locale, locationId: number | null | undefined): string {
  switch (locationId) {
    case 1:
      return t(locale, 'treatmentLocationOffice');
    case 2:
      return t(locale, 'treatmentLocationHome');
    default:
      return '—';
  }
}

export function locationOptions(locale: Locale): { value: number; label: string }[] {
  return [
    { value: 1, label: t(locale, 'treatmentLocationOffice') },
    { value: 2, label: t(locale, 'treatmentLocationHome') },
  ];
}

export function treatmentStatusLabel(locale: Locale, status: number | null | undefined): string {
  switch (status) {
    case 1:
      return t(locale, 'treatmentStatusPaid');
    case 0:
      return t(locale, 'treatmentStatusUnpaid');
    default:
      return '—';
  }
}

export function treatmentStatusOptions(locale: Locale): { value: number; label: string }[] {
  return [
    { value: 0, label: t(locale, 'treatmentStatusUnpaid') },
    { value: 1, label: t(locale, 'treatmentStatusPaid') },
  ];
}

export function paymentTypeLabel(locale: Locale, paymentType: number | null | undefined): string {
  switch (paymentType) {
    case 1:
      return t(locale, 'paymentType1');
    case 2:
      return t(locale, 'paymentType2');
    case 3:
      return t(locale, 'paymentType3');
    case 4:
      return t(locale, 'paymentType4');
    default:
      return '—';
  }
}

export function paymentTypeOptions(locale: Locale): { value: number; label: string }[] {
  return [
    { value: 1, label: t(locale, 'paymentType1') },
    { value: 2, label: t(locale, 'paymentType2') },
    { value: 3, label: t(locale, 'paymentType3') },
    { value: 4, label: t(locale, 'paymentType4') },
  ];
}

export function paymentCheckStatusLabel(
  locale: Locale,
  status: number | null | undefined,
): string {
  switch (status) {
    case -1:
      return t(locale, 'paymentCheckStatusNeg1');
    case 0:
      return t(locale, 'paymentCheckStatus0');
    case 1:
      return t(locale, 'paymentCheckStatus1');
    default:
      return '—';
  }
}

export function bankDepositTypeLabel(
  locale: Locale,
  depositType: number | null | undefined,
): string {
  switch (depositType) {
    case 1:
      return t(locale, 'bankDepositType1');
    case 2:
      return t(locale, 'bankDepositType2');
    default:
      return '—';
  }
}

export function bankDepositTypeOptions(locale: Locale): { value: number; label: string }[] {
  return [
    { value: 1, label: t(locale, 'bankDepositType1') },
    { value: 2, label: t(locale, 'bankDepositType2') },
  ];
}

const eurFormatterFr = new Intl.NumberFormat('fr-FR', {
  style: 'currency',
  currency: 'EUR',
});

export function formatEur(amount: number | null | undefined): string {
  if (amount === null || amount === undefined || !Number.isFinite(amount)) {
    return '—';
  }
  return eurFormatterFr.format(amount);
}

export function formatSlipDate(raw: string | null | undefined, locale: Locale): string {
  if (!raw) return '';
  const datePart = raw.trim().slice(0, 10);
  const parts = datePart.split('-').map(Number);
  if (parts.length !== 3 || parts.some((n) => !Number.isFinite(n))) {
    return raw;
  }
  const [y, m, d] = parts;
  const dt = new Date(Date.UTC(y, m - 1, d));
  return new Intl.DateTimeFormat(locale === 'fr' ? 'fr-FR' : 'en-GB', {
    day: '2-digit',
    month: 'short',
    year: 'numeric',
    timeZone: 'UTC',
  }).format(dt);
}
