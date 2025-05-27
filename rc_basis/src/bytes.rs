use base64::engine::general_purpose::STANDARD_NO_PAD;
use base64::{DecodeError, Engine};

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/05/27
///

/// 生成一个uuid
/// `03B5916C0B104D619BD43D5148837217`
pub fn uuid() -> String {
    uuid::Uuid::new_v4()
        .to_string()
        .to_uppercase()
        .replace("-", "")
}

/// 将字节数组转换成utf8字符串
pub fn bytes_to_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).to_string()
    //String::from_utf8(bytes.to_vec()).unwrap()
}

/// 将utf8字符串转换成字节数组
pub fn string_to_bytes(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

/// 将字节数组进行base64加密
/// [STANDARD]
/// [STANDARD_NO_PAD]
pub fn base64_encode(bytes: &[u8]) -> String {
    STANDARD_NO_PAD.encode(bytes)
}

/// 将base64的字符串进行解密
pub fn base64_decode(s: &str) -> Result<Vec<u8>, DecodeError> {
    STANDARD_NO_PAD.decode(s)
}

/// 将字节数组进行md5加密
/// `93E11B05413C8F043BFFCFC5C3D6E68B`
pub fn md5_encode(bytes: &[u8]) -> String {
    format!("{:X}", md5::compute(bytes))
}
