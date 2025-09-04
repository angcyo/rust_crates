///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/09/02
///
/// 字节写入工具
pub struct ByteWriter {
    /// 最终数据存放的字节数组
    pub bytes: Vec<u8>,
    /// 限制允许写入的最大字节数
    /// [usize::MAX]
    max_size: usize,
}

impl Default for ByteWriter {
    fn default() -> Self {
        Self::new(usize::MAX)
    }
}

impl ByteWriter {
    /// 创建一个字节写入工具
    pub fn new(max_size: usize) -> Self {
        Self {
            bytes: Vec::new(),
            max_size,
        }
    }

    /// 是否允许写入
    fn _can_write(&self) -> bool {
        self.bytes.len() < self.max_size
    }

    /// 写入一个字节
    pub fn write_byte(&mut self, byte: u8) -> bool {
        if self._can_write() {
            self.bytes.push(byte);
            return true;
        }
        false
    }

    pub fn write_vec(&mut self, bytes: &Vec<u8>) -> bool {
        if self._can_write() {
            self.bytes.extend_from_slice(bytes);
            return true;
        }
        false
    }

    /// 写入一个字节数组
    pub fn write_bytes(&mut self, bytes: &[u8]) -> bool {
        if self._can_write() {
            self.bytes.extend_from_slice(bytes);
            return true;
        }
        false
    }

    /// 写入一个字节数组, 接收迭代器参数
    pub fn write_bytes_iterator<'a>(&mut self, bytes: impl Iterator<Item = u8> + 'a) -> bool {
        if self._can_write() {
            self.bytes.extend(bytes);
            return true;
        }
        false
    }

    /// 写入ASCII字符串
    pub fn write_ascii_string(&mut self, string: &str) -> bool {
        if self._can_write() {
            self.write_bytes(string.as_bytes());
            return true;
        }
        false
    }

    /// 写入一个字符串
    /// - [write_end] 是否写入字符串的结束符 0
    pub fn write_string(&mut self, string: &str, write_end: bool) -> bool {
        if self._can_write() {
            self.bytes.extend_from_slice(string.as_bytes());
            if write_end {
                self.write_byte(0);
            }
            return true;
        }
        false
    }
    /// 写入一个有符号的整数
    /// - 支持1, 2, 4, 8字节数
    /// - 支持大小端序
    ///
    /// - [value] 待写入的数值
    /// - [size] 待写入的数值字节数, 1, 2, 4, 8字节
    /// - [le] 是否使用小端序
    pub fn write_int(&mut self, value: i64, size: usize, le: bool) -> bool {
        if self._can_write() {
            //let bytes = value.to_be_bytes();
            let bytes = value.to_le_bytes();
            let bytes = &bytes[0..size];
            if le {
                self.write_bytes(&bytes);
            } else {
                let bytes: Vec<u8> = bytes.iter().cloned().rev().collect();
                self.write_bytes(&bytes);
            };
        }
        false
    }

    pub fn write_int8(&mut self, value: i8, le: bool) -> bool {
        if self._can_write() {
            //let bytes = value.to_be_bytes();
            let bytes = value.to_le_bytes();
            if le {
                self.write_bytes(&bytes);
            } else {
                let bytes: Vec<u8> = bytes.iter().cloned().rev().collect();
                self.write_bytes(&bytes);
            };
        }
        false
    }

    pub fn write_int16(&mut self, value: i16, le: bool) -> bool {
        if self._can_write() {
            //let bytes = value.to_be_bytes();
            let bytes = value.to_le_bytes();
            if le {
                self.write_bytes(&bytes);
            } else {
                let bytes: Vec<u8> = bytes.iter().cloned().rev().collect();
                self.write_bytes(&bytes);
            };
        }
        false
    }

    pub fn write_int32(&mut self, value: i32, le: bool) -> bool {
        if self._can_write() {
            //let bytes = value.to_be_bytes();
            let bytes = value.to_le_bytes();
            if le {
                self.write_bytes(&bytes);
            } else {
                let bytes: Vec<u8> = bytes.iter().cloned().rev().collect();
                self.write_bytes(&bytes);
            };
        }
        false
    }

    pub fn write_int64(&mut self, value: i64, le: bool) -> bool {
        if self._can_write() {
            //let bytes = value.to_be_bytes();
            let bytes = value.to_le_bytes();
            if le {
                self.write_bytes(&bytes);
            } else {
                let bytes: Vec<u8> = bytes.iter().cloned().rev().collect();
                self.write_bytes(&bytes);
            };
        }
        false
    }

    pub fn write_float32(&mut self, value: f32, le: bool) -> bool {
        if self._can_write() {
            let bytes = value.to_le_bytes();
            if le {
                self.write_bytes(&bytes);
            } else {
                let bytes: Vec<u8> = bytes.iter().cloned().rev().collect();
                self.write_bytes(&bytes);
            };
            return true;
        }
        false
    }

    /// 写入一个有符号的浮点数
    pub fn write_float64(&mut self, value: f64, le: bool) -> bool {
        if self._can_write() {
            let bytes = value.to_le_bytes();
            if le {
                self.write_bytes(&bytes);
            } else {
                let bytes: Vec<u8> = bytes.iter().cloned().rev().collect();
                self.write_bytes(&bytes);
            };
            return true;
        }
        false
    }
}
