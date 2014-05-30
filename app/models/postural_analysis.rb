class PosturalAnalysis < ActiveRecord::Base

  belongs_to :user
  belongs_to :patient


  enum left_right_handed: {'Right Handed' => 1, 'Left Handed' => 2}

end
