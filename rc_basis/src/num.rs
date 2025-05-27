use rand::distr::uniform::{SampleRange, SampleUniform};
use rand::Rng;

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/05/27
///

/// 随机生成一个浮点数
pub fn random_f64() -> f64 {
    let mut rng = rand::rng();
    rng.random()
}

/// 在一个范围内随机
/// `random_range(0..100)`
pub fn random_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    let mut rng = rand::rng();
    rng.random_range(range)
}

/// 求2个f32的最小值
pub fn min_f32(a: f32, b: f32) -> f32 {
    if a < b {
        return a;
    }
    b
}

/// 求2个f32的最大值
pub fn max_f32(a: f32, b: f32) -> f32 {
    if a > b {
        return a;
    }
    b
}
