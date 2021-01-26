# frozen_string_literal: true

# PaymentsController is dispaying payments list and
# payments listing in order to be printed
class PaymentsController < ApplicationController
  before_filter :authenticate_user!

  before_action :set_payment, only: %i[show edit update destroy]

  include ListingHelper

  def index
    params[:per_page] = 10 if params[:per_page].nil?
    @search_params = put_and_get_search_params_in_session('payments', { 'search' => params[:q], 'page' => params[:page], 'per_page' => params[:per_page] }, params[:filter])

    @q = current_user.payments.order(paid_at: :desc).paginate(page: @search_params['page'], per_page: @search_params['per_page']).search(@search_params['search'])
    @payments = @q.result(distinct: true)
    @total_items = current_user.payments.all.count
    @total_items_selected = @payments.count
  end

  def listing
    params[:per_page] = 1000 if params[:per_page].nil?

    @search_params = put_and_get_search_params_in_session('listing_payments', { 'search' => params[:q], 'page' => params[:page], 'per_page' => params[:per_page] }, params[:filter])

    @q = current_user.payments.includes(:medical_treatment, medical_treatment: :patient, medical_treatment: :medical_treatment_type).search(@search_params['search'])
    @payments = current_user.payments.includes(medical_treatment: %i[patient medical_treatment_type]).paginate(page: @search_params['page'], per_page: @search_params['per_page']).search(@search_params['search']).result(distinct: true).order(:paid_at, :payment_type)
    @total_items = current_user.payments.all.count
    @total_items_selected = @payments.count
  end

  # GET /payments/1
  # GET /payments/1.json
  def show; end

  # GET /payments/new
  def new
    @payment = Payment.new

    unless params[:medical_treatment_id].nil?
      @payment.medical_treatment_id = params[:medical_treatment_id]

      @medical_treatment = MedicalTreatment.find(params[:medical_treatment_id])

      @payment.amount = @medical_treatment.price
    end

    @payment.payment_type = params[:payment_type].to_i unless params[:payment_type].nil?

    if @payment.payment_type.to_i == 1
      @payment.build_payment_bank_check
      @payment.payment_bank_check.account_owner = @medical_treatment.patient.last_name
    end

    @payment.paid_at = Date.today
  end

  # GET /payments/1/edit
  def edit; end

  # POST /payments
  # POST /payments.json
  def create
    @payment = Payment.new(payment_params)

    @payment.user = current_user

    unless @payment.payment_bank_check.nil?
      @payment.payment_bank_check.user = current_user
      @payment.payment_bank_check.amount = @payment.amount
      @payment.payment_bank_check.status = 0
    end

    respond_to do |format|
      if @payment.save
        format.html { redirect_to @payment, notice: 'Payment was successfully created.' }
        format.json { render action: 'show', status: :created, location: @payment }
        # format.js { redirect_to @payment.medical_treatment, notice: 'Payment was successfully created.' }
        format.js { render 'reload' }
      else
        format.html { render action: 'new' }
        format.json { render json: @payment.errors, status: :unprocessable_entity }
        format.js { render 'new', { medical_treatment_id: params[:medical_treatment_id], payment_type: params[:payment_payment_type] } }
      end
    end
  end

  # PATCH/PUT /payments/1
  # PATCH/PUT /payments/1.json
  def update
    respond_to do |format|
      if @payment.update(payment_params)
        format.html { redirect_to @payment, notice: 'Payment was successfully updated.' }
        format.json { head :no_content }
      else
        format.html { render action: 'edit' }
        format.json { render json: @payment.errors, status: :unprocessable_entity }
      end
    end
  end

  # DELETE /payments/1
  # DELETE /payments/1.json
  def destroy
    destroyed = false

    destroyed = true if @payment.safe_destroy == true

    respond_to do |format|
      format.html { redirect_to payments_url }
      format.json { head :no_content }
    end
  end

  private

  # Use callbacks to share common setup or constraints between actions.
  def set_payment
    @payment = Payment.find(params[:id])
  end

  # Never trust parameters from the scary internet, only allow the white list through.
  def payment_params
    params.require(:payment).permit(:medical_treatment_id, :paid_at, :payment_type, :amount, payment_bank_check_attributes: %i[id bank_name account_owner check_number])
  end
end
