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
/// [format] 如果不指定格式, 会按照文件后缀, 保存对应的像素格式, 如果格式不匹配会报错.
/// [save_image]
/// [write_image_file]
pub fn save_image(
    image: &DynamicImage,
    file_path: &str,
    format: Option<image::ImageFormat>,
) -> anyhow::Result<()> {
    rc_basis::files::ensure_parent_dir_exist(file_path);
    match format {
        Some(format) => Ok(image.save_with_format(file_path, format)?),
        None => Ok(image.save(file_path)?),
    }
    /*.map_err(|e| format!("保存图片失败: {}", e))*/
}

/// [save_image]
/// [write_image_file]
pub fn write_image_file(
    image: &DynamicImage,
    file_path: &str,
    format: Option<image::ImageFormat>,
) -> anyhow::Result<()> {
    save_image(image, file_path, format)
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
