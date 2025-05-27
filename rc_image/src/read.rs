use image::{DynamicImage, ImageError, ImageReader};

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/05/27
///

/// 从文件中读取图片数据
pub fn read_image_file(image_file_path: &str) -> Result<DynamicImage, ImageError> {
    ImageReader::open(image_file_path)?.decode()
}

/// 从字节数据中读取图片
pub fn read_image_bytes(image_bytes: &[u8]) -> Result<DynamicImage, ImageError> {
    image::load_from_memory(image_bytes)
    //ImageReader::new(image_bytes)?.decode()
}
