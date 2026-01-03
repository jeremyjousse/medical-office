# Medical Office Management System

## Project Overview
This project is a Ruby on Rails application designed for managing a medical office. It handles patients, medical treatments, practitioners, payments, and scheduling. It appears to be a legacy application updated to run on Ruby 2.6.10 and Rails 4.2.

**Key Technologies:**
*   **Framework:** Ruby on Rails 4.2.11.3
*   **Language:** Ruby 2.6.10
*   **Database:** SQLite3
*   **Templating:** HAML
*   **Authentication:** Devise
*   **Frontend:** Bootstrap (Sass), CoffeeScript, jQuery
*   **Testing:** RSpec, Turnip (for acceptance tests), Capybara, FactoryGirl

## Building and Running

### Prerequisites
Ensure you have Ruby 2.6.10 installed (e.g., via rbenv or rvm).

### Installation
1.  Install dependencies:
    ```bash
    bundle install
    ```
2.  Setup the database:
    ```bash
    bundle exec rake db:setup
    ```
    *Note: `db:setup` will create the database, load the schema, and run seeds.*

### Running the Application
Start the Rails server:
```bash
bundle exec rails server
```
The application will be accessible at `http://localhost:3000`.
**Note:** The root route redirects to `passthrough#index`, and most main features are scoped under a locale (e.g., `http://localhost:3000/en/`).

## Testing
The project uses RSpec for unit and functional tests, and Turnip for acceptance testing (Gherkin syntax).

To run the full test suite:
```bash
bundle exec rspec
```

To run acceptance tests specifically (Turnip):
```bash
bundle exec rspec -r turnip/rspec spec/acceptance
```
*Note: Acceptance steps are located in `spec/steps/`.*

## Development Conventions

*   **Architecture:** Standard Rails MVC architecture.
*   **Views:** HAML is used for views instead of ERB.
*   **Forms:** `simple_form` is used for form generation.
*   **Uploads:** `carrierwave` is used for handling file uploads (e.g., `DocumentScan`, `PosturalAnalysisScan`).
*   **Internationalization:** The application uses Rails I18n with routes scoped by locale (e.g., `/:locale/...`).
*   **Code Style:** `rubocop` is included in the Gemfile for linting.
*   **Database Cleaning:** The test suite uses `DatabaseCleaner` with a truncation strategy, explicitly preserving the `countries` table during cleanup.
