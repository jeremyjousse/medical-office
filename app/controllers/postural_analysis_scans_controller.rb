class PosturalAnalysisScansController < ApplicationController




  def new
    @postural_analysis_scan = PosturalAnalysisScan.new
    @postural_analysis.postural_analysis_id = params[:postural_analysis_id]
  end

  def create
    p_attr = {}
    #p_attr = postural_analysis_scan_params
    p_attr[:scan] = postural_analysis_scan_params[:scan].first if postural_analysis_scan_params[:scan].class == Array
    p_attr[:user_id] = current_user.id

    if params[:postural_analysis_id]
      @postural_analysis = PosturalAnalysis.find(params[:postural_analysis_id])
      @postural_analysis_scan = @postural_analysis.postural_analysis_scans.build(p_attr)
    end

    if @postural_analysis_scan.save
      respond_to do |format|
        format.html {
          render :json => [@postural_analysis_scan.to_jq_upload].to_json,
          :content_type => 'text/html',
          :layout => false
        }
        format.json {
          render :json => [@postural_analysis_scan.to_jq_upload].to_json
        }
      end
    else
      render :json => [{:error => "custom_failure"}], :status => 304
    end
  end

  private

  def postural_analysis_scan_params
    params.require(:postural_analysis_scan).permit(:description, :postural_analysis_id, :crop_x, :crop_y, :crop_w, :crop_h, :gallery_token, :scan => [])
  end

end
