$ ->
  $("#medical_treatment_medical_treatment_type_id").on "change", ->
    alert('eeee')
	$(".datepicker").datepicker({"format": "yyyy-mm-dd", "weekStart": 1, "autoclose": true})
	$("#patient_country_id").select2()
	$("#patient_family_doctor_id").select2()
	$("#patient_physical_therapists_id").select2()
	$("#patient_osteopath_id").select2()
	$(".select2").each (i, e) ->
    select = $(e)
    options = {}
    if select.hasClass("ajax")
      options.ajax =
        url: select.data("source")
        dataType: "json"
        data: (term, page) ->
          q: term
          page: page
          per: 10

        results: (data, page) ->
          results: data

      options.dropdownCssClass = "bigdrop"
    select.select2 options
  $("#ajax_modal").modal
