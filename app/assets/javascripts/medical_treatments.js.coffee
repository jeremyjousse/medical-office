$ ->
	$(".datepicker").datepicker({"format": "yyyy-mm-dd", "weekStart": 1, "autoclose": true})
	$("#medical_treatment_patient_id").select2()
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
  $("#medical_treatment_medical_treatment_type_id").change ->
    id = $("#medical_treatment_medical_treatment_type_id").val()
    url = "/fr/medical_treatment_types/:id/price_finder.json"
    url = url.replace(/:id/, id)
    $.getJSON url, (data) =>
      $("#medical_treatment_price").val(data['text'])
  $("#ajax_modal").modal
