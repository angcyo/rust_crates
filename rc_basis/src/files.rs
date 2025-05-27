use std::fs::File;
use std::io::{Read, Write};

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/05/26
///

/// 确保文件对应的文件夹存在
pub fn ensure_dir_exist(file_path: &str) {
    let dir = std::path::Path::new(file_path);
    if !dir.exists() {
        std::fs::create_dir_all(dir).unwrap();
    }
}

pub fn ensure_parent_dir_exist(file_path: &str) {
    let dir = std::path::Path::new(file_path).parent().unwrap();
    if !dir.exists() {
        std::fs::create_dir_all(dir).unwrap();
    }
}

/// 取"/"最后一节路径
pub fn last_path(file_path: &str) -> String {
    file_path.split("/").last().unwrap().to_string()
}

/// 将字符串保存到文件
pub fn save_string_to_file(file_path: &str, content: &str) -> anyhow::Result<()> {
    ensure_parent_dir_exist(file_path);
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// 从文件中读取字节数据
pub fn read_file_bytes(file_path: &str) -> anyhow::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
