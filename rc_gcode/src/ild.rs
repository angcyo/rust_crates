use crate::split_path_contours;
use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, DynamicImage, GenericImageView, Pixel, Rgba};
use lyon_algorithms::walk::{RegularPattern, WalkerEvent, walk_along_path};
use std::fs::File;
use std::io::BufReader;
use std::u8;

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/09/09
///
/// ild 格式写入工具
///
/// https://ilda.com/resources/StandardsDocs/ILDA_IDTF14_rev011.pdf
///
pub struct IldWriter {
    /// 最终的字节数据
    pub bytes: Vec<u8>,
}

impl Default for IldWriter {
    fn default() -> Self {
        Self { bytes: vec![] }
    }
}

impl IldWriter {
    /// 获取一个字节的状态码
    /// - [on] 是否开激光
    pub fn get_status_code(on: bool) -> u8 {
        0b0_0_0_0_0_0_0_0
            | if on {
                0b0_0_0_0_0_0_0_0
            } else {
                0b0_1_0_0_0_0_0_0
            }
    }

    pub fn write_vec(&mut self, bytes: &Vec<u8>) {
        self.bytes.extend_from_slice(bytes);
    }

    pub fn write_ascii_string(&mut self, string: &str) {
        self.write_bytes(string.as_bytes());
    }

    /// 写入一个字节数组
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    /// 填充指定个字节数, 填充为0
    pub fn fill_byte(&mut self, count: usize) {
        for _ in 0..count {
            self.bytes.push(0);
        }
    }

    /// 写入一个字节
    pub fn write_byte(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    /// 写入一个有符号整数
    pub fn write_int16(&mut self, value: i16) {
        let bytes = value.to_be_bytes();
        self.write_bytes(&bytes);
    }

    /// 写入头信息
    /// - [fmt] 数据格式
    ///     - 格式 0 – 带索引颜色的 3D 坐标
    ///     - 格式 1 – 带索引颜色的 2D 坐标
    ///     - 格式 2 – 索引色框的调色板
    ///     - 格式 3 在 ILDA 技术委员会内提出，但从未获得批准。
    ///     - 格式 4 – 具有真彩色的 3D 坐标
    ///     - 格式 5 – 真彩色 2D 坐标
    /// - [frame_count] 总帧数, 范围为 1 – 65535。对于调色板，此值应为 0。
    /// - [frame_index] 当前帧索引, 计数从第 0 帧开始。范围为 0 – 65534。
    /// - [data_count] 当前帧的数据个数 （0 – 65535）
    pub fn writer_header(&mut self, fmt: u8, frame_count: u16, frame_index: u16, data_count: u16) {
        self.write_ascii_string("ILDA"); //1~4
        self.fill_byte(3); //5~7 预留
        self.write_byte(fmt); //8 数据格式
        self.write_ascii_string("angcyo.f"); //9~16 框架名
        self.write_ascii_string("angcyo.c"); //17~24 公司名
        self.write_bytes(&data_count.to_be_bytes()); //25~26 无符号整数
        self.write_bytes(&frame_index.to_be_bytes()); //27~28 无符号整数
        self.write_bytes(&frame_count.to_be_bytes()); //29~30 无符号整数
        self.write_byte(0); //31 投影仪编号
        self.write_byte(0); //32 预留
    }

    /// 写入一个3D索引颜色坐标点
    pub fn write_point_3d_index_color(&mut self, point: &(i16, i16, i16), color_index: u8) {
        self.write_bytes(&point.0.to_be_bytes()); //X
        self.write_bytes(&point.1.to_be_bytes()); //Y
        self.write_bytes(&point.2.to_be_bytes()); //Z
        self.write_byte(Self::get_status_code(true)); //状态码
        self.write_byte(color_index); //颜色索引
    }

    /// 写入一个2D索引颜色坐标点
    pub fn write_point_2d_index_color(&mut self, point: &(i16, i16), color_index: u8) {
        self.write_bytes(&point.0.to_be_bytes()); //X
        self.write_bytes(&point.1.to_be_bytes()); //Y
        self.write_byte(Self::get_status_code(true)); //状态码
        self.write_byte(color_index); //颜色索引
    }

    /// 写入一个颜色
    pub fn write_color(&mut self, r: u8, g: u8, b: u8) {
        self.write_byte(r);
        self.write_byte(g);
        self.write_byte(b);
    }

    pub fn write_bgr(&mut self, r: u8, g: u8, b: u8) {
        self.write_byte(b);
        self.write_byte(g);
        self.write_byte(r);
    }

    /// 写入一个3D索引真彩坐标点
    pub fn write_point_3d_index_rgb(&mut self, point: &(i16, i16, i16), r: u8, g: u8, b: u8) {
        self.write_bytes(&point.0.to_be_bytes()); //X
        self.write_bytes(&point.1.to_be_bytes()); //Y
        self.write_bytes(&point.2.to_be_bytes()); //Z
        self.write_byte(Self::get_status_code(true)); //状态码
        self.write_bgr(r, g, b);
    }

    /// 写入一个2D索引真彩坐标点
    pub fn write_point_2d_index_rgb(&mut self, point: &(i16, i16), r: u8, g: u8, b: u8) {
        self.write_bytes(&point.0.to_be_bytes()); //X
        self.write_bytes(&point.1.to_be_bytes()); //Y
        self.write_byte(Self::get_status_code(true)); //状态码
        self.write_bgr(r, g, b);
    }
}

/// 将路径[Path]转换为ild字节数据
///
/// - [path_to_ild_bytes]
/// - [path_to_ild_bytes_index]
pub fn path_to_ild_bytes(
    path: &lyon_path::Path,
    tolerance: f32,
    interval: f32,
    r: u8,
    g: u8,
    b: u8,
) -> Vec<u8> {
    let mut writer = IldWriter::default();
    // 获取路径上的点
    let mut points = vec![];

    let path_vec = split_path_contours(path);
    for path in path_vec {
        let start = 0.0;
        let mut pattern = RegularPattern {
            callback: &mut |event: WalkerEvent| {
                points.push((event.position.x, event.position.y));
                true
            },
            interval,
        };
        walk_along_path(path.iter(), start, tolerance, &mut pattern);
    }

    //防止超范围
    let count = points.len() as u16;
    writer.writer_header(5, 1, 0, count);
    for i in 0..count {
        let point = points[i as usize];
        writer.write_point_2d_index_rgb(&(point.0 as i16, point.1 as i16), r, g, b);
    }
    writer.bytes
}

pub fn path_to_ild_bytes_3d(
    path: &lyon_path::Path,
    tolerance: f32,
    interval: f32,
    r: u8,
    g: u8,
    b: u8,
    z: i16,
) -> Vec<u8> {
    let mut writer = IldWriter::default();
    // 获取路径上的点
    let mut points = vec![];

    let path_vec = split_path_contours(path);
    for path in path_vec {
        let start = 0.0;
        let mut pattern = RegularPattern {
            callback: &mut |event: WalkerEvent| {
                points.push((event.position.x, event.position.y));
                true
            },
            interval,
        };
        walk_along_path(path.iter(), start, tolerance, &mut pattern);
    }

    //防止超范围
    let count = points.len() as u16;
    writer.writer_header(4, 1, 0, count);
    for i in 0..count {
        let point = points[i as usize];
        writer.write_point_3d_index_rgb(&(point.0 as i16, point.1 as i16, z), r, g, b);
    }
    writer.bytes
}

/// - [path_to_ild_bytes]
/// - [path_to_ild_bytes_index]
pub fn path_to_ild_bytes_index(
    path: &lyon_path::Path,
    tolerance: f32,
    interval: f32,
    color_index: u8,
) -> Vec<u8> {
    let mut writer = IldWriter::default();
    // 获取路径上的点
    let mut points = vec![];

    let path_vec = split_path_contours(path);
    for path in path_vec {
        let start = 0.0;
        let mut pattern = RegularPattern {
            callback: &mut |event: WalkerEvent| {
                points.push((event.position.x, event.position.y));
                true
            },
            interval,
        };
        walk_along_path(path.iter(), start, tolerance, &mut pattern);
    }

    //防止超范围
    let count = points.len() as u16;
    writer.writer_header(1, 1, 0, count);
    for i in 0..count {
        let point = points[i as usize];
        writer.write_point_2d_index_color(&(point.0 as i16, point.1 as i16), color_index);
    }
    writer.bytes
}

pub fn path_to_ild_bytes_index_3d(
    path: &lyon_path::Path,
    tolerance: f32,
    interval: f32,
    color_index: u8,
    z: i16,
) -> Vec<u8> {
    let mut writer = IldWriter::default();
    // 获取路径上的点
    let mut points = vec![];

    let path_vec = split_path_contours(path);
    for path in path_vec {
        let start = 0.0;
        let mut pattern = RegularPattern {
            callback: &mut |event: WalkerEvent| {
                points.push((event.position.x, event.position.y));
                true
            },
            interval,
        };
        walk_along_path(path.iter(), start, tolerance, &mut pattern);
    }

    //防止超范围
    let count = points.len() as u16;
    writer.writer_header(0, 1, 0, count);
    for i in 0..count {
        let point = points[i as usize];
        writer.write_point_3d_index_color(&(point.0 as i16, point.1 as i16, z), color_index);
    }
    writer.writer_header(0, 1, 0, 0);
    writer.bytes
}

/// 将图片转换为ild字节数据
/// - [frame_count] 总帧数, 范围为 1 – 65535。对于调色板，此值应为 0。
/// - [frame_index] 当前帧索引, 计数从第 0 帧开始。范围为 0 – 65534。
/// - [gray_threshold] 灰度阈值, >这个值的像素, 视为白色255, 无数据
/// - [alpha_threshold] 透明阈值, 透明通道<=这个值的像素, 视为白色255, 无数据
///
/// - [image_path_to_ild_bytes]
/// - [image_to_ild_bytes]
/// - [gif_path_to_ild_bytes_2d_rgb]
pub fn image_to_ild_bytes(
    img: &DynamicImage,
    offset_x: i16,
    offset_y: i16,
    gray_threshold: u8,
    alpha_threshold: u8,
) -> Vec<u8> {
    let mut writer = IldWriter::default();
    write_image_to_ild_bytes_2d_rgb(
        &mut writer,
        img,
        1,
        0,
        offset_x,
        offset_y,
        gray_threshold,
        alpha_threshold,
    );
    writer.bytes
}

/// - [image_path_to_ild_bytes]
/// - [image_to_ild_bytes]
/// - [gif_path_to_ild_bytes_2d_rgb]
pub fn image_path_to_ild_bytes(
    img_path: &String,
    offset_x: i16,
    offset_y: i16,
    gray_threshold: u8,
    alpha_threshold: u8,
) -> Vec<u8> {
    let img = image::open(img_path).expect("打开文件失败!");
    image_to_ild_bytes(&img, offset_x, offset_y, gray_threshold, alpha_threshold)
}

/// 将Gif图片转换为ild字节数据
/// - [image_path_to_ild_bytes]
/// - [image_to_ild_bytes]
/// - [gif_path_to_ild_bytes_2d_rgb]
pub fn gif_path_to_ild_bytes_2d_rgb(
    gif_path: &String,
    offset_x: i16,
    offset_y: i16,
    gray_threshold: u8,
    alpha_threshold: u8,
) -> Vec<u8> {
    let gif = File::open(gif_path).expect("打开文件失败");
    let reader = BufReader::new(gif);
    let decoder = GifDecoder::new(reader).expect("gif解码失败");
    let frames = decoder.into_frames();
    let frames = frames.collect_frames().expect("gif解码失败");
    let len = frames.len();
    let mut frame_index = 0;

    let mut writer = IldWriter::default();
    frames.into_iter().for_each(|frame| {
        write_image_to_ild_bytes_2d_rgb(
            &mut writer,
            frame.buffer(),
            len as u16,
            frame_index,
            offset_x,
            offset_y,
            gray_threshold,
            alpha_threshold,
        );
        frame_index += 1;
    });

    writer.bytes
}

pub fn gif_path_to_ild_bytes_2d_color_index(
    gif_path: &String,
    offset_x: i16,
    offset_y: i16,
    gray_threshold: u8,
    alpha_threshold: u8,
    color_index: u8,
) -> Vec<u8> {
    let gif = File::open(gif_path).expect("打开文件失败");
    let reader = BufReader::new(gif);
    let decoder = GifDecoder::new(reader).expect("gif解码失败");
    let frames = decoder.into_frames();
    let frames = frames.collect_frames().expect("gif解码失败");
    let len = frames.len();
    let mut frame_index = 0;

    let mut writer = IldWriter::default();
    frames.into_iter().for_each(|frame| {
        //if (frame_index < 2) {
        write_image_to_ild_bytes_2d_color_index(
            &mut writer,
            frame.buffer(),
            len as u16,
            frame_index,
            offset_x,
            offset_y,
            gray_threshold,
            alpha_threshold,
            color_index,
        );
        //}
        frame_index += 1;
    });

    writer.bytes
}

/// 写入一帧数据到ild数据中
pub fn write_image_to_ild_bytes_2d_rgb<I>(
    writer: &mut IldWriter,
    img: &I,
    frame_count: u16,
    frame_index: u16,
    offset_x: i16,
    offset_y: i16,
    gray_threshold: u8,
    alpha_threshold: u8,
) where
    I: GenericImageView<Pixel = Rgba<u8>>,
{
    // 获取路径上的点
    let mut points = vec![];
    for (x, y, pixel) in img.pixels() {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];
        let a = pixel[3];

        //计算灰度值
        let gray = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
        if gray <= gray_threshold || a <= alpha_threshold {
            //黑色, 无数据
        } else {
            points.push((x as i16, y as i16, r, g, b));
        }
    }

    //防止超范围
    let count = points.len() as u16;
    writer.writer_header(5, frame_count, frame_index, count);
    for i in 0..count {
        let point = points[i as usize];
        writer.write_point_2d_index_rgb(
            &(point.0 + offset_x, point.1 + offset_y),
            point.2,
            point.3,
            point.4,
        );
    }
}

pub fn write_image_to_ild_bytes_2d_color_index<I>(
    writer: &mut IldWriter,
    img: &I,
    frame_count: u16,
    frame_index: u16,
    offset_x: i16,
    offset_y: i16,
    gray_threshold: u8,
    alpha_threshold: u8,
    color_index: u8,
) where
    I: GenericImageView<Pixel = Rgba<u8>>,
{
    // 获取路径上的点
    let mut points = vec![];
    for (x, y, pixel) in img.pixels() {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];
        let a = pixel[3];

        //计算灰度值
        let gray = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
        if gray <= gray_threshold || a <= alpha_threshold {
            //黑色, 无数据
        } else {
            points.push((x as i16, y as i16, r, g, b));
        }
    }

    //防止超范围
    let count = points.len() as u16;
    writer.writer_header(1, frame_count, frame_index, count);
    for i in 0..count {
        let point = points[i as usize];
        writer.write_point_2d_index_color(&(point.0 + offset_x, point.1 + offset_y), color_index);
    }
}

#[cfg(test)]
mod tests {
    use crate::ild::{
        gif_path_to_ild_bytes_2d_color_index, gif_path_to_ild_bytes_2d_rgb, path_to_ild_bytes,
        path_to_ild_bytes_3d, path_to_ild_bytes_index, path_to_ild_bytes_index_3d,
    };
    use lyon_path::Path;
    use lyon_path::math::point;
    use rc_basis::files::save_bytes_to_file;
    use rc_basis::test::{get_test_file_path, get_test_output_file_path};
    use std::fmt::format;

    #[test]
    fn test_path_to_ild_bytes() {
        let left = 10.0;
        let top = 10.0;
        let right = 10000.0;
        let bottom = 10000.0;
        let mut builder = Path::builder();
        builder.begin(point(left, top));
        builder.line_to(point(right, top));
        builder.line_to(point(right, bottom));
        builder.line_to(point(left, bottom));
        builder.line_to(point(left, top));
        builder.end(false);
        let path = builder.build();

        let r = 0xff;
        let g = 0x1F;
        let b = 0xFF;
        let color_index = 46;

        let use_color_index = false;
        let use_3d = true;
        //let bytes = path_to_ild_bytes(&path, 0.1, 1.0, r, g, b);
        //let bytes =
        let bytes = if use_3d {
            if use_color_index {
                path_to_ild_bytes_index_3d(&path, 0.1, 1.0, color_index, -1024)
            } else {
                path_to_ild_bytes_3d(&path, 0.1, 1.0, r, g, b, -1024)
            }
        } else {
            if use_color_index {
                path_to_ild_bytes_index(&path, 0.1, 1.0, color_index)
            } else {
                path_to_ild_bytes(&path, 0.1, 1.0, r, g, b)
            }
        };

        let output = get_test_output_file_path(
            format!(
                "path_to_ild_{}{}.ild",
                if use_3d { "3d" } else { "2d" },
                if use_color_index { "_index" } else { "" },
            )
            .as_str(),
        );
        save_bytes_to_file(output.as_str(), &bytes).unwrap();
    }

    #[test]
    fn test_image_to_ild_bytes() {
        //image_to_ild_bytes
        let count = 10 as u16;
        for i in 0..count {
            println!("{}", i)
        }
    }

    #[test]
    fn test_gif_to_ild_bytes() {
        //let gif = "向上走的小人.gif";
        let gif = "bo.gif";
        let color_index = 46;
        let gray_threshold = 140;
        let alpha_threshold = 250;

        let use_color_index = true;
        let use_3d = false;
        let bytes = if use_color_index {
            gif_path_to_ild_bytes_2d_color_index(
                &get_test_file_path(gif),
                0,
                0,
                gray_threshold,
                alpha_threshold,
                color_index,
            )
        } else {
            gif_path_to_ild_bytes_2d_rgb(
                &get_test_file_path(gif),
                0,
                0,
                gray_threshold,
                alpha_threshold,
            )
        };

        let output = get_test_output_file_path(
            format!(
                "gif_to_ild_{}{}.ild",
                if use_3d { "3d" } else { "2d" },
                if use_color_index { "_index" } else { "" },
            )
            .as_str(),
        );
        save_bytes_to_file(output.as_str(), &bytes).unwrap();
    }
}
