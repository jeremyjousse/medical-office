class BankDepositsController < ApplicationController
  before_action :set_bank_deposit, only: [:show, :edit, :update, :destroy]

  # GET /bank_deposits
  # GET /bank_deposits.json
  def index
    @bank_deposits = BankDeposit.all
  end

  # GET /bank_deposits/1
  # GET /bank_deposits/1.json
  def show
  end

  # GET /bank_deposits/new
  def new
    @bank_deposit = BankDeposit.new
  end

  # GET /bank_deposits/1/edit
  def edit
  end

  # POST /bank_deposits
  # POST /bank_deposits.json
  def create
    @bank_deposit = BankDeposit.new(bank_deposit_params)

    respond_to do |format|
      if @bank_deposit.save
        format.html { redirect_to @bank_deposit, notice: 'Bank deposit was successfully created.' }
        format.json { render action: 'show', status: :created, location: @bank_deposit }
      else
        format.html { render action: 'new' }
        format.json { render json: @bank_deposit.errors, status: :unprocessable_entity }
      end
    end
  end

  # PATCH/PUT /bank_deposits/1
  # PATCH/PUT /bank_deposits/1.json
  def update
    respond_to do |format|
      if @bank_deposit.update(bank_deposit_params)
        format.html { redirect_to @bank_deposit, notice: 'Bank deposit was successfully updated.' }
        format.json { head :no_content }
      else
        format.html { render action: 'edit' }
        format.json { render json: @bank_deposit.errors, status: :unprocessable_entity }
      end
    end
  end

  # DELETE /bank_deposits/1
  # DELETE /bank_deposits/1.json
  def destroy
    @bank_deposit.destroy
    respond_to do |format|
      format.html { redirect_to bank_deposits_url }
      format.json { head :no_content }
    end
  end

  private
    # Use callbacks to share common setup or constraints between actions.
    def set_bank_deposit
      @bank_deposit = BankDeposit.find(params[:id])
    end

    # Never trust parameters from the scary internet, only allow the white list through.
    def bank_deposit_params
      params.require(:bank_deposit).permit(:type, :number, :amount)
    end
end
