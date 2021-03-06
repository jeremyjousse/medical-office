class Patients::MedicalTreatmentsController < ApplicationController
  before_filter :authenticate_user!

  before_action :set_medical_treatment, only: [:show, :edit, :update, :destroy]

  before_filter :authorize_medical_treatment_access!, only: [:show, :edit, :update, :destroy]

  include ListingHelper

  def index
    search_params = put_and_get_search_params_in_session('medical_treatments',{search: params[:q], page: params[:page], per_page: params[:per_page]},params[:filter])

    @q = current_user.medical_treatments.order(date: :desc).paginate(:page => search_params[:page], :per_page => search_params[:per_page]).search(search_params[:search])
    @medical_treatments = @q.result(distinct: true)
    @total_items = current_user.medical_treatments.count
    @total_items_selected = @medical_treatments.count
  end

  def show
    @patient = current_user.patients.find(params[:patient_id])
    @medical_treatment = @patient.medical_treatments.find(params[:id])
  end

  def new
    @medical_treatment = MedicalTreatment.new
    @medical_treatment.patient_id = params[:patient_id]
    @medical_treatment.date = Date.today
  end

  def create
    @medical_treatment = MedicalTreatment.new(medical_treatment_params)

    @medical_treatment.user = current_user

    @medical_treatment.payments.each do |payment|
      payment.user = current_user
    end
    respond_to do |format|
      if @medical_treatment.save
        format.html { redirect_to @medical_treatment, notice: 'Medical treatment was successfully created.' }
        format.json { render action: 'show', status: :created, location: @medical_treatment }
        format.js { redirect_to(new_patient_medical_treatment_payment_path(patient_id: @medical_treatment.patient.id, medical_treatment_id: @medical_treatment.id), format: :js)}
      else
        format.html { render action: 'new' }
        format.json { render json: @medical_treatment.errors, status: :unprocessable_entity }
        format.js { render 'new', {patient_id: params[:patient_id]}}

      end
    end
  end

  def edit
    @medical_treatment = current_user.medical_treatments.find(params[:id])
  end

  def update
    respond_to do |format|
      if @medical_treatment.update(medical_treatment_params)

        @medical_treatment.payments.each do |payment|
          if payment.user_id.nil?
            payment.user = current_user
            payment.save
          end
        end
       flash[:notice] = 'Medical treatment was successfully created.'
       format.js { redirect_to(patient_medical_treatment_path(patien_id: @medical_treatment.patient_id, id: @medical_treatment.id), format: :js)}
      else
        format.js { render 'edit', {patient_id: params[:patient_id]}}
      end
    end
  end

  def destroy
    @medical_treatment.destroy
    respond_to do |format|
      format.html { redirect_to medical_treatments_url }
      format.json { head :no_content }
    end
  end

  def receipt
    respond_to do |format|
      format.msword { set_header('msword', 'yo.doc') }
    end
  end

  private

  def set_medical_treatment
    @medical_treatment = MedicalTreatment.find(params[:id])
  end

  def medical_treatment_params
    params.require(:medical_treatment).permit(:user_id, :patient_id, :date, :location_id, :medical_treatment_type_id, :price, :note)
  end

  def authorize_medical_treatment_access!
    return render_404 unless @medical_treatment.user_id == current_user.id
  end
end
