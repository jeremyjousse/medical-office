module ShowHelper

	def generate_show_page_item(name, value)
		haml_tag :tr do
			haml_tag :th do
				haml_concat name
			end
			haml_tag :td do
				haml_concat value
			end
		end
	end

	def generate_show_page_item_old(name, value)
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