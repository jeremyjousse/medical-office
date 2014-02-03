class MedicalTreatmentsController < ApplicationController
  
  before_filter :authenticate_user!

  before_action :set_medical_treatment, only: [:show, :edit, :update, :destroy]
  
  before_filter :authorize_medical_treatment_access!, only: [:show, :edit, :update, :destroy]

  def index
    @q = current_user.medical_treatments.paginate(:page => params[:page], :per_page => 8).search(params[:q])
    @medical_treatments = @q.result(distinct: true)
    @total_items = current_user.medical_treatments.find(:all).count
    @total_items_selected = @medical_treatments.count
  end


  def show
  end



  def new
    @medical_treatment = MedicalTreatment.new
    @medical_treatment.patient_id = params[:patient_id]
  end


  def edit
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
