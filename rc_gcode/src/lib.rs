use crate::writer::GCodeWriter;
use lyon_algorithms::aabb::fast_bounding_box;
use lyon_path::iterator::PathIterator;

pub mod handler;
pub mod parser;
pub mod writer;

/// 将[Path]转换成GCode
///
/// - [tolerance] 公差 0.01
/// - [precision] GCode小数点位数
pub fn path_to_gcode(
    path: &lyon_path::Path,
    tolerance: f32,
    precision: usize,
    begin: &String,
) -> String {
    let mut writer = GCodeWriter::new(precision);
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
    use crate::{path_bounds, path_to_gcode};
    use lyon_algorithms::aabb::fast_bounding_box;
    use lyon_path::iterator::PathIterator;
    use lyon_path::math::point;
    use lyon_path::Path;
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
        let mut parser = GCodeParser::new(gcode);
        parser.parse(&mut GCodeValueHandlerImpl::default());
    }

    #[test]
    fn test_gcode_path_parser() {
        //let gcode_path = get_test_file_path(".output/path_to_gcode.gcode");
        let gcode_path = get_test_file_path("Toothy_Baby_Croc.gcode");
        let gcode = read_file_to_string(gcode_path.as_str()).unwrap();
        let mut parser = GCodeParser::new(gcode);

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
}
