class PractitionersController < ApplicationController
  
  before_filter :authenticate_user!

  before_action :set_practitioner, only: [:show, :edit, :update, :destroy]

  before_filter :authorize_modify_practitioner!, only: [:edit, :update]

  def index
    @q = current_user.practitioners.paginate(:page => params[:page], :per_page => 8).search(params[:q], :user_id => current_user.id)
    @practitioners = @q.result(distinct: true)
    @total_items = current_user.practitioners.count
    @total_items_selected = @practitioners.count
  end


  def finder
    #params[:per] = 10 unless !params[:per].nil?
    params[:per] = 10
    @practitioners = Practitioner.order('last_name').finder(params[:q]).page(params[:page])
    respond_to do |format|
      format.json { render json: @practitioners }
    end
  end

  # GET /practitioners/1
  def show
  end

  # GET /practitioners/new
  def new
    @practitioner = Practitioner.new
  end

  # GET /practitioners/1/edit
  def edit
  end

  # POST /practitioners
  def create
    @practitioner = Practitioner.new(practitioner_params)

    @practitioner.user = current_user

    respond_to do |format|
      if @practitioner.save
        format.html { redirect_to practitioners_path, notice: 'Practitioner was successfully created.' }
        #format.json { render action: 'show', status: :created, location: @practitioner }
      else
        format.html { render action: 'new' }
        #format.json { render json: @practitioner.errors, status: :unprocessable_entity }
      end
    end
  end

  # PATCH/PUT /practitioners/1
  # PATCH/PUT /practitioners/1.json
  def update



    respond_to do |format|
      if @practitioner.update(practitioner_params)
        format.html { redirect_to @practitioner, notice: 'Practitioner was successfully updated.' }
        #format.json { head :no_content }
      else
        format.html { render action: 'edit' }
        #format.json { render json: @practitioner.errors, status: :unprocessable_entity }
      end
    end
  end

  # DELETE /practitioners/1
  # DELETE /practitioners/1.json
  def destroy
    @practitioner.destroy
    respond_to do |format|
      format.html { redirect_to practitioners_url }
      #format.json { head :no_content }
    end
  end

  protected
    # Use callbacks to share common setup or constraints between actions.
    def set_practitioner
      @practitioner = Practitioner.find(params[:id])
    end

    # Never trust parameters from the scary internet, only allow the white list through.
    def practitioner_params
      params.require(:practitioner).permit(:first_name, :last_name, :phone, :mobile_phone, :email, :address, :postal_code, :city, :country_id, :note, :speciality_id)
    end

    def authorize_modify_practitioner!
      return render_404 unless @practitioner.user_id == current_user.id
    end
end
