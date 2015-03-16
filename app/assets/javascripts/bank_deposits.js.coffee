# Place all the behaviors and hooks related to the matching controller here.
# All this logic will automatically be available in application.js.
# You can use CoffeeScript in this file: http://coffeescript.org/
$ ->
  $("#bank_deposit_type").change ->
    url = "/fr/payment_bank_checks/pending"
    $.ajax(url: url).done (html) ->
      $("#bank_deposit_payments").html html

