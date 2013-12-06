class MedicalTreatmentTypesController < ApplicationController
  before_action :set_medical_treatment_type, only: [:show, :edit, :update, :destroy]

  # GET /medical_treatment_types
  # GET /medical_treatment_types.json
  def index
    @medical_treatment_types = MedicalTreatmentType.all
  end

  # GET /medical_treatment_types/1
  # GET /medical_treatment_types/1.json
  def show
  end

  # GET /medical_treatment_types/new
  def new
    @medical_treatment_type = MedicalTreatmentType.new
  end

  # GET /medical_treatment_types/1/edit
  def edit
  end

  # POST /medical_treatment_types
  # POST /medical_treatment_types.json
  def create
    @medical_treatment_type = MedicalTreatmentType.new(medical_treatment_type_params)

    @medical_treatment_type.user = current_user


    respond_to do |format|
      if @medical_treatment_type.save
        format.html { redirect_to @medical_treatment_type, notice: 'Medical treatment type was successfully created.' }
        format.json { render action: 'show', status: :created, location: @medical_treatment_type }
      else
        format.html { render action: 'new' }
        format.json { render json: @medical_treatment_type.errors, status: :unprocessable_entity }
      end
    end
  end

  # PATCH/PUT /medical_treatment_types/1
  # PATCH/PUT /medical_treatment_types/1.json
  def update
    respond_to do |format|
      if @medical_treatment_type.update(medical_treatment_type_params)
        format.html { redirect_to @medical_treatment_type, notice: 'Medical treatment type was successfully updated.' }
        format.json { head :no_content }
      else
        format.html { render action: 'edit' }
        format.json { render json: @medical_treatment_type.errors, status: :unprocessable_entity }
      end
    end
  end

  # DELETE /medical_treatment_types/1
  # DELETE /medical_treatment_types/1.json
  def destroy
    @medical_treatment_type.destroy
    respond_to do |format|
      format.html { redirect_to medical_treatment_types_url }
      format.json { head :no_content }
    end
  end

  private
    # Use callbacks to share common setup or constraints between actions.
    def set_medical_treatment_type
      @medical_treatment_type = MedicalTreatmentType.find(params[:id])
    end

    # Never trust parameters from the scary internet, only allow the white list through.
    def medical_treatment_type_params
      params.require(:medical_treatment_type).permit(:name, :price, :user_id)
    end
end
