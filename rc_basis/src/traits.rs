use regex::Regex;
use std::str::FromStr;

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a> \
/// @date 2025/05/28
///

/// 尝试将字符串转换成指定类型
pub trait Parse {
    /// 带关联类型的 trait
    type Error;
    fn parse(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
/// 我们约束 T 必须同时实现了 FromStr 和 Default
/// 这样在使用的时候我们就可以用这两个 trait 的方法了
impl<T> Parse for T
where
    T: FromStr + Default,
{
    /// 定义关联类型 Error 为 String
    type Error = String;

    fn parse(s: &str) -> Result<Self, Self::Error> {
        let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        if let Some(captures) = re.captures(s) {
            // 当出错时我们返回 Err(String)
            captures
                .get(0)
                .map_or(Err("failed to capture".to_string()), |s| {
                    s.as_str()
                        .parse()
                        .map_err(|_err| "failed to parse captured string".to_string())
                })
        } else {
            Err("failed to parse string".to_string())
        }
        /*// 生成一个创建缺省值的闭包，这里主要是为了简化后续代码
        // Default::default() 返回的类型根据上下文能推导出来，是 Self
        // 而我们约定了 Self，也就是 T 需要实现 Default trait
        let d = || Default::default();
        if let Some(captures) = re.captures(s) {
            captures
                .get(0)
                .map_or(d(), |s| s.as_str().parse().unwrap_or(d()))
        } else {
            d()
        }*/
    }
}
#[cfg(test)]
mod tests {
    use crate::traits::Parse;

    #[test]
    fn test_parse_should_work() {
        assert_eq!(u32::parse("123abcd"), Ok(123));
        assert_eq!(u32::parse("123.45abcd"), Ok(0));
        assert_eq!(f64::parse("123.45abcd"), Ok(123.45));
        assert_eq!(f64::parse("abcd"), Ok(0f64));
    }

    /// 正则字符串替换
    #[test]
    fn test_regex_replace() {
        let origin = "G1 X70.482 Y59.953 F7800";
        let regex = r"G1 (X([-+]?\d*\.?\d+)+ Y([-+]?\d*\.?\d+)+ F([-+]?\d*\.?\d+)+)";
        let re = regex::Regex::new(regex).unwrap();
        let replaced = re.replace_all(origin, "G0 $1");
        println!("{}", replaced);//G0 X70.482 Y59.953 F7800

        let replaced = re.replace_all(origin, "$0");
        println!("{}", replaced);//G1 X70.482 Y59.953 F7800

        let replaced = re.replace_all(origin, "$2");
        println!("{}", replaced);//70.482

        let replaced = re.replace_all(origin, "$3");
        println!("{}", replaced);//59.953

        let replaced = re.replace_all(origin, "$4");
        println!("{}", replaced);//7800
        
        let replaced = re.replace_all(origin, "$5");
        println!("{}", replaced);//

        let replaced = re.replace_all(origin, "$6");
        println!("{}", replaced.to_string());//
    }
}
