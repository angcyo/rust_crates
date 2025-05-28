///
/// @author <a href="mailto: angcyo@126.com">angcyo</a> \
/// @date 2025/05/27
///

/// 使用 `macro_rules!` 的 声明宏（declarative macro），和三种 过程宏（procedural macro）：
/// - 自定义 #[derive] 宏，用于在结构体和枚举上通过添加 derive 属性生成代码
/// - 类属性宏，定义可用于任意项的自定义属性
/// - 类函数宏，看起来像函数，但操作的是作为其参数传递的 token
/// 
/// https://kaisery.github.io/trpl-zh-cn/ch20-05-macros.html
#[macro_export]
macro_rules! ptl {
    () => {
        eprintln!("[{}:{}:{}]", file!(), line!(), column!())
    };
      ($($arg:tt)*) => {{
        eprintln!("[{}:{}:{}]->{}", file!(), line!(), column!(), format!($($arg)*))
    }};
}

/// 定义一个宏, 用来获取当前当前调用的文件, 文件行/列号
/// `xxx/src/main.rs:25:31`
#[macro_export]
macro_rules! file_line {
    () => {
        format!("{}:{}:{}", file!(), line!(), column!())
    };
}

/// 要在 Rust 中获取 Cargo.toml 文件中的版本信息，通常我们可以利用 env! 宏来在编译时获取版本。
/// 使用 env! 宏获取版本信息
#[macro_export]
macro_rules! version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! ok {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                return Err(err);
            }
        }
    };
}

#[macro_export]
macro_rules! some {
    ($expr:expr) => {
        match $expr {
            Some(val) => val,
            None => {
                return None;
            }
        }
    };
}

/// 定义一个宏, 用来成功退出程序
/// 0: 表示程序成功完成，按照约定，这是一个“正常”退出代码。
#[macro_export]
macro_rules! exit_success {
    () => {
        std::process::exit(0)
    };
}

/// 非 0 的值: 通常表示各种错误或异常情况。例如：
///   1: 通常表示一般错误。
///   2: 通常表示命令行参数错误。
///   其他非 0 的值可以根据具体的应用程序和需求来定义。
#[macro_export]
macro_rules! exit_failure {
    () => {
        std::process::exit(1)
    };
}
