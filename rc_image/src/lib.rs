pub use image;
pub use imageproc;

pub mod convert;
pub mod read;
pub mod write;

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/27
///

#[cfg(test)]
mod tests {

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
}
