///
/// @author <a href="mailto: angcyo@126.com">angcyo</a> \
/// @date 2025/06/04
///

/// 使用正则替换传入的字符串
pub fn regex_replace_string(scr: &str, regex: &str, replace: &str) -> String {
    let re = regex::Regex::new(regex).unwrap();
    let replaced = re.replace_all(scr, replace);
    replaced.to_string()
}
