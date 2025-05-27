use crate::read::{read_image_bytes, read_image_file};
use image::{DynamicImage, EncodableLayout, ImageBuffer, ImageError, Pixel};
use std::ops::Deref;

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/05/27
///

/// 调整图片大小
pub fn resize_image(image: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    image.resize(width, height, image::imageops::FilterType::Nearest)
}

/// 调整图片格式或大小
pub fn convert_image_format(
    image: &DynamicImage,
    format: image::ImageFormat,
    size: Option<(u32, u32)>,
) -> rc_basis::anyhow::Result<DynamicImage> {
    let image: &DynamicImage = if let Some(s) = size {
        &resize_image(image, s.0, s.1)
    } else {
        image
    };

    let mut buffer = std::io::Cursor::new(Vec::new());
    image.write_to(&mut buffer, format)?;

    Ok(read_image_bytes(&buffer.into_inner())?)
}

/// 将图片对象转换成base64字符串数据
pub fn image_to_base64(img: &DynamicImage) -> rc_basis::anyhow::Result<String> {
    // 直接写入 Vec<u8>
    //let mut buffer = Vec::new();
    let mut buffer = std::io::Cursor::new(Vec::new());
    img.write_to(&mut buffer, image::ImageFormat::Png)?;

    // 编码为 Base64
    let base64_string = rc_basis::bytes::base64_encode(&buffer.into_inner());
    Ok(base64_string)
}
pub fn image_buffer_to_base64<P: Pixel + image::PixelWithColorType, Container>(
    img_buffer: &ImageBuffer<P, Container>,
) -> rc_basis::anyhow::Result<String>
where
    P: Pixel,
    [P::Subpixel]: EncodableLayout,
    Container: Deref<Target = [P::Subpixel]>,
{
    let mut buffer = std::io::Cursor::new(Vec::new());
    img_buffer.write_to(&mut buffer, image::ImageFormat::Png)?;

    // 编码为 Base64
    let base64_string = rc_basis::bytes::base64_encode(&buffer.into_inner());
    Ok(base64_string)
}

/// 调整指定路径图片的大小, 并输出
pub fn resize_image_file(
    image_file_path: &str,
    width: u32,
    height: u32,
    output_file_path: &str,
) -> Result<(), ImageError> {
    if let Ok(image) = read_image_file(image_file_path) {
        let resized_image = resize_image(&image, width, height);
        resized_image.save(output_file_path)
    } else {
        Err(ImageError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to read image",
        )))
    }
}

/// 从文件中读取图片,并输出对应的base64协议图片数据
pub fn read_image_file_to_base64(image_file_path: &str) -> rc_basis::anyhow::Result<String> {
    if let Ok(image) = read_image_file(image_file_path) {
        //let image_bytes = image.into_bytes(); //颜色字节数组
        //let base64_data = crate::utils::base64_encode(&image_bytes);
        //Ok(format!("data:image/jpeg;base64,{base64_data}"))
        let base64_data = image_to_base64(&image)?;
        Ok(format!("data:image/png;base64,{base64_data}"))
    } else {
        Err(rc_basis::anyhow::anyhow!("Failed to read image"))
    }
}

///将base64图片转换成图片
pub fn base64_to_image(base64_data: &str) -> rc_basis::anyhow::Result<DynamicImage> {
    let base64_data = if base64_data.contains(",") {
        base64_data.split(',').last().unwrap()
    } else {
        base64_data
    };
    let bytes = rc_basis::bytes::base64_decode(base64_data)?;
    Ok(read_image_bytes(bytes.as_slice())?)
}
