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
    digit: usize,

    /// 当前的X坐标
    x: f64,

    /// 当前的Y坐标
    y: f64,
}

/// 实现Default
impl Default for GCodeWriter {
    fn default() -> Self {
        Self::new(6)
    }
}

impl GCodeWriter {
    pub fn new(digit: usize) -> Self {
        Self {
            lines: vec![],
            digit,
            x: 0.0,
            y: 0.0,
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
        format!("{:.precision$}", value, precision = self.digit)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
        self.write_line(&format!(
            "G0 X{} Y{}",
            self.format_value(x),
            self.format_value(y),
        ));
    }

    pub fn line_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
        self.write_line(&format!(
            "G1 X{} Y{}",
            self.format_value(x),
            self.format_value(y),
        ));
    }

    /// 顺时针绘制一个圆弧
    /// - `G2` 顺时针画弧
    /// - `G3` 逆时针画弧
    /// - [clockwise] 是否顺时针绘制
    pub fn arc_to(&mut self, x: f64, y: f64, cx: f64, cy: f64, clockwise: bool) {
        let i = cx - self.x;
        let j = cy - self.y;
        self.x = x;
        self.y = y;
        self.write_line(&format!(
            "{} X{} Y{} I{} J{}",
            if clockwise { "G2" } else { "G3" },
            self.format_value(x),
            self.format_value(y),
            self.format_value(i),
            self.format_value(j),
        ));
    }
}

//--

pub struct SvgPathWriter {
    /// 写入的一行一行数据
    lines: Vec<String>,

    /// 保留几位小数点
    digit: usize,

    /// 当前的X坐标
    x: f64,

    /// 当前的Y坐标
    y: f64,
}

impl Default for SvgPathWriter {
    fn default() -> Self {
        Self::new(6)
    }
}

impl SvgPathWriter {
    pub fn new(digit: usize) -> Self {
        Self {
            lines: vec![],
            digit,
            x: 0.0,
            y: 0.0,
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
        self.lines.join(" ")
    }

    //--

    fn format_value(&self, value: f64) -> String {
        format!("{:.precision$}", value, precision = self.digit)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
        self.write_line(&format!(
            "M{},{}",
            self.format_value(x),
            self.format_value(y),
        ));
    }

    pub fn line_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
        self.write_line(&format!(
            "L{},{}",
            self.format_value(x),
            self.format_value(y),
        ));
    }

    /// 二阶贝塞尔
    pub fn bezier_to(&mut self, x1: f64, y1: f64, x: f64, y: f64) {
        self.x = x;
        self.y = y;
        self.write_line(&format!(
            "Q{},{} {},{}",
            self.format_value(x1),
            self.format_value(y1),
            self.format_value(x),
            self.format_value(y),
        ));
    }

    /// 三阶贝塞尔
    pub fn bezier3_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x: f64, y: f64) {
        self.x = x;
        self.y = y;
        self.write_line(&format!(
            "C{},{} {},{} {},{}",
            self.format_value(x1),
            self.format_value(y1),
            self.format_value(x2),
            self.format_value(y2),
            self.format_value(x),
            self.format_value(y),
        ));
    }

    /// 圆弧
    pub fn arc_to(&mut self, x: f64, y: f64, cx: f64, cy: f64, clockwise: bool) {
        self.x = x;
        self.y = y;
        self.write_line(&format!(
            "A{},{} {} {} {},{}",
            self.format_value(cx),
            self.format_value(cy),
            if clockwise { 1 } else { 0 },
            0,
            self.format_value(x),
            self.format_value(y),
        ));
    }
}
