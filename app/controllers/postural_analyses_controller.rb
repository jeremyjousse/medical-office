class PosturalAnalysesController < ApplicationController

  def new
    @postural_analysis = PosturalAnalysis.new
    @postural_analysis.patient_id = params[:patient_id]
  end

  def create
    @postural_analysis = PosturalAnalysis.new(new_postural_analysis_params)

    @postural_analysis.user = current_user

    respond_to do |format|
      if @postural_analysis.save
        #format.html { redirect_to @medical_treatment, notice: 'Medical treatment was successfully created.' }
        #format.json { render action: 'show', status: :created, location: @medical_treatment }
        format.js { render 'reload' }
      else
        #format.html { render action: 'new' }
        #format.json { render json: @medical_treatment.errors, status: :unprocessable_entity }
        format.js { render 'new', {patient_id: params[:patient_id]}}
      end
    end

  end

  def edit
    @postural_analysis = current_user.postural_analyses.find(params[:id])

    @postural_analysis_scan = @postural_analysis.postural_analysis_scans.build

    @postural_analysis_scans = @postural_analysis.postural_analysis_scans.all


  end

  def update
    @postural_analysis = current_user.postural_analyses.find(params[:id])

    if @postural_analysis.user_id != current_user.id
      exit()
    end

    respond_to do |format|
      if @postural_analysis.update(edit_postural_analysis_params)
        format.js { render 'reload' }
      else
        format.js { render 'edit'}
      end
    end
  end

  private

  def new_postural_analysis_params
    params.require(:postural_analysis).permit(:user_id, :patient_id, :shoe_size, :sole_pattern, :pain, :medical_history, :radiologic_examinations, :headache, :giddiness, :falls, :lying_down_eo_mo, :lying_down_ec_mo, :lying_down_eo_mc, :jaw_centring, :tooth_fillings, :popping_jaw, :jaw_opening, :temporal_muscles, :micro_current, :podoscope, :stork_balance_test, :left_right_handed, :barre_vertical_test, :ilium, :scapulum, :plumb_line, :walking, :cervical_rotation, :thumb_test, :dynamic_posture_test, :romberg_test, :trampling, :extensors_test, :eye_convergence, :master_eye, :maddox_test, :visual_correction, :atlas, :jammed)
  end

  def edit_postural_analysis_params
    params.require(:postural_analysis).permit(                       :shoe_size, :sole_pattern, :pain, :medical_history, :radiologic_examinations, :headache, :giddiness, :falls, :lying_down_eo_mo, :lying_down_ec_mo, :lying_down_eo_mc, :jaw_centring, :tooth_fillings, :popping_jaw, :jaw_opening, :temporal_muscles, :micro_current, :podoscope, :stork_balance_test, :left_right_handed, :barre_vertical_test, :ilium, :scapulum, :plumb_line, :walking, :cervical_rotation, :thumb_test, :dynamic_posture_test, :romberg_test, :trampling, :extensors_test, :eye_convergence, :master_eye, :maddox_test, :visual_correction, :atlas, :jammed)
  end

end
