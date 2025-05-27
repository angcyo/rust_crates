///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/05/27
///

/// 定义一个RGBA颜色接口
pub trait RgbaColor {
    /// 创建一个RGBA颜色
    fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self;

    //--
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
    fn a(&self) -> u8;

    //--

    /// 混合另一个颜色
    fn mix(&self, other: &Self) -> Self;
}

impl RgbaColor for u32 {
    fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
        (r as u32) << 24 | (g as u32) << 16 | (b as u32) << 8 | (a as u32)
    }

    fn r(&self) -> u8 {
        ((self >> 24) & 0xffu32) as u8
    }

    fn g(&self) -> u8 {
        ((self >> 16) & 0xffu32) as u8
    }

    fn b(&self) -> u8 {
        ((self >> 8) & 0xffu32) as u8
    }

    fn a(&self) -> u8 {
        (self & 0xffu32) as u8
    }

    fn mix(&self, other: &Self) -> u32 {
        let (sr, sg, sb, sa) = (self.r(), self.g(), self.b(), self.a());
        let (dr, dg, db, da) = (other.r(), other.g(), other.b(), other.a());

        // 转为 0.0~1.0
        let (sr, sg, sb, sa) = (
            sr as f32 / 255.0,
            sg as f32 / 255.0,
            sb as f32 / 255.0,
            sa as f32 / 255.0,
        );
        let (dr, dg, db, da) = (
            dr as f32 / 255.0,
            dg as f32 / 255.0,
            db as f32 / 255.0,
            da as f32 / 255.0,
        );

        // 计算输出 alpha
        let out_a = sa + da * (1.0 - sa);

        // 防止除0
        let (out_r, out_g, out_b) = if out_a == 0.0 {
            (0.0, 0.0, 0.0)
        } else {
            (
                (sr * sa + dr * da * (1.0 - sa)) / out_a,
                (sg * sa + dg * da * (1.0 - sa)) / out_a,
                (sb * sa + db * da * (1.0 - sa)) / out_a,
            )
        };

        u32::from_rgba(
            (out_r * 255.0).round() as u8,
            (out_g * 255.0).round() as u8,
            (out_b * 255.0).round() as u8,
            (out_a * 255.0).round() as u8,
        )
    }
}
