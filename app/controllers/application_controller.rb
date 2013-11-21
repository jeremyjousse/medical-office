class ApplicationController < ActionController::Base
  # Prevent CSRF attacks by raising an exception.
  # For APIs, you may want to use :null_session instead.
  protect_from_forgery with: :exception

  # Get locale from URL
  before_filter :set_locale

  def self.default_url_options(options={})
    options.merge({ :locale => I18n.locale })
  end

  def set_locale
  	I18n.locale = params[:locale] || extract_locale_from_url_then_accept_language_header

      #I18n.default_locale = :fr

    # if I18n.locale == 'fr'
      
    # end
  end


  private
    def extract_locale_from_url_then_accept_language_header
      return 'en' if request.env["HTTP_ACCEPT_LANGUAGE"].nil?

      lng = request.env['HTTP_ACCEPT_LANGUAGE'].scan(/^[a-z]{2}/).first
      lng = 'en' unless lng == 'fr'
      lng
    end

end
