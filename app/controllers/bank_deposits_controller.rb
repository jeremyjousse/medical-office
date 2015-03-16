class BankDepositsController < ApplicationController
  before_action :set_bank_deposit, only: [:show, :edit, :update, :destroy]


  def index
    #@bank_deposits = BankDeposit.all
    @q = current_user.bank_deposits.order(created_at: :desc).paginate(:page => params[:page], :per_page => 10).search(params[:q])
    @bank_deposits = @q.result(distinct: true)
    @total_items = current_user.bank_deposits.all().count
    @total_items_selected = @bank_deposits.count
  end


  def show
    #@bank_deposit = current_user.bank_deposits.where('bank_deposits.id = ?',params[:id]).includes(:payments, :user, :bank_account ).includes('JOIN payment_bank_checks ON payments.id = payment_bank_checks.payment_id').select("users.*, bank_accounts.*, payment_bank_checks.*, payments.paid_at, bank_deposits.*").first
    @bank_deposit = current_user.bank_deposits.where('bank_deposits.id = ?',params[:id]).eager_load( :user, :bank_account ).first

    @payment_bank_checks = @bank_deposit.payment_bank_checks.joins('INNER JOIN payments ON payments.id = payment_bank_checks.payment_id').order("payments.paid_at ASC")

  end

  # GET /bank_deposits/new
  def new
    redirect_to bank_deposits_path, notice: 'You must chose a bank deposit type.' unless BankDeposit::DEPOSIT_TYPES.has_key?(params[:deposit_type].to_i)
    @bank_deposit = BankDeposit.new
    @bank_deposit.deposit_type = params[:deposit_type].to_i
    if @bank_deposit.deposit_type == 1
      @pending_bank_checks = PaymentBankCheck.pending.order('payments.paid_at')
      @checked_bank_checks = []
      @pending_bank_checks.each do |pbc|
        @checked_bank_checks << pbc.id
      end
    end
  end

  # GET /bank_deposits/1/edit
  def edit
  end

  # POST /bank_deposits
  # POST /bank_deposits.json
  def create
    @bank_deposit = BankDeposit.new

    @bank_deposit.user = current_user
    @bank_deposit.deposit_date = Date.today

    payment_bank_checks = {}

    redirect_to bank_deposits_path, notice: 'select a type.' and return unless !params[:bank_deposit][:deposit_type].nil? && params[:bank_deposit][:type] != 1

    @bank_deposit.deposit_type = params[:bank_deposit][:deposit_type]

    if params[:bank_deposit][:payment_bank_check_ids].kind_of?(Array) != true
      redirect_to bank_deposits_path, notice: 'Select bank check.' and return
    end

    bank_account = current_user.bank_accounts.find(params[:bank_deposit][:bank_account_id])

    if bank_account.bank_check_deposit_number.nil?
      redirect_to bank_deposits_path, notice: 'Select bank.' and return
    end

    @bank_deposit.number = bank_account.bank_check_deposit_number + 1
    @bank_deposit.bank_account_id = bank_account.id

    params[:bank_deposit][:payment_bank_check_ids].each do |payment_bank_check_id|
      next if payment_bank_check_id.to_i < 1
      payment_bank_check = current_user.payment_bank_checks.find(payment_bank_check_id)
      if !payment_bank_check.nil?
        payment_bank_checks[payment_bank_check_id] = payment_bank_check
        @bank_deposit.amount = @bank_deposit.amount.to_i + payment_bank_check.amount

      end
    end

    respond_to do |format|
      if @bank_deposit.save
        payment_bank_checks.each do |payment_bank_check_id, payment_bank_check|

          payment_bank_check.status = 1
          payment_bank_check.bank_deposit_id = @bank_deposit.id
          payment_bank_check.save

          payment = current_user.payments.find(payment_bank_check.payment_id)
          payment.bank_deposit_id = @bank_deposit.id
          payment.save
        end

        bank_account.bank_check_deposit_number = bank_account.bank_check_deposit_number + 1
        bank_account.save

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
    redirect_to bank_deposits_path, notice: 'Error' and return if current_user.bank_deposits.find(@bank_deposit.id).nil?

    current_user.payment_bank_checks.where(bank_deposit_id: @bank_deposit.id).each do |payment_bank_check|
      payment_bank_check.status = 0
      payment_bank_check.bank_deposit_id = nil
      payment_bank_check.save
    end

    current_user.payments.where(bank_deposit_id: @bank_deposit.id).each do |payment|
      payment.bank_deposit_id = nil
      payment.save
    end

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
      params.require(:bank_deposit).permit(:deposit_type, :bank_account_id, :amount)
    end

    def new_bank_deposit_params
      params.require(:bank_deposit).permit(:deposit_type)
    end
end
