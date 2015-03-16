class AddUserRefToPosturalAnalysisScans < ActiveRecord::Migration
  def change
    add_reference :postural_analysis_scans, :user, index: true
  end
end
