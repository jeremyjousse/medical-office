module ApplicationHelper

	def link_to_add_fields(name, f, association, callback)
    new_object = f.object.class.reflect_on_association(association).klass.new
    fields = f.fields_for(association, new_object, :child_index => "new_#{association}") do |builder|
      render("shared/" + association.to_s.singularize + "_form", :f => builder)
    end
    link_to_function(name, "add_fields(this, \"#{association}\", \"#{escape_javascript(fields)}\", \"#{callback}\")", :class => "btn")
  end

	def hidden_add_association_fields(f, association)
    new_object = f.object.class.reflect_on_association(association).klass.new
    fields = f.fields_for(association, new_object, :child_index => "new_#{association}") do |builder|
      render(association.to_s.pluralize + "/modal_form", :f => builder)
    end

    haml_tag :div, id: "hidden_#{association}_association_fields", data: "#{escape_javascript(fields)}" do
		end
  end

  def simple_form_notification_message(message)

    if !message.nil?
      haml_tag :div, :class => "alert alert-danger alert-dismissable" do
        haml_tag :button, type: :button , class: "close", data: {dismiss: "alert"}, aria: {hidden: "true"} do
          haml_concat "&times;"
        end
        haml_concat message
      end
    end

  end


  # def flash_class(level)
  #   case level
  #     when :notice then "info"
  #     when :error then "danger"
  #     when :alert then "warning"
  #   end
  # end

	def get_flash_type_to_bootstrap_alert_class(type)
    case type.to_s
      when 'alert'   then "alert alert-warning"
      when 'error'   then "alert alert-danger"
      when 'notice'  then "alert alert-info"
      when 'success' then "alert alert-success"
      else "alert alert-info"
    end
  end

   class BootstrapLinkRenderer < ::WillPaginate::ActionView::LinkRenderer
    protected

    def html_container(html)
      tag :ul, html, container_attributes
    end

    def page_number(page)
      tag :li, link(page, page, :rel => rel_value(page)), :class => ('active' if page == current_page)
    end

    def gap
      tag :li, link(super, '#'), :class => 'disabled'
    end

    def previous_or_next_page(page, text, classname)
      tag :li, link(text, page || '#'), :class => [classname[0..3], classname, ('disabled' unless page)].join(' ')
    end
  end

  def page_navigation_links(pages)
    will_paginate(pages, :class => 'pagination', :inner_window => 2, :outer_window => 0, :renderer => BootstrapLinkRenderer, :previous_label => '&larr;'.html_safe, :next_label => '&rarr;'.html_safe)
  end

end
