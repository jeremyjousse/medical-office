// This is a manifest file that'll be compiled into application.js, which will include all the files
// listed below.
//
// Any JavaScript/Coffee file within this directory, lib/assets/javascripts, vendor/assets/javascripts,
// or vendor/assets/javascripts of plugins, if any, can be referenced here using a relative path.
//
// It's not advisable to add code directly here, but if you do, it'll appear at the bottom of the
// compiled file.
//
// Read Sprockets README (https://github.com/sstephenson/sprockets#sprockets-directives) for details
// about supported directives.
//
//= require jquery
//= require jquery_ujs
//= require turbolinks
//= require bootstrap/dropdown
//= require bootstrap/modal
//= require bootstrap/alert


//= require select2
//= require bootstrap-datepicker/core
//= require bootstrap-datepicker/locales/bootstrap-datepicker.fr.js


function add_fields(link, association, content, callback) {
  var new_id = new Date().getTime();
  var regexp = new RegExp("new_" + association, "g")
  $(link).parent().before(content.replace(regexp, new_id));

  if (callback != '')
  {

  	var fn = window[callback];

  	if (typeof fn === "function") fn();
  }
}

function reload_datepickers(){
	$(".datepicker").datepicker({"format": "yyyy-mm-dd", "weekStart": 1, "autoclose": true});
}

$(function() {
	$(".alert").alert();
});
