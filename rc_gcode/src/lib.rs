pub mod writer;

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/09/03
///

#[cfg(test)]
mod tests {
    use crate::writer::GCodeWriter;
    use lyon_path::iterator::PathIterator;
    use lyon_path::math::point;
    use lyon_path::{Path, Winding};

    #[test]
    fn it_works() {}

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
        builder.begin(point(10., 10.));
        builder.line_to(point(30., 40.));
        builder.end(false);
        builder.add_circle(point(20., 20.), 5., Winding::Positive);
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

        println!("{}", writer.to_string());
    }
}
