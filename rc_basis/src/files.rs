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

/// @return 路径对象[std::path::Path]
pub fn ensure_parent_dir_exist(file_path: &str) -> &std::path::Path {
    //let path_buf = std::path::PathBuf::from(file_path);
    //path_buf.as_path();
    //path_buf.to_path_buf();
    let path = std::path::Path::new(file_path);
    let dir = path.parent().unwrap();
    if !dir.exists() {
        std::fs::create_dir_all(dir).unwrap();
    }
    path
}

/// 取"/"最后一节路径
pub fn last_path(file_path: &str) -> String {
    file_path.split("/").last().unwrap().to_string()
}

/// 将字符串保存到文件
///
/// @return 保存后的文件路径
pub fn save_string_to_file(file_path: &str, content: &str) -> anyhow::Result<String> {
    save_bytes_to_file(file_path, content.as_bytes())
}

/// 保存字节数据到文件
///
/// @return 保存后的文件路径
pub fn save_bytes_to_file(file_path: &str, bytes: &[u8]) -> anyhow::Result<String> {
    let path = ensure_parent_dir_exist(file_path);
    let mut file = File::create(file_path)?;
    file.write_all(bytes)?;
    Ok(path.canonicalize()?.to_str().unwrap_or("").to_string())
}

/// 从文件中读取字节数据
pub fn read_file_bytes(file_path: &str) -> anyhow::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// 调用系统程序, 打开文件
pub fn open_file_with_sys(path: &String) -> &String {
    #[cfg(target_os = "windows")]
    {
        // Windows使用 start 命令
        std::process::Command::new("cmd")
            .args(&["/C", "start", "", path])
            .spawn()
            .unwrap();
    }

    #[cfg(target_os = "macos")]
    {
        // macOS 使用 open 命令
        std::process::Command::new("open")
            .arg(path)
            .spawn()
            .unwrap();
    }

    path
}
