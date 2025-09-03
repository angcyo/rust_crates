///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/09/03
///
/// 用来生成GCode字符串数据
///
pub struct GCodeWriter {
    /// 写入的一行一行数据
    lines: Vec<String>,

    /// 保留几位小数点
    precision: usize,
}

/// 实现Default
impl Default for GCodeWriter {
    fn default() -> Self {
        Self::new(6)
    }
}

impl GCodeWriter {
    pub fn new(precision: usize) -> Self {
        Self {
            lines: vec![],
            precision,
        }
    }

    pub fn write_line(&mut self, line: &str) {
        self.lines.push(line.to_string());
    }

    pub fn write_lines(&mut self, lines: &[&str]) {
        for line in lines {
            self.write_line(line);
        }
    }

    pub fn to_string(&self) -> String {
        self.lines.join("\n")
    }

    //--

    fn format_value(&self, value: f64) -> String {
        format!("{:.precision$}", value, precision = self.precision)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        self.write_line(&format!(
            "G0 X{} Y{}",
            self.format_value(x),
            self.format_value(y),
        ));
    }

    pub fn line_to(&mut self, x: f64, y: f64) {
        self.write_line(&format!(
            "G1 X{} Y{}",
            self.format_value(x),
            self.format_value(y),
        ));
    }
}
