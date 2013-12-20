class PatientsController < ApplicationController
  
  before_filter :authenticate_user!

  before_action :set_patient, only: [:show, :edit, :update, :destroy]

  def index
    #@patients = Patient.all
    #@patients = Patient.order("last_name","first_name").where(user_id: current_user.id)
    @q = Patient.paginate(:page => params[:page], :per_page => 10).search(params[:q])
    @patients = @q.result(distinct: true)
    @total_items = Patient.find(:all).count
    @total_items_selected = @patients.count
  end

  
  def show
  end

  
  def new
    @patient = Patient.new
  end

  
  def edit
  end

  
  def create
    @patient = Patient.new(patient_params)

    @patient.user = current_user

    respond_to do |format|
      if @patient.save
        format.html { redirect_to @patient, notice: 'Patient was successfully created.' }
        format.json { render action: 'show', status: :created, location: @patient }
      else
        format.html { render action: 'new' }
        format.json { render json: @patient.errors, status: :unprocessable_entity }
      end
    end
  end

  
  def update
    respond_to do |format|
      if @patient.update(patient_params)
        format.html { redirect_to @patient, notice: 'Patient was successfully updated.' }
        format.json { head :no_content }
      else
        format.html { render action: 'edit' }
        format.json { render json: @patient.errors, status: :unprocessable_entity }
      end
    end
  end

  
  def destroy
    @patient.destroy
    respond_to do |format|
      format.html { redirect_to patients_url }
      format.json { head :no_content }
    end
  end

  private
    def set_patient
      @patient = Patient.find(params[:id])
    end

    
    def patient_params
      params.require(:patient).permit(:first_name, :last_name, :birthdate, :profession, :phone, :mobile_phone, :email, :address, :postal_code, :city, :country_id, :national_insurance_number, :family_doctor_id, :physical_therapists_id, :physical_therapists_id, :osteopath_id, :legal_guardian_id, :note, :diabetic, :universal_healthcare_coverage, :long_duration_disease, :diabete_network_id)
    end
end
