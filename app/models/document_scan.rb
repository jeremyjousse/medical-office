class DocumentScan < ActiveRecord::Base

  belongs_to :user
  belongs_to :patient

  mount_uploader :file, DocumentScanUploader

  def to_jq_upload
  {
    "name" => read_attribute(:file),
    "size" => file.size,
    "url" => file.url,
    "thumbnail_url" => file.thumb.url,
    "delete_url" => id,
    "picture_id" => id,
    "delete_type" => "DELETE"
  }
  end

  def crop_image
    scan.recreate_versions! if crop_x.present?
    current_version = self.file.current_path
    large_version = "#{Rails.root}/public" + self.file.versions[:large].to_s

    FileUtils.rm(current_version)
    FileUtils.cp(large_version, current_version)
  end

end
