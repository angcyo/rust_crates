use crate::read::{read_image_bytes, read_image_file};
use image::{
    DynamicImage, EncodableLayout, GenericImageView, ImageBuffer, ImageError, Pixel, Rgba,
    RgbaImage,
};
use nalgebra::{Matrix3, Vector3};
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

/// rgba像素转换成png图片字节数据
/// [read_image_bytes]
/// [image::load_from_memory]
pub fn rgba_to_png_bytes(
    rgba: &Vec<u8>,
    width: u32,
    height: u32,
    format: Option<image::ImageFormat>,
) -> rc_basis::anyhow::Result<Vec<u8>> {
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, rgba.clone()).unwrap();
    let mut buffer = std::io::Cursor::new(Vec::new());
    image.write_to(
        &mut buffer,
        format.unwrap_or_else(|| image::ImageFormat::Png),
    )?;
    //Ok(read_image_bytes(&buffer.into_inner())?)
    Ok(buffer.into_inner())
}

/// 旋转图片
///
/// - [angle] 旋转角度, 角度制
pub fn rotate_image(src: &DynamicImage, angle: f32) -> DynamicImage {
    let (w, h) = (src.width(), src.height());

    // 2. 旋转角度（15°）
    let angle_rad = angle.to_radians();

    // 3. 新宽高计算
    let cos_theta = angle_rad.cos().abs();
    let sin_theta = angle_rad.sin().abs();
    let new_w = (w as f32 * cos_theta + h as f32 * sin_theta).ceil() as u32;
    let new_h = (w as f32 * sin_theta + h as f32 * cos_theta).ceil() as u32;

    // 4. 原中心、新中心
    let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
    let (ncx, ncy) = (new_w as f32 / 2.0, new_h as f32 / 2.0);

    // 5. 创建输出图像（白色填充）
    let mut out = RgbaImage::from_pixel(new_w, new_h, Rgba([255, 255, 255, 255]));

    // 6. 逆变换，每个目标像素找原图像素
    let sin_neg = (-angle_rad).sin();
    let cos_neg = (-angle_rad).cos();

    for y_out in 0..new_h {
        for x_out in 0..new_w {
            // 目标坐标相对中心
            let dx = x_out as f32 - ncx;
            let dy = y_out as f32 - ncy;
            // 逆旋转
            let src_x = cos_neg * dx - sin_neg * dy + cx;
            let src_y = sin_neg * dx + cos_neg * dy + cy;

            // 最近邻采样（可改为双线性）
            if src_x >= 0.0 && src_x < w as f32 && src_y >= 0.0 && src_y < h as f32 {
                let px = src.get_pixel(src_x as u32, src_y as u32);
                out.put_pixel(x_out, y_out, px);
            }
            // 否则保持透明
        }
    }
    DynamicImage::ImageRgba8(out)

    /*let (w, h) = src.dimensions();
    let angle_rad = angle.to_radians(); //角度制转弧度制

    // 计算新尺寸
    let (new_w, new_h) = rotated_dimensions(w, h, angle_rad);

    // 创建空白画布用于包裹旋转后的图像
    let mut canvas = RgbaImage::from_pixel(new_w, new_h, Rgba([0, 0, 0, 0]));

    // 旋转图像
    let rotated_img = rotate_about_center(
        &src.to_rgba8(),
        angle,
        Interpolation::Bilinear,
        Rgba([0, 0, 0, 0]),
    );

    // 原图中心、目标画布中心
    let center_src = (w as f32 / 2.0, h as f32 / 2.0);
    let center_dst = (new_w as f32 / 2.0, new_h as f32 / 2.0);
    // 将旋转后的图像居中放到新画布
    image::imageops::overlay(
        &mut canvas,
        &rotated_img,
        (center_dst.0 - center_src.0) as i64,
        (center_dst.1 - center_src.1) as i64,
    );
    DynamicImage::ImageRgba8(canvas)*/
}

/// 图片作用矩阵
/// 计算变换后图片的尺寸和坐标偏移
fn bounding_box_after_transform(
    width: u32,
    height: u32,
    m: &Matrix3<f32>,
) -> ((u32, u32), (f32, f32)) {
    let corners = [
        Vector3::new(0.0, 0.0, 1.0),
        Vector3::new(width as f32, 0.0, 1.0),
        Vector3::new(0.0, height as f32, 1.0),
        Vector3::new(width as f32, height as f32, 1.0),
    ];
    let mut xs = vec![];
    let mut ys = vec![];
    for c in &corners {
        let v = m * c;
        xs.push(v.x);
        ys.push(v.y);
    }
    let min_x = xs.iter().cloned().fold(f32::INFINITY, f32::min);
    let max_x = xs.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let min_y = ys.iter().cloned().fold(f32::INFINITY, f32::min);
    let max_y = ys.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

    let new_width = (max_x - min_x).ceil() as u32;
    let new_height = (max_y - min_y).ceil() as u32;
    ((new_width, new_height), (min_x, min_y))
}

/// 对图片施加任意3x3矩阵变换，并输出完整包含所有数据的新图片
pub fn transform_image_full(src: &DynamicImage, matrix: &Matrix3<f32>) -> RgbaImage {
    let (w, h) = src.dimensions();
    let (new_size, min_xy) = bounding_box_after_transform(w, h, matrix);
    let (new_w, new_h) = new_size;
    let min_x = min_xy.0;
    let min_y = min_xy.1;

    // 偏移矩阵，把坐标平移到正区域
    let offset = Matrix3::new(1.0, 0.0, -min_x, 0.0, 1.0, -min_y, 0.0, 0.0, 1.0);
    let m2 = offset * matrix;
    let inv = m2.try_inverse().unwrap();

    let mut out_img = RgbaImage::new(new_w, new_h);

    for y in 0..new_h {
        for x in 0..new_w {
            let dst_pos = Vector3::new(x as f32, y as f32, 1.0);
            let src_pos = inv * dst_pos;
            let sx = src_pos.x;
            let sy = src_pos.y;
            if sx >= 0.0 && sy >= 0.0 && sx < w as f32 && sy < h as f32 {
                // 最近邻采样
                let pixel = src.get_pixel(sx as u32, sy as u32);
                out_img.put_pixel(x, y, pixel);
            }
        }
    }
    out_img
    //DynamicImage::ImageRgba8(out_img);
}
