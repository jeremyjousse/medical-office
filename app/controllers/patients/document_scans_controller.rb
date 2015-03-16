class Patients::DocumentScansController < ApplicationController

  def new
    @patient = current_user.patients.find(params[:patient_id])
    @document_scan = DocumentScan.new
    @document_scan.patient = @patient
    # @document_scan.patient_id = params[:patient_id]
  end

  def create
    @patient = current_user.patients.find(params[:patient_id])

    @document_scan = DocumentScan.new(document_scan_params)

    @document_scan.user = current_user



    @document_scan.patient = @patient

    if params[:document_scan][:scan].class == Array
      @document_scan.file = params[:document_scan][:scan].first
    else
      @document_scan.file = params[:document_scan][:scan]
    end

    respond_to do |format|
      if @document_scan.save
        format.html { redirect_to @patient, notice: 'Document scan was successfully created.' }
        format.js { render 'reload' }
      else
        format.js { render 'new', {patient_id: params[:patient_id]}}
      end
    end
  end

  def show
    @patient = current_user.patients.find(params[:patient_id])
    @document_scan = @patient.document_scans.find(params[:id])
  end

  private

  def document_scan_params
    params.require(:document_scan).permit(:patient_id, :title, :scan => [])
  end

end
