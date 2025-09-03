use crate::handler::GCodeValueHandler;
use std::str::Chars;

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/09/03
///
/// 解析 GCode 字符串数据
pub struct GCodeParser {
    /// GCode 数据
    gcode: String,
}

/// GCode 数值部分
/// - G90
/// - G21
/// - G0 / G1 / G2 / G3
/// - S / F / M
/// - X / Y / I / J
#[derive(Clone, Debug)]
pub struct GCodeValue {
    /// 指令
    pub command: String,
    /// 数值
    pub value: String,
}

impl GCodeValue {
    pub fn new() -> Self {
        Self {
            command: "".to_string(),
            value: "".to_string(),
        }
    }

    /// 拼接在一起
    pub fn to_string(&self) -> String {
        format!("{}{}", self.command, self.value)
    }

    pub fn is_xy(&self) -> bool {
        self.is_x() || self.is_y()
    }

    pub fn is_x(&self) -> bool {
        self.command == "X"
    }
    pub fn is_y(&self) -> bool {
        self.command == "Y"
    }

    /// 数值
    pub fn value_f32(&self) -> f32 {
        self.value.parse::<f32>().unwrap_or(0.0)
    }

    /// 数值
    pub fn value_f64(&self) -> f64 {
        self.value.parse::<f64>().unwrap_or(0.0)
    }
}

/// 实现[Display]
impl std::fmt::Display for GCodeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.command, self.value)
    }
}

impl GCodeParser {
    pub fn new(gcode: String) -> Self {
        Self { gcode }
    }

    /// 开始解析
    pub fn parse(&mut self, handler: &mut impl GCodeValueHandler) {
        let mut chars = self.gcode.chars();
        handler.start();
        loop {
            let (line, is_end) = self._read_gcode_value_line(&mut chars);
            if !line.is_empty() {
                handler.handle_gcode_value(line);
            }
            if is_end {
                break;
            }
        }
        handler.end();
    }

    /// 读取一行中有效的[GCodeValue]
    fn _read_gcode_value_line(&self, chars: &mut Chars) -> (Vec<GCodeValue>, bool) {
        let mut values: Vec<GCodeValue> = Vec::new();
        let mut value = GCodeValue::new();
        while let Some(c) = chars.next() {
            match c {
                //有效指令
                'G' | 'M' | 'X' | 'Y' | 'I' | 'J' | 'S' | 'F' | 'Z' => {
                    if !value.command.is_empty() {
                        // 处理上一个指令
                        values.push(value);
                        value = GCodeValue::new();
                    }
                    value.command.push(c);
                }
                //有效数字
                '0'..='9' | '.' | '-' | '+' | 'E' | 'e' => {
                    value.value.push(c);
                }
                ';' => {
                    //注释, 则跳过后续所有内容
                    self._skip_until_newline(chars);
                    if !value.command.is_empty() {
                        values.push(value);
                    }
                    return (values, false);
                }
                _ => {
                    if self._is_newline(c) {
                        if !value.command.is_empty() {
                            values.push(value);
                        }
                        return (values, false);
                    }
                }
            }
        }
        (values, true)
    }

    /// 跳过所有内容, 直到换行
    fn _skip_until_newline(&self, chars: &mut Chars) {
        while let Some(c) = chars.next() {
            if self._is_newline(c) {
                break;
            }
        }
    }

    /// 是否是换行符
    fn _is_newline(&self, c: char) -> bool {
        c == '\n' || c == '\r'
    }
}
