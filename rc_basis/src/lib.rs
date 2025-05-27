pub mod bytes;
pub mod colors;
pub mod files;
pub mod macros;
pub mod num;

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/27
///
pub use anyhow;
use chrono::Utc;


/// 获取当前工作目录
pub fn get_current_dir() -> String {
    std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

/// 获取当前13位毫秒时间戳
pub fn now_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

/// 获取当前日期的字符串
pub fn now_date_time() -> String {
    Utc::now().to_string()
}

//--

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        println!("Hello, test!");
        assert_eq!(2 + 2, 4);
    }
}
