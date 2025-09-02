pub mod reader;
pub mod writer;

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/09/02
///
/// 字节读写工具类

/// 十进制转成十六进制
/// - [dec] 十进制数字, 占8个字节
/// - [size] 需要转换几个字节
pub fn dec_to_hex(dec: i64, size: usize) -> String {
    let bytes = dec.to_le_bytes();
    let bytes = &bytes[0..size];
    let bytes = bytes.iter().rev();
    bytes.map(|x| format!("{:02X}", x)).collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::dec_to_hex;
    use crate::reader::ByteReader;
    use crate::writer::ByteWriter;

    #[test]
    fn it_works() {}

    #[test]
    fn test_byte_reader() {
        println!("{:?}", dec_to_hex(0xffeeccaa, 8));
    }

    #[test]
    fn test_byte_writer() {
        let mut writer = ByteWriter::new(usize::MAX);
        writer.write_int(0xffeeccaa, 2, false);
        writer.write_int(0xffeeccaa, 4, false);
        writer.write_int(0xffeeccaa, 8, true);
        writer.write_int16(-1919, false);
        writer.write_int32(-191919, false);
        writer.write_int64(-19191919, true);
        writer.write_string("angcyo", true);
        writer.write_float64(-19.1991, false);
        writer.write_float64(-19.1991, true);
        writer.write_float32(-19.0, false);
        writer.write_float32(-19.0, true);

        let mut reader = ByteReader::new(writer.bytes());
        println!("{:?}", reader.read_hex_int(2, false));
        println!("{:?}", reader.read_hex_int(4, false));
        println!("{:?}", reader.read_hex_int(8, true));
        println!("{:?}", reader.read_int16(false));
        println!("{:?}", reader.read_int32(false));
        println!("{:?}", reader.read_int64(true));
        println!("{:?}", reader.read_utf8());
        println!("{:?}", reader.read_float64(false));
        println!("{:?}", reader.read_float64(true));
        println!("{:?}", reader.read_float32(false));
        println!("{:?}", reader.read_float32(true));

        /*writer.write_int(100, 1, false);
        writer.write_int(100, 2, false);
        writer.write_int(100, 4, false);
        writer.write_int(100, 8, false);

        writer.write_int(100, 1, true);
        writer.write_int(100, 2, true);
        writer.write_int(100, 4, true);
        writer.write_int(100, 8, true);*/

        println!("{:?}", writer.bytes());
        println!("bytes:{}", writer.bytes().len());
    }
}
