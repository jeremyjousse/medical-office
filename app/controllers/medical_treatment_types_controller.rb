class MedicalTreatmentTypesController < ApplicationController

  before_filter :authenticate_user!

  before_action :set_medical_treatment_type, only: [:show, :edit, :update, :destroy]

  before_filter :authorize_medical_treatment_type_access!, only: [:show, :edit, :update, :destroy]


  def index
    @q = current_user.medical_treatment_types.paginate(:page => params[:page], :per_page => 8).search(params[:q])
    @medical_treatment_types = @q.result(distinct: true)
    @total_items = current_user.medical_treatment_types.count
    @total_items_selected = @medical_treatment_types.count
  end

  
  def price_finder
    @medical_treatment_type = current_user.medical_treatment_types.find(params[:id])
    respond_to do |format|
      format.json { render json: @medical_treatment_type }
    end
  end


  def show
  end


  def new
    @medical_treatment_type = MedicalTreatmentType.new
  end


  def edit
  end


  def create
    @medical_treatment_type = MedicalTreatmentType.new(medical_treatment_type_params)
    @medical_treatment_type.user = current_user

    respond_to do |format|
      if @medical_treatment_type.save
        format.html { redirect_to medical_treatment_types_path, notice: 'Medical treatment type was successfully created.' }
      else
        format.html { render action: 'new' }
      end
    end
  end


  def update
    respond_to do |format|
      if @medical_treatment_type.update(medical_treatment_type_params)
        format.html { redirect_to @medical_treatment_type, notice: 'Medical treatment type was successfully updated.' }
      else
        format.html { render action: 'edit' }
      end
    end
  end


  def destroy
    @medical_treatment_type.destroy
    respond_to do |format|
      format.html { redirect_to medical_treatment_types_url }
    end
  end

  private  
    def set_medical_treatment_type
      @medical_treatment_type = MedicalTreatmentType.find(params[:id])
    end

    # Never trust parameters from the scary internet, only allow the white list through.
    def medical_treatment_type_params
      params.require(:medical_treatment_type).permit(:name, :price, :user_id)
    end

    def authorize_medical_treatment_type_access!
      return render_404 unless @medical_treatment_type.user_id == current_user.id
    end
end
