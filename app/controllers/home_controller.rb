class HomeController < ApplicationController
  def index
    # @users = User.all

    logger.debug index_path(:locale => I18n.locale)
    #logger.debug root_url(:locale => I18n.locale)
  end
end