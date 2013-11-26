class HomeController < ApplicationController
  def index
    # @users = User.all

    logger.debug index_path(:locale => I18n.locale)

    if user_signed_in?
    	if current_user.office.nil?
    		office = Office.new
    		office.save

    		user = User.find_by_id(current_user.id)
    		user.office_id = office.id
    		user.save

    		redirect_to edit_office_path(office, :locale => I18n.locale) and return

    	end
    end

    #logger.debug root_url(:locale => I18n.locale)
  end
end