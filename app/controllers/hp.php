<!-- <div id="sign_in_sign_up_panel">
	<div class="well text-center">
	    <h3>{{dc.hc_template_connect_to_access}}</h3>
	    <a href="/login" class="btn btn-default">{{dc.hc_template_login_button_text}}</a><span>&nbsp;&nbsp;&nbsp;ou&nbsp;&nbsp;&nbsp;</span><a href="/registration" class="btn btn-default">{{dc.hc_template_sign_in_button_text}}</a>
	</div>
</div> -->
<div id="large_search_box">
	<h3>{{dc.hc_template_your_question}}</h3>
	{{search}}
	<div id="suggested_searches">
		{{dc.hc_template_popular_search}}
	</div>
</div>

<div id="hp_knowledge_base">
  {{category_list}}
</div>

<div class="row">
	<div class="col-md-12">
		<div id="hp_promoted_articles">
			<div class="panel panel-default">
			  <div class="panel-heading"><h4>{{dc.hc_template_promoted_articles_text}}</h4></div>
			  <div class="panel-body">
			    {{promoted_articles}}
			  </div>
			</div>
		</div>
	</div>
</div>
