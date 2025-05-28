use image::{DynamicImage, ImageBuffer, Rgba};
use rc_basis::anyhow;
use rc_basis::colors::RgbaColor;

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/05/27
///

/// 使用指定宽高和颜色创建图片
/// [color] RGBA颜色值
pub fn create_image<Color: RgbaColor>(
    width: u32,
    height: u32,
    color: Color,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    ImageBuffer::from_pixel(
        width,
        height,
        Rgba([color.r(), color.g(), color.b(), color.a()]),
    )
}

/// 将图片保存到文件
pub fn save_image(
    image: &DynamicImage,
    file_path: &str,
    format: Option<image::ImageFormat>,
) -> anyhow::Result<()> {
    match format {
        Some(format) => Ok(image.save_with_format(file_path, format)?),
        None => Ok(image.save(file_path)?),
    }
    /*.map_err(|e| format!("保存图片失败: {}", e))*/
}

pub fn save_image_buffer(
    image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    file_path: &str,
    format: Option<image::ImageFormat>,
) -> anyhow::Result<()> {
    match format {
        Some(format) => Ok(image.save_with_format(file_path, format)?),
        None => Ok(image.save(file_path)?),
    }
    /*.map_err(|e| format!("保存图片失败: {}", e))*/
}
