use crate::writer::{GCodeWriter, SvgPathWriter};
use lyon_algorithms::aabb::fast_bounding_box;
use lyon_algorithms::walk::{walk_along_path, RegularPattern, WalkerEvent};
use lyon_path::iterator::PathIterator;

pub mod handler;
pub mod lines;
pub mod parser;
pub mod writer;
pub mod ydd;

/// 将有多个轮廓的[Path]拆成单轮廓的[Path]
pub fn split_path_contours(path: &lyon_path::Path) -> Vec<lyon_path::Path> {
    let mut contours = Vec::new();
    let mut builder = None;

    for event in path.iter() {
        match event {
            lyon_path::Event::Begin { at } => {
                builder = Some(lyon_path::Path::builder());
                builder.as_mut().unwrap().begin(at);
            }
            lyon_path::Event::Line { from, to } => {
                if let Some(b) = builder.as_mut() {
                    b.line_to(to);
                }
            }
            lyon_path::Event::Quadratic { from, ctrl, to } => {
                if let Some(b) = builder.as_mut() {
                    b.quadratic_bezier_to(ctrl, to);
                }
            }
            lyon_path::Event::Cubic {
                from,
                ctrl1,
                ctrl2,
                to,
            } => {
                if let Some(b) = builder.as_mut() {
                    b.cubic_bezier_to(ctrl1, ctrl2, to);
                }
            }
            lyon_path::Event::End { last, first, close } => {
                if let Some(mut b) = builder.take() {
                    b.end(close);
                    contours.push(b.build());
                }
            }
        }
    }
    contours
}

/// 将[Path]转换成GCode
/// - 支持多轮廓
///
/// - [tolerance] 公差 0.01
/// - [digit] GCode小数点位数
pub fn path_to_gcode(
    path: &lyon_path::Path,
    tolerance: f32,
    digit: usize,
    begin: &String,
) -> String {
    let mut writer = GCodeWriter::new(digit);
    if !begin.is_empty() {
        writer.write_line(begin);
    }
    path.iter()
        .flattened(tolerance)
        .for_each(|event| match event {
            lyon_path::Event::Begin { at } => {
                writer.move_to(at.x as f64, at.y as f64);
            }
            lyon_path::Event::Line { from, to } => {
                writer.line_to(to.x as f64, to.y as f64);
            }
            _ => {}
        });
    writer.to_string()
}

/// 将[Path]转换成svg path数据
/// - 支持多轮廓
///
/// - [tolerance] 公差 0.01
/// - [digit] 小数点位数
pub fn path_to_svg_path(
    path: &lyon_path::Path,
    tolerance: f32,
    digit: usize,
    begin: &String,
) -> String {
    let mut writer = SvgPathWriter::new(digit);
    if !begin.is_empty() {
        writer.write_line(begin);
    }
    path.iter()
        .flattened(tolerance)
        .for_each(|event| match event {
            lyon_path::Event::Begin { at } => {
                writer.move_to(at.x as f64, at.y as f64);
            }
            lyon_path::Event::Line { from, to } => {
                writer.line_to(to.x as f64, to.y as f64);
            }
            _ => {}
        });
    writer.to_string()
}

/// 将[Path]沿着路径一段一段转换成GCode
/// - 多轮廓将会先拆分成单轮廓
///
/// - [interval] 步进, 步长, 每一段的长度
/// - [tolerance] 公差 0.01
/// - [digit] GCode小数点位数 6
pub fn path_walk_along_to_gcode(
    path: &lyon_path::Path,
    interval: f32,
    tolerance: f32,
    digit: usize,
    begin: &String,
) -> String {
    let mut writer = GCodeWriter::new(digit);
    if !begin.is_empty() {
        writer.write_line(begin);
    }

    //--
    let path_vec = split_path_contours(path);
    for path in path_vec.iter() {
        let start = 0.0;
        let mut pattern = RegularPattern {
            callback: &mut |event: WalkerEvent| {
                if (event.distance == start) {
                    writer.move_to(event.position.x as f64, event.position.y as f64);
                } else {
                    writer.line_to(event.position.x as f64, event.position.y as f64);
                }
                true // Return true to continue walking the path.
            },
            // Invoke the callback above at a regular interval of 3 units.
            interval,
        };
        walk_along_path(path.iter(), start, tolerance, &mut pattern);
        //尾部会有interval范围内的长度接不上
        //let length = approximate_length(path.iter(), tolerance);
        //walk_along_path(path.iter(), length , tolerance, &mut pattern);
    }

    //--
    writer.to_string()
}

/// 获取路径的边界LTRB
pub fn path_bounds(path: &lyon_path::Path) -> (f32, f32, f32, f32) {
    let box2d = fast_bounding_box(path);
    (box2d.min.x, box2d.min.y, box2d.max.x, box2d.max.y)
}

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/09/03
///

#[cfg(test)]
mod tests {
    use crate::handler::{GCodeValueHandlerImpl, GCodeValueHandlerPath};
    use crate::parser::GCodeParser;
    use crate::writer::GCodeWriter;
    use crate::{path_bounds, path_to_gcode, path_to_svg_path, path_walk_along_to_gcode};
    use lyon_algorithms::aabb::fast_bounding_box;
    use lyon_path::iterator::PathIterator;
    use lyon_path::math::point;
    use lyon_path::{Path, Winding};
    use rc_basis::files::read_file_to_string;
    use rc_basis::test::{get_test_file_path, get_test_output_file_path, save_and_open_file};

    #[test]
    fn it_works() {
        let text = "a\na\r\n你好Rust";
        println!("{}", text.len());
        println!("{}", text.chars().count());

        let mut count = 0;
        for c in text.chars() {
            println!("{}", c);
            count += 1;
        }
        println!("{}", count);
    }

    #[test]
    fn test_gcode_parser() {
        //let gcode_path = get_test_file_path(".output/path_to_gcode.gcode");
        let gcode_path = get_test_file_path("Toothy_Baby_Croc.gcode");
        let gcode = read_file_to_string(gcode_path.as_str()).unwrap();
        let mut parser = GCodeParser::new(&gcode);
        parser.parse(&mut GCodeValueHandlerImpl::default());
    }

    #[test]
    fn test_gcode_path_parser() {
        //let gcode_path = get_test_file_path(".output/path_to_gcode.gcode");
        let gcode_path = get_test_file_path("Toothy_Baby_Croc.gcode");
        let gcode = read_file_to_string(gcode_path.as_str()).unwrap();
        let mut parser = GCodeParser::new(&gcode);

        let mut handler = GCodeValueHandlerPath::default();
        parser.parse(&mut handler);
        println!("{}", handler.layers.len());

        for (i, layer) in handler.layers.iter().enumerate() {
            println!(
                "第{}层:Z={} {:?}",
                i,
                layer.z_f32(),
                path_bounds(&layer.path)
            );

            if (i == 20) {
                let gcode = path_to_gcode(&layer.path, 0.01, 6, &"G90\nG21".to_string());
                let output = get_test_output_file_path("path_to_gcode.gcode");
                save_and_open_file(&output, gcode.as_bytes());
            }
        }
    }

    #[test]
    fn test_gcode_writer() {
        let mut writer = GCodeWriter::new(6);
        writer.write_line("G90"); //绝对位置
        writer.write_line("G21"); //单位为mm
        //writer.move_to(10.0, 10.0);
        //writer.line_to(100.0, 100.0);
        //writer.line_to(50.123456789, 20.2345);

        writer.move_to(-10.0, 0.0);
        writer.arc_to(0.0, -10.0, 0.0, 0.0, false);
        writer.arc_to(10.0, 0.0, 0.0, 0.0, false);
        writer.arc_to(0.0, 10.0, 0.0, 0.0, false);
        writer.arc_to(-10.0, 0.0, 0.0, 0.0, false);
        println!("{}", writer.to_string());
    }
    #[test]
    fn test_path_gcode_writer() {
        let mut writer = GCodeWriter::new(6);
        writer.write_line("G90"); //绝对位置
        writer.write_line("G21"); //单位为mm

        let mut builder = Path::builder();
        //builder.end(false);
        builder.begin(point(10., 10.));
        builder.line_to(point(30., 40.));
        /*builder.end(false);
        builder.add_circle(point(20., 20.), 5., Winding::Positive);
        builder.begin(point(30., 30.));
        builder.line_to(point(60., 80.));
        builder.end(false);*/

        //builder.add_circle(point(0., 0.), 10., Winding::Positive);
        let path = builder.build();

        path.iter().flattened(0.01).for_each(|event| match event {
            lyon_path::Event::Begin { at } => {
                writer.move_to(at.x as f64, at.y as f64);
            }
            lyon_path::Event::Line { from, to } => {
                writer.line_to(to.x as f64, to.y as f64);
            }
            /*lyon_path::Event::Quadratic { from, ctrl, to } => {
                //writer.quadratic_bezier_to(ctrl.x, ctrl.y, to.x, to.y);

                //扁平化成线
                let mut sub_builder = Path::builder();
                sub_builder.begin(from);
                sub_builder.quadratic_bezier_to(ctrl, to);
                sub_builder.end(false);
                let sub_path = sub_builder.build();
                sub_path
                    .iter()
                    .flattened(0.01)
                    .for_each(|event| match event {
                        lyon_path::Event::Begin { at } => {
                            //writer.move_to(at.x as f64, at.y as f64);
                        }
                        lyon_path::Event::Line { from, to } => {
                            writer.line_to(to.x as f64, to.y as f64);
                        }
                        _ => {}
                    })
            }
            lyon_path::Event::Cubic {
                from,
                ctrl1,
                ctrl2,
                to,
            } => {
                /*writer.arc_to(
                    to.x as f64,
                    to.y as f64,
                    (to.x + from.x / 2f32) as f64,
                    (to.y + from.y / 2f32) as f64,
                    false,
                );*/
                //扁平化成线
                let mut sub_builder = Path::builder();
                sub_builder.begin(from);
                sub_builder.cubic_bezier_to(ctrl1, ctrl2, to);
                sub_builder.end(false);
                let sub_path = sub_builder.build();
                sub_path
                    .iter()
                    .flattened(0.01)
                    .for_each(|event| match event {
                        lyon_path::Event::Begin { at } => {
                            //writer.move_to(at.x as f64, at.y as f64);
                        }
                        lyon_path::Event::Line { from, to } => {
                            writer.line_to(to.x as f64, to.y as f64);
                        }
                        _ => {}
                    })
            }*/
            _ => {}
        });

        let output = get_test_output_file_path("path_to_gcode.gcode");
        save_and_open_file(&output, writer.to_string().as_bytes());
        println!("{}", writer.to_string());

        let bounds = fast_bounding_box(path.iter());
        println!("The bounding box is: {:?}.", bounds);
    }

    #[test]
    fn test_path_walk_along_to_gcode() {
        let mut builder = Path::builder();
        builder.begin(point(10., 10.));
        builder.line_to(point(20., 20.));
        builder.end(false);
        builder.begin(point(20., 10.));
        builder.line_to(point(30., 20.));
        builder.end(false);
        builder.add_circle(point(30., 10.), 5., Winding::Positive);

        let path = builder.build();

        let gcode = path_walk_along_to_gcode(&path, 0.1, 0.01, 6, &"G90\nG21".to_string());
        let output = get_test_output_file_path("path_walk_along_to_gcode.gcode");
        save_and_open_file(&output, gcode.as_bytes());
        println!("{}", gcode);
    }

    #[test]
    fn test_path_to_gcode() {
        let mut builder = Path::builder();
        builder.begin(point(10., 10.));
        builder.line_to(point(20., 20.));
        builder.end(false);

        builder.begin(point(20., 10.));
        builder.line_to(point(30., 20.));
        builder.end(false);

        let path = builder.build();

        let gcode = path_to_gcode(&path, 0.1, 6, &"G90\nG21".to_string());
        let output = get_test_output_file_path("path_to_gcode2.gcode");
        // let gcode = path_walk_along_to_gcode(&path, 0.5, 0.01, 6, &"G90\nG21".to_string());
        //let output = get_test_output_file_path("path_walk_along_to_gcode.gcode");
        save_and_open_file(&output, gcode.as_bytes());
        println!("{}", gcode);
    }

    #[test]
    fn test_path_to_svg_path() {
        let mut builder = Path::builder();
        builder.begin(point(10., 10.));
        builder.line_to(point(20., 20.));
        builder.end(false);
        builder.begin(point(20., 10.));
        builder.line_to(point(30., 20.));
        builder.end(false);
        builder.add_circle(point(30., 10.), 5., Winding::Positive);

        let path = builder.build();

        let gcode = path_to_svg_path(&path, 0.1, 6, &"".to_string());
        let output = get_test_output_file_path("path_to_svg_path.txt");
        // let gcode = path_walk_along_to_gcode(&path, 0.5, 0.01, 6, &"G90\nG21".to_string());
        //let output = get_test_output_file_path("path_walk_along_to_gcode.gcode");
        save_and_open_file(&output, gcode.as_bytes());
        println!("{}", gcode);
    }
}
