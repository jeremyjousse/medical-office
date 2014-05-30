class CreatePosturalAnalysis < ActiveRecord::Migration
  def change
    create_table :postural_analyses do |t|
      t.integer :user_id
      t.integer :patient_id
      t.integer :shoe_size

      t.timestamps
    end
  end
end
