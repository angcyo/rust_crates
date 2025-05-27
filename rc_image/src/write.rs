use image::{ImageBuffer, Rgba};
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
