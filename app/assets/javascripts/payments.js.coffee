payments_ready = ->
	$(".datepicker").datepicker({"format": "yyyy-mm-dd", "weekStart": 1, "autoclose": true})

$ ->
	payments_ready()

$(document).on('page:load', payments_ready())
