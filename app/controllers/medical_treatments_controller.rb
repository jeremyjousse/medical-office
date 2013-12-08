class MedicalTreatmentsController < ApplicationController
  before_action :set_medical_treatment, only: [:show, :edit, :update, :destroy]

  # GET /medical_treatments
  # GET /medical_treatments.json
  def index
    #@medical_treatments = MedicalTreatment.all
    @medical_treatments = MedicalTreatment.order("date").where(user_id: current_user.id)
  end

  # GET /medical_treatments/1
  # GET /medical_treatments/1.json
  def show
  end

  # GET /medical_treatments/new
  def new
    @medical_treatment = MedicalTreatment.new
  end

  # GET /medical_treatments/1/edit
  def edit
  end

  # POST /medical_treatments
  # POST /medical_treatments.json
  def create
    @medical_treatment = MedicalTreatment.new(medical_treatment_params)

    @medical_treatment.user = current_user

    respond_to do |format|
      if @medical_treatment.save
        format.html { redirect_to @medical_treatment, notice: 'Medical treatment was successfully created.' }
        format.json { render action: 'show', status: :created, location: @medical_treatment }
      else
        format.html { render action: 'new' }
        format.json { render json: @medical_treatment.errors, status: :unprocessable_entity }
      end
    end
  end

  # PATCH/PUT /medical_treatments/1
  # PATCH/PUT /medical_treatments/1.json
  def update
    respond_to do |format|
      if @medical_treatment.update(medical_treatment_params)
        format.html { redirect_to @medical_treatment, notice: 'Medical treatment was successfully updated.' }
        format.json { head :no_content }
      else
        format.html { render action: 'edit' }
        format.json { render json: @medical_treatment.errors, status: :unprocessable_entity }
      end
    end
  end

  # DELETE /medical_treatments/1
  # DELETE /medical_treatments/1.json
  def destroy
    @medical_treatment.destroy
    respond_to do |format|
      format.html { redirect_to medical_treatments_url }
      format.json { head :no_content }
    end
  end

  private
    # Use callbacks to share common setup or constraints between actions.
    def set_medical_treatment
      @medical_treatment = MedicalTreatment.find(params[:id])
    end

    # Never trust parameters from the scary internet, only allow the white list through.
    def medical_treatment_params
      params.require(:medical_treatment).permit(:user_id, :patient_id, :date, :location_id, :treatment_type_id, :price, :note)
    end
end
