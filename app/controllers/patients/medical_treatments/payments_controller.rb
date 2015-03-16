class Patients::MedicalTreatments::PaymentsController < ApplicationController
  before_filter :authenticate_user!

  # before_action :set_payment, only: [:show, :edit, :update, :destroy]

  #before_filter :authorize_payment_access!, only: [:show, :edit, :update, :destroy]

  include ListingHelper

  def new
    @payment = Payment.new

    if !params[:medical_treatment_id].nil? then
      @payment.medical_treatment_id = params[:medical_treatment_id]

      @medical_treatment = MedicalTreatment.find(params[:medical_treatment_id])

      @payment.amount = @medical_treatment.price
    end

    if !params[:payment_type].nil? then
      @payment.payment_type = params[:payment_type].to_i
    end

    if @payment.payment_type.to_i == 1 then
      @payment.build_payment_bank_check
      @payment.payment_bank_check.account_owner = @medical_treatment.patient.last_name
    end

    @payment.paid_at = Date.today
  end

  def create
    if !params[:medical_treatment_id].nil? then
      @medical_treatment = current_user.medical_treatments.find(params[:medical_treatment_id])
    end

    @payment = Payment.new(payment_params)

    @payment.user = current_user

    if !@payment.payment_bank_check.nil? then
      @payment.payment_bank_check.user = current_user
      @payment.payment_bank_check.amount = @payment.amount
      @payment.payment_bank_check.status = 0
    end

    respond_to do |format|
      if @payment.save
        flash[:notice] = 'Payement added'
        format.js { redirect_to(patient_medical_treatment_path(patient_id: @medical_treatment.patient.id, id: @medical_treatment.id, locale: I18n.locale), format: :js)}
      else
        format.js { render 'new', {medical_treatment_id: params[:medical_treatment_id]}}
      end
    end
  end

  def edit
    @payment = current_user.payments.find(params[:id])
    @medical_treatment = current_user.medical_treatments.find(@payment.medical_treatment.id)
  end

  def update
    @payment = current_user.payments.find(params[:id])

    logger.info '@payment.medical_treatment_id' + @payment.medical_treatment_id.to_s + ' @payment.medical_treatment.patient_id ' + @payment.medical_treatment.patient_id.to_s
    respond_to do |format|
      if @payment.update(payment_params)
        flash[:notice] = 'Payment was successfully updated.'
        format.js { redirect_to(patient_medical_treatment_path(patient_id: @payment.medical_treatment.patient.id, id: @payment.medical_treatment.id, locale: I18n.locale), format: :js)}
      else
        format.js { render 'edit', {patient_id: params[:patient_id], medical_treatment_id: @payment.medical_treatment.id, id: @payment.id}}
      end
    end
  end

  def destroy
    @payment = current_user.payments.find(params[:id])

    if @payment.safe_destroy == true

    end

    @patient = @payment.medical_treatment.patient
    @medical_treatment = @payment.medical_treatment
    respond_to do |format|

      format.js { render 'patients/medical_treatments/show' }
    end
  end

private

  def payment_params
    params.require(:payment).permit(:medical_treatment_id, :paid_at, :payment_type, :amount, payment_bank_check_attributes: [:id, :bank_name, :account_owner, :check_number])
  end
end
