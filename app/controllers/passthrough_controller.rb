class PassthroughController < ApplicationController

  def index
    # if user_signed_in?
    #   if current_user.has_role? :admin
    #     path = admin_schools_path(:locale => I18n.locale)
    #   else
    #     path = school_path(current_user.school.id, :locale => I18n.locale)
    #   end
    # else
      path = index_path(:locale => I18n.locale)

      logger.info '+++++++++++++++++' + I18n.locale.to_s
      logger.info '+++++++++++++++++' + path.to_s
    # end

    redirect_to path     
  end
end