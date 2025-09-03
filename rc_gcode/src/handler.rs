use crate::parser::GCodeValue;
use lyon_path::geom::point;
use lyon_path::Path;
use std::cell::RefCell;

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/09/03
///
/// 用来处理[GCodeValue]
pub trait GCodeValueHandler {
    /// 开始
    fn start(&mut self) {}
    /// 处理[GCodeValue]
    fn handle_gcode_value(&mut self, gcode_value_line: Vec<GCodeValue>);
    /// 结束
    fn end(&mut self) {}
}

/// 日志输出实现
pub struct GCodeValueHandlerImpl {
    /// 行数
    pub line_count: usize,
}

impl Default for GCodeValueHandlerImpl {
    fn default() -> Self {
        GCodeValueHandlerImpl { line_count: 0 }
    }
}

impl GCodeValueHandler for GCodeValueHandlerImpl {
    fn start(&mut self) {
        self.line_count = 0;
    }

    fn handle_gcode_value(&mut self, gcode_value_line: Vec<GCodeValue>) {
        self.line_count += 1;
        println!("{:?}", gcode_value_line);
    }

    fn end(&mut self) {
        println!("line_count:{}", self.line_count);
    }
}

/// 将[GCodeValue]解析成[Path]
/// - 如果遇到了Z, 那么之后的数据都会合并到一层中
pub struct GCodeValueHandlerPath {
    /// 每一层的数据
    pub layers: Vec<GCodeValueHandlerPathLayer>,
    //--
    /// 当前数据相对于mm需要缩放的系数
    /// 用来将英寸值转换成mm值
    /// 1inch = 25.4mm
    mm_value_scale: f32,
    /// 当前是否是相对坐标
    is_relative: bool,
    /// 最后一次的x/y
    last_x: f32,
    last_y: f32,
    /// 当前数据所处的z坐标
    z: Option<GCodeValue>,
    /// 路径的构建器
    last_path_builder: RefCell<Option<lyon_path::Builder>>,
    /// 是否开始了路径
    is_path_begin: bool,
    /// 路径是否有直线
    is_path_line: bool,
}

impl Default for GCodeValueHandlerPath {
    fn default() -> Self {
        GCodeValueHandlerPath {
            layers: vec![],
            mm_value_scale: 1.0,
            is_relative: false,
            last_x: 0.0,
            last_y: 0.0,
            z: None,
            last_path_builder: RefCell::new(None),
            is_path_begin: false,
            is_path_line: false,
        }
    }
}

impl GCodeValueHandlerPath {
    /// 移动到指定位置
    fn move_to(&mut self, gcode_value_line: &Vec<GCodeValue>) {
        let x_value = gcode_value_line
            .iter()
            .find(|gcode_value| gcode_value.is_x());
        let y_value = gcode_value_line
            .iter()
            .find(|gcode_value| gcode_value.is_y());
        //--
        let x = if let Some(x_value) = x_value {
            let x = x_value.value_f32() * self.mm_value_scale;
            if self.is_relative { x + self.last_x } else { x }
        } else {
            self.last_x
        };
        let y = if let Some(y_value) = y_value {
            let y = y_value.value_f32() * self.mm_value_scale;
            if self.is_relative { y + self.last_y } else { y }
        } else {
            self.last_y
        };
        if self.last_path_builder.borrow().is_none() {
            self.last_path_builder = RefCell::new(Some(lyon_path::Builder::new()));
            self.is_path_begin = false;
            self.is_path_line = false;
        }
        if let Some(last_path_builder) = self.last_path_builder.get_mut() {
            if self.is_path_begin {
                // 结束上一层
                self.is_path_begin = false;
                last_path_builder.end(false);
            }
            self.is_path_begin = true;
            last_path_builder.begin(point(x, y));
        }
    }

    /// 连接到指定位置
    fn line_to(&mut self, gcode_value_line: &Vec<GCodeValue>) {
        let x_value = gcode_value_line
            .iter()
            .find(|gcode_value| gcode_value.is_x());
        let y_value = gcode_value_line
            .iter()
            .find(|gcode_value| gcode_value.is_y());
        //--
        let x = if let Some(x_value) = x_value {
            let x = x_value.value_f32() * self.mm_value_scale;
            if self.is_relative { x + self.last_x } else { x }
        } else {
            self.last_x
        };
        let y = if let Some(y_value) = y_value {
            let y = y_value.value_f32() * self.mm_value_scale;
            if self.is_relative { y + self.last_y } else { y }
        } else {
            self.last_y
        };
        if self.last_path_builder.borrow().is_none() {
            self.last_path_builder = RefCell::new(Some(lyon_path::Builder::new()));
            self.is_path_begin = false;
            self.is_path_line = false;
            if let Some(last_path_builder) = self.last_path_builder.get_mut() {
                self.is_path_begin = true;
                last_path_builder.begin(point(x, y));
            }
        } else {
            if let Some(last_path_builder) = self.last_path_builder.get_mut() {
                self.is_path_line = true;
                last_path_builder.line_to(point(x, y));
            }
        }
    }

    /// 追加最后一层, 如果有
    fn append_last_layer(&mut self) {
        if (self.is_path_line) {
            //之前收集到了数据
            if let Some(mut last_path_builder) = self.last_path_builder.clone().into_inner() {
                if self.is_path_begin {
                    last_path_builder.end(false);
                }
                let path = last_path_builder.build();
                self.layers.push(GCodeValueHandlerPathLayer {
                    z: self.z.clone(),
                    path,
                });
            }
        }
        self.z = None;
        self.last_path_builder = RefCell::new(None);
        self.is_path_begin = false;
        self.is_path_line = false;
    }
}

/// [GCodeValueHandlerPath]每一层的数据
pub struct GCodeValueHandlerPathLayer {
    /// 当前层Z坐标, 如果有
    pub z: Option<GCodeValue>,
    /// 核心路径数据
    pub path: Path,
}

impl GCodeValueHandlerPathLayer {
    pub fn have_z(&self) -> bool {
        self.z.is_some()
    }
    pub fn z_f32(&self) -> f32 {
        if let Some(value) = &self.z {
            return value.value_f32();
        }
        0.0
    }

    /// 数值
    pub fn z_f64(&self) -> f64 {
        if let Some(value) = &self.z {
            return value.value_f64();
        }
        0.0
    }
}

impl GCodeValueHandler for GCodeValueHandlerPath {
    fn handle_gcode_value(&mut self, gcode_value_line: Vec<GCodeValue>) {
        for gcode_value in &gcode_value_line {
            let cmd = gcode_value.to_string();
            match cmd.as_str() {
                "G90" => {
                    //绝对位置
                    self.is_relative = false;
                }
                "G91" => {
                    //相对位置
                    self.is_relative = true;
                }
                "G20" => {
                    // inch数值
                    self.mm_value_scale = 25.4;
                }
                "G21" => {
                    // mm数值
                    self.mm_value_scale = 1.0;
                }
                "G0" => {
                    let have_xy = gcode_value_line
                        .iter()
                        .any(|gcode_value| gcode_value.is_xy());
                    if have_xy {
                        self.move_to(&gcode_value_line);
                        break;
                    }
                }
                "G1" => {
                    let have_xy = gcode_value_line
                        .iter()
                        .any(|gcode_value| gcode_value.is_xy());
                    if have_xy {
                        self.line_to(&gcode_value_line);
                        break;
                    }
                }
                _ => {
                    if match gcode_value.command.as_str() {
                        "Z" => {
                            //有层了
                            self.append_last_layer();
                            self.z = Some(gcode_value.clone());
                            true
                        }
                        _ => false,
                    } {
                        break;
                    }
                }
            }
        }
    }

    fn end(&mut self) {
        self.append_last_layer()
    }
}
