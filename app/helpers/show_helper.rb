module ShowHelper

	def generate_show_page_item(name, value)
		haml_tag :div, :class => "show-group" do
			haml_tag :div, :class => "show-label" do
				haml_concat name
			end
			haml_tag :div, :class => "show-value" do
				haml_concat value
			end
		end
	end

end