class PosturalAnalysis < ActiveRecord::Base

  belongs_to :user
  belongs_to :patient
  has_many :postural_analysis_scans

  enum left_right_handed: {'Right Handed' => 1, 'Left Handed' => 2}

end
