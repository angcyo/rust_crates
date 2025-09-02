use nalgebra::Matrix3;

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/09/02
///

/// 平移矩阵
pub fn translate_matrix(tx: f32, ty: f32) -> Matrix3<f32> {
    Matrix3::new(1.0, 0.0, tx, 0.0, 1.0, ty, 0.0, 0.0, 1.0)
}

/// 缩放矩阵
pub fn scale_matrix(sx: f32, sy: f32) -> Matrix3<f32> {
    Matrix3::new(sx, 0.0, 0.0, 0.0, sy, 0.0, 0.0, 0.0, 1.0)
}

/// 旋转矩阵
pub fn rotate_matrix(theta: f32) -> Matrix3<f32> {
    let (sin, cos) = theta.sin_cos();
    Matrix3::new(cos, -sin, 0.0, sin, cos, 0.0, 0.0, 0.0, 1.0)
}

/// 绕着指定点旋转的矩阵
/// - [theta] 旋转角度, 单位: 弧度
pub fn rotate_center_matrix(theta: f32, cx: f32, cy: f32) -> Matrix3<f32> {
    let (sin, cos) = theta.sin_cos();
    // 平移到原点
    let to_origin = Matrix3::new(1.0, 0.0, -cx, 0.0, 1.0, -cy, 0.0, 0.0, 1.0);
    // 旋转
    let rotate = Matrix3::new(cos, -sin, 0.0, sin, cos, 0.0, 0.0, 0.0, 1.0);
    // 平移回去
    let back = Matrix3::new(1.0, 0.0, cx, 0.0, 1.0, cy, 0.0, 0.0, 1.0);
    back * rotate * to_origin
}

/// 角度转弧度
/// ```
///  let a = deg_to_rad(0.0);  // 0.0
///  let a = deg_to_rad(45.0); //0.7853981634
///  let a = deg_to_rad(90.0); //1.5707963268
///  let a = deg_to_rad(135.0); //2.3561944902
///  let a = deg_to_rad(180.0); //3.1415926536
///  let a = deg_to_rad(225.0); //4.1887902048
///  let a = deg_to_rad(270.0); //4.7123889804
///  let a = deg_to_rad(315.0); //5.4977871435
///  let a = deg_to_rad(360.0); //6.2831853072
/// ```
pub fn deg_to_rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

/// 弧度转角度
///
/// ```
///  let a = rad_to_deg(0.0);  // 0.0
///  let a = rad_to_deg(0.7853981634); //45.0
///  let a = rad_to_deg(1.5707963268); //90.0
///  let a = rad_to_deg(2.3561944902); //135.0
///  let a = rad_to_deg(3.1415926536); //180
///  let a = rad_to_deg(4.1887902048); //225.0
///  let a = rad_to_deg(4.7123889804); //270.0
///  let a = rad_to_deg(5.4977871435); //315.0
///  let a = rad_to_deg(6.2831853072); //360.0
/// ```
pub fn rad_to_deg(rad: f32) -> f32 {
    rad * 180.0 / std::f32::consts::PI
}
