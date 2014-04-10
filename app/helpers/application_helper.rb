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


  def flash_class(level)
    case level
      when :notice then "info"
      when :error then "danger"
      when :alert then "warning"
    end
  end

end
