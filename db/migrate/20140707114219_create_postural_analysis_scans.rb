class CreatePosturalAnalysisScans < ActiveRecord::Migration
  def change
    create_table :postural_analysis_scans do |t|
      t.references :postural_analysis, index: true
      t.string :scan
    end
  end
end
