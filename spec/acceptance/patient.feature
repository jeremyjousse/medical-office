Feature: Patients

	Background:
		Given I am signed in as "jean.dupond@test.com" "1234qerty"

	@patient
	Scenario: Add a patient with valid informations
		When I add a patient with valid informations
		Then I should see patient show page