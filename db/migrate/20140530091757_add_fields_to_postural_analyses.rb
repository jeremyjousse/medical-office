class AddFieldsToPosturalAnalyses < ActiveRecord::Migration
  def change
    add_column :postural_analyses, :sole_pattern, :string
    add_column :postural_analyses, :pain, :text
  end
end
