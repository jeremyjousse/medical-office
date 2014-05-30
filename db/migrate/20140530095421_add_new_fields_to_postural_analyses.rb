class AddNewFieldsToPosturalAnalyses < ActiveRecord::Migration
  def change
    add_column :postural_analyses, :medical_history, :text
    add_column :postural_analyses, :radiologic_examinations, :text
    add_column :postural_analyses, :headache, :string
    add_column :postural_analyses, :giddiness, :string
    add_column :postural_analyses, :falls, :string
    add_column :postural_analyses, :lying_down_eo_mo, :string
    add_column :postural_analyses, :lying_down_ec_mo, :string
    add_column :postural_analyses, :lying_down_eo_mc, :string
    add_column :postural_analyses, :jaw_centring, :string
    add_column :postural_analyses, :tooth_fillings, :string
    add_column :postural_analyses, :popping_jaw, :string
    add_column :postural_analyses, :jaw_opening, :string
    add_column :postural_analyses, :temporal_muscles, :string
    add_column :postural_analyses, :micro_current, :string
    add_column :postural_analyses, :podoscope, :string
    add_column :postural_analyses, :stork_balance_test, :string
    add_column :postural_analyses, :left_right_handed, :integer
    add_column :postural_analyses, :barre_vertical_test, :integer
    add_column :postural_analyses, :ilium, :string
    add_column :postural_analyses, :scapulum, :string
    add_column :postural_analyses, :plumb_line, :string
    add_column :postural_analyses, :walking, :text
    add_column :postural_analyses, :cervical_rotation, :string
    add_column :postural_analyses, :thumb_test, :string
    add_column :postural_analyses, :dynamic_posture_test, :string
    add_column :postural_analyses, :romberg_test, :string
    add_column :postural_analyses, :trampling, :string
    add_column :postural_analyses, :extensors_test, :string
    add_column :postural_analyses, :eye_convergence, :string
    add_column :postural_analyses, :master_eye, :string
    add_column :postural_analyses, :maddox_test, :string
    add_column :postural_analyses, :visual_correction, :string
    add_column :postural_analyses, :atlas, :string
    add_column :postural_analyses, :jammed, :string
  end
end
