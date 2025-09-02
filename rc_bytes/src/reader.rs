use std::cmp::min;

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/09/02
///
/// 字节读取工具
pub struct ByteReader<'a> {
    /// 需要读取的字节数组
    bytes: &'a [u8],
    /// 当前读取的位置
    offset: usize,
}

/// 实现迭代器
impl<'a> Iterator for ByteReader<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.bytes.len() {
            return None;
        }
        let byte = self.bytes[self.offset];
        self.offset += 1;
        Some(byte)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.bytes.len() - self.offset;
        (remaining, Some(remaining))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.offset += n;
        self.next()
    }
}

impl<'a> ByteReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    /// 是否读取结束
    fn _is_end(&self) -> bool {
        self.offset >= self.bytes.len()
    }

    /// 偏移指定字节大小的位置
    pub fn offset(&mut self, offset: usize) {
        self.offset += offset;
    }

    /// 读取指定字节大小的字节数组
    fn read_bytes(&mut self, size: usize) -> &'a [u8] {
        let start = self.offset;
        let max_size = self.bytes.len() - start;
        let end = self.offset + min(max_size, size);
        self.offset = end;
        &self.bytes[start..end]
    }

    /// 读取一个有符号的整数
    /// - 支持1, 2, 4, 8字节数
    /// - 支持大小端序
    ///
    /// - [size] 待写入的数值字节数, 1, 2, 4, 8字节
    /// - [le] 是否使用小端序
    pub fn read_int(&mut self, size: usize, le: bool) -> i64 {
        let bytes = self.read_bytes(size);
        if le {
            bytes
                .iter()
                .fold(0, |acc, &x| acc | (x as i64) << (size - 1) * 8)
        } else {
            bytes
                .iter()
                .rev()
                .fold(0, |acc, &x| acc | (x as i64) << (size - 1) * 8)
        }
    }

    pub fn read_int16(&mut self, le: bool) -> i16 {
        let bytes = self.read_bytes(2);
        if le {
            i16::from_le_bytes(bytes.try_into().unwrap())
        } else {
            i16::from_be_bytes(bytes.try_into().unwrap())
        }
    }
    pub fn read_int8(&mut self, le: bool) -> i8 {
        let bytes = self.read_bytes(1);
        if le {
            i8::from_le_bytes(bytes.try_into().unwrap())
        } else {
            i8::from_be_bytes(bytes.try_into().unwrap())
        }
    }

    pub fn read_int32(&mut self, le: bool) -> i32 {
        let bytes = self.read_bytes(4);
        if le {
            i32::from_le_bytes(bytes.try_into().unwrap())
        } else {
            i32::from_be_bytes(bytes.try_into().unwrap())
        }
    }

    pub fn read_int64(&mut self, le: bool) -> i64 {
        let bytes = self.read_bytes(8);
        if le {
            i64::from_le_bytes(bytes.try_into().unwrap())
        } else {
            i64::from_be_bytes(bytes.try_into().unwrap())
        }
    }

    /// [read_int] 的十六进制形式输出
    pub fn read_hex_int(&mut self, size: usize, le: bool) -> String {
        let bytes = self.read_bytes(size);
        if le {
            bytes
                .iter()
                .rev()
                .map(|x| format!("{:02X}", x))
                .collect::<String>()
        } else {
            bytes
                .iter()
                .map(|x| format!("{:02X}", x))
                .collect::<String>()
        }
    }

    /// 读取有符号的浮点数
    pub fn read_float32(&mut self, le: bool) -> f32 {
        let bytes = self.read_bytes(4);
        if le {
            //小端序
            f32::from_le_bytes(bytes.try_into().unwrap())
        } else {
            //大端序
            f32::from_be_bytes(bytes.try_into().unwrap())
        }
    }

    /// 读取有符号的浮点数
    pub fn read_float64(&mut self, le: bool) -> f64 {
        let bytes = self.read_bytes(8);
        if le {
            //小端序
            f64::from_le_bytes(bytes.try_into().unwrap())
        } else {
            //大端序
            f64::from_be_bytes(bytes.try_into().unwrap())
        }
    }

    /// 读取utf8字符串, 直到遇到0字节
    pub fn read_utf8(&mut self) -> String {
        let mut bytes: Vec<u8> = Vec::new();
        loop {
            let byte = self.next();
            match byte {
                Some(byte) => {
                    if byte == 0 {
                        break;
                    }
                    bytes.push(byte);
                }
                None => break,
            }
        }
        // utf8 解码
        String::from_utf8(bytes).unwrap()
    }
}
