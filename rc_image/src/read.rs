use image::{
    DynamicImage, EncodableLayout, ImageBuffer, ImageError, ImageFormat, ImageReader, Pixel,
};
use std::ops::Deref;

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/05/27
///

/// 从文件中读取图片数据
/// 会解析文件后缀, 识别对应的像素格式, 如果格式不匹配会报错.
pub fn read_image_file(image_file_path: &str) -> Result<DynamicImage, ImageError> {
    ImageReader::open(image_file_path)?.decode()
}

/// 从字节数据中读取图片
pub fn read_image_bytes(image_bytes: &[u8]) -> Result<DynamicImage, ImageError> {
    image::load_from_memory(image_bytes)
    //ImageReader::new(image_bytes)?.decode()
}

/// 从图片缓冲区中读取图片, 需要指定图片格式
pub fn read_image_buffer<P: Pixel + image::PixelWithColorType, Container>(
    img_buffer: &ImageBuffer<P, Container>,
    format: ImageFormat,
) -> Result<DynamicImage, ImageError>
where
    P: Pixel,
    [P::Subpixel]: EncodableLayout,
    Container: Deref<Target = [P::Subpixel]>,
{
    let mut buffer = std::io::Cursor::new(Vec::new());
    img_buffer.write_to(&mut buffer, format)?;
    read_image_bytes(&buffer.into_inner())
}
