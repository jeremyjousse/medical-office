class PaymentBankChecksController < ApplicationController
  
  before_filter :authenticate_user!

  before_action :set_payment_bank_check, only: [:show, :edit, :update, :destroy]

  # GET /payment_bank_checks
  # GET /payment_bank_checks.json
  def index
    @payment_bank_checks = PaymentBankCheck.all
  end

  # GET /payment_bank_checks/1
  # GET /payment_bank_checks/1.json
  def show
  end

  # GET /payment_bank_checks/new
  def new
    @payment_bank_check = PaymentBankCheck.new
  end

  # GET /payment_bank_checks/1/edit
  def edit
  end

  # POST /payment_bank_checks
  # POST /payment_bank_checks.json
  def create
    @payment_bank_check = PaymentBankCheck.new(payment_bank_check_params)

    respond_to do |format|
      if @payment_bank_check.save
        format.html { redirect_to @payment_bank_check, notice: 'Payment bank check was successfully created.' }
        format.json { render action: 'show', status: :created, location: @payment_bank_check }
      else
        format.html { render action: 'new' }
        format.json { render json: @payment_bank_check.errors, status: :unprocessable_entity }
      end
    end
  end

  # PATCH/PUT /payment_bank_checks/1
  # PATCH/PUT /payment_bank_checks/1.json
  def update
    respond_to do |format|
      if @payment_bank_check.update(payment_bank_check_params)
        format.html { redirect_to @payment_bank_check, notice: 'Payment bank check was successfully updated.' }
        format.json { head :no_content }
      else
        format.html { render action: 'edit' }
        format.json { render json: @payment_bank_check.errors, status: :unprocessable_entity }
      end
    end
  end

  # DELETE /payment_bank_checks/1
  # DELETE /payment_bank_checks/1.json
  def destroy
    @payment_bank_check.destroy
    respond_to do |format|
      format.html { redirect_to payment_bank_checks_url }
      format.json { head :no_content }
    end
  end

  private
    # Use callbacks to share common setup or constraints between actions.
    def set_payment_bank_check
      @payment_bank_check = PaymentBankCheck.find(params[:id])
    end

    # Never trust parameters from the scary internet, only allow the white list through.
    def payment_bank_check_params
      params.require(:payment_bank_check).permit(:account_owner, :bank_name, :check_number, :amount, :status, :check_deposit_id, :paiment_id)
    end
end
