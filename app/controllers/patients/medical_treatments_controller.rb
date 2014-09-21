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
  end



  def new
    @medical_treatment = MedicalTreatment.new
    @medical_treatment.patient_id = params[:patient_id]
    @medical_treatment.date = Date.today



  end


  def edit
  end


  def create
    @medical_treatment = MedicalTreatment.new(medical_treatment_params)

    @medical_treatment.user = current_user

    @medical_treatment.payments.each do |payment|
      payment.user = current_user
    end


#logger.info "------" + request.format.inspect

    respond_to do |format|
      if @medical_treatment.save
        format.html { redirect_to @medical_treatment, notice: 'Medical treatment was successfully created.' }
        format.json { render action: 'show', status: :created, location: @medical_treatment }
        #format.js { redirect_to 'payments/new', {medical_treatment_id: @medical_treatment.id}}

        #format.js { redirect_to(:format => :js, :params => {medical_treatment_id: @medical_treatment.id})}
        format.js { redirect_to(new_payment_path(medical_treatment_id: @medical_treatment.id), format: :js)}
      else
        format.html { render action: 'new' }
        format.json { render json: @medical_treatment.errors, status: :unprocessable_entity }
        format.js { render 'new', {patient_id: params[:patient_id]}}

      end
    end
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

        format.html { redirect_to @medical_treatment, notice: 'Medical treatment was successfully updated.' }
      else
        format.html { render action: 'edit' }
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


  private
    # Use callbacks to share common setup or constraints between actions.
    def set_medical_treatment
      @medical_treatment = MedicalTreatment.find(params[:id])
    end

    # Never trust parameters from the scary internet, only allow the white list through.
    def medical_treatment_params
      params.require(:medical_treatment).permit(:user_id, :patient_id, :date, :location_id, :medical_treatment_type_id, :price, :note, payments_attributes: [:id, :amount, :paid_at, :payment_type])
    end

    def authorize_medical_treatment_access!
      return render_404 unless @medical_treatment.user_id == current_user.id
    end
end
