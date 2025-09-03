pub mod writer;

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/09/03
///

#[cfg(test)]
mod tests {
    use crate::writer::GCodeWriter;

    #[test]
    fn it_works() {}

    #[test]
    fn test_gcode_writer() {
        let mut writer = GCodeWriter::new(6);
        writer.write_line("G90"); //绝对位置
        writer.write_line("G21"); //单位为mm
        writer.move_to(10.0, 10.0);
        writer.line_to(100.0, 100.0);
        writer.line_to(50.123456789, 20.2345);
        println!("{}", writer.to_string());
    }
}
