pub use image;
pub use imageproc;

pub mod convert;
pub mod matrix;
pub mod read;
pub mod write;

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/27
///

#[cfg(test)]
mod tests {
    use crate::convert::transform_image_full;
    use crate::matrix::{deg_to_rad, rotate_matrix, scale_matrix};
    use image::DynamicImage;
    use imageproc::drawing::Canvas;
    use rc_basis::files::open_file_with_sys;
    use rc_basis::test::get_test_output_file_path;

    #[test]
    fn it_works() {}

    #[test]
    fn test_image_to_base64() {
        //let image_path = "tests/Looks Good To Me.png";
        let image_path = "../tests/img.png";
        let output_path = "../.output/img_base64.txt";
        let base64 = crate::convert::read_image_file_to_base64(image_path).unwrap();
        rc_basis::files::save_string_to_file(output_path, base64.as_str()).unwrap();

        let output_path2 = "../.output/img_base64.png";
        let bytes = rc_basis::files::read_file_bytes(output_path).unwrap();
        crate::convert::base64_to_image(rc_basis::bytes::bytes_to_string(&bytes).as_str())
            .unwrap()
            .save(output_path2)
            .unwrap();
    }

    #[test]
    fn test_image_resize() {
        rc_basis::ptl_current_dir!();
        let image = crate::read::read_image_file("../tests/test.png").unwrap();
        let scale = 10;
        let width = image.width() * scale;
        let height = image.height() * scale;
        rc_basis::ptl!("width:{width} height:{height}");
        rc_basis::wrap_time_cost("Nearest", || {
            let image = image.resize(width, height, image::imageops::FilterType::Nearest);
            /*crate::write::write_image_file(&image, "../tests/.output/test_nearest.png", None)
            .unwrap();*/
        });
        rc_basis::wrap_time_cost("Triangle", || {
            let image = image.resize(width, height, image::imageops::FilterType::Triangle);
            /*crate::write::write_image_file(&image, "../tests/.output/test_triangle.png", None)
            .unwrap();*/
        });
        rc_basis::wrap_time_cost("CatmullRom", || {
            let image = image.resize(width, height, image::imageops::FilterType::CatmullRom);
            /*crate::write::write_image_file(&image, "../tests/.output/test_catmull_rom.png", None)
            .unwrap();*/
        });
        rc_basis::wrap_time_cost("Gaussian", || {
            let image = image.resize(width, height, image::imageops::FilterType::Gaussian);
            /*crate::write::write_image_file(&image, "../tests/.output/test_gaussian.png", None)
            .unwrap();*/
        });
        rc_basis::wrap_time_cost("Lanczos3", || {
            let image = image.resize(width, height, image::imageops::FilterType::Lanczos3);
            /*crate::write::write_image_file(&image, "../tests/.output/test_lanczos3.png", None)
            .unwrap();*/
        });
    }

    /// 测试图片变换矩阵
    #[test]
    fn test_image_transform() {
        let image = crate::read::read_image_file("../tests/test.png").unwrap();
        //创建一个旋转矩阵Matrix3
        let rotate_matrix = rotate_matrix(deg_to_rad(45.0));
        //创建一个缩放矩阵Matrix3
        let scale_matrix = scale_matrix(0.5, 1.5);
        let new_image = transform_image_full(&image, &(scale_matrix * rotate_matrix));
        //let new_image = DynamicImage::ImageRgba8(new_image);

        let output = get_test_output_file_path(format!("{}.png", "test_transform").as_str());
        new_image.save(output.as_str()).unwrap();

        open_file_with_sys(&output);
    }
}
