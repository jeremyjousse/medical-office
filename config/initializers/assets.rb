Rails.application.config.assets.precompile += %w( bank_accounts.js bank_deposits.js medical_treatments.js patients.js payments.js practitioners.js ) if Rails.env.development?
