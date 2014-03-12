class PaymentsController < ApplicationController

  before_filter :authenticate_user!

  before_action :set_payment, only: [:show, :edit, :update, :destroy]



  # GET /payments
  # GET /payments.json
  def index
    @payments = Payment.order("created_at").where(user_id: current_user.id)
  end

  # GET /payments/1
  # GET /payments/1.json
  def show
  end

  # GET /payments/new
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
    end

    @payment.paid_at = Date.today

  end

  # GET /payments/1/edit
  def edit
  end

  # POST /payments
  # POST /payments.json
  def create
    @payment = Payment.new(payment_params)
    
    @payment.user = current_user

    if !@payment.payment_bank_check.nil? then
      @payment.payment_bank_check.user = current_user
      @payment.payment_bank_check.amount = @payment.amount
      @payment.payment_bank_check.status = 0
    end

    respond_to do |format|
      if @payment.save
        format.html { redirect_to @payment, notice: 'Payment was successfully created.' }
        format.json { render action: 'show', status: :created, location: @payment }
        #format.js { redirect_to @payment.medical_treatment, notice: 'Payment was successfully created.' }
        format.js { render 'reload' }
      else
        format.html { render action: 'new' }
        format.json { render json: @payment.errors, status: :unprocessable_entity }
        format.js { render 'new', {medical_treatment_id: params[:medical_treatment_id], payment_type: params[:payment_payment_type]}}
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
    @payment.destroy
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
      params.require(:payment).permit(:medical_treatment_id, :paid_at, :payment_type, :amount, payment_bank_check_attributes: [:id, :bank_name, :account_owner, :check_number])
    end



end
