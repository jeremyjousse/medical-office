class PosturalAnalysisScan < ActiveRecord::Base

  belongs_to :user

  mount_uploader :scan, PosturalAnalysisScanUploader
  belongs_to :postural_analysis_scans


  def to_jq_upload
  {
    "name" => read_attribute(:scan),
    "size" => scan.size,
    "url" => scan.url,
    "thumbnail_url" => scan.thumb.url,
    "delete_url" => id,
    "picture_id" => id,
    "delete_type" => "DELETE"
  }
  end

  def crop_image
    scan.recreate_versions! if crop_x.present?
    current_version = self.scan.current_path
    large_version = "#{Rails.root}/public" + self.scan.versions[:large].to_s

    FileUtils.rm(current_version)
    FileUtils.cp(large_version, current_version)
  end

end
