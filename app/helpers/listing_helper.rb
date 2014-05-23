module ListingHelper

	def put_and_get_search_params_in_session(controller_name, params_hash , filter)

		search_params_hash = {search: {}, page: 1, per_page: 10}

		if session[:listing_filter_params].nil?
			session[:listing_filter_params] = {}
		end


		if filter == 'reset'
		 	params_hash = {search: {}, page: 1, per_page: 10, reseted: true}


		elsif session[:listing_filter_controller_name] == controller_name

			if !params_hash[:search].nil?

				params_hash[:search].each do |search_param_key,search_param_value|
					search_params_hash[:search][search_param_key] = search_param_value

				end



if search_params_hash[:search].diff(ActiveSupport::JSON.decode(session[:listing_filter_params][:search])) != {}
	params_hash[:page] = 1
end


			else
				search_params_hash[:search] = ActiveSupport::JSON.decode(session[:listing_filter_params][:search])
			end

			if !params_hash[:page].nil?
				search_params_hash[:page] = params_hash[:page]
			else
				search_params_hash[:page] = session[:listing_filter_params][:page]
			end

			if !params_hash[:per_page].nil?
				search_params_hash[:per_page] = params_hash[:per_page]
			else
				search_params_hash[:per_page] = session[:listing_filter_params][:per_page]
			end

		end

#logger.info '-------'+search_params_hash[:search].inspect
		session[:listing_filter_controller_name] = controller_name
		session[:listing_filter_params][:search] = search_params_hash[:search].to_json
		session[:listing_filter_params][:page] = search_params_hash[:page]
		session[:listing_filter_params][:per_page] = search_params_hash[:per_apge]


		if !params_hash[:reseted].nil? && params_hash[:reseted] == true

			respond_to do |format|
					format.html { redirect_to "/" + I18n.locale.to_s + "/"+controller_path }
			end


		end


		search_params_hash

	end

end
