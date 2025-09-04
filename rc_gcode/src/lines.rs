use crate::split_path_contours;
use lyon_algorithms::walk::{walk_along_path, RegularPattern, WalkerEvent};
use lyon_path::iterator::PathIterator;

///
/// @author <a href="mailto: angcyo@126.com">angcyo</a>
/// @date 2025/09/04
///
/// 将[Path]处理成一段一段的线段集合数据
///
/// - 所有线段集合
///     - 线段的开始点, 线段中的每个点
///         - x
///         - y
///
/// - [path] 核心入参, 支持多轮廓
/// - [tolerance] 公差
/// - [interval] 是否要间隔进行采样, >0 时生效.
pub fn path_to_lines(
    path: &lyon_path::Path,
    tolerance: f32,
    interval: f32,
) -> Vec<Vec<(f32, f32)>> {
    let mut lines = vec![];
    let mut line = None;

    each_path_line(path, tolerance, interval, |new_line, p| {
        //新的线段
        if new_line && let Some(l) = line.take() {
            lines.push(l);
        }
        line = Some(vec![]);

        //添加点
        if let Some(p) = p {
            if let Some(mut l) = line.take() {
                l.push((p.0, p.1));
            }
        }
    });

    if let Some(l) = line {
        lines.push(l);
    }

    lines
}

/// 枚举路径的线段集合
pub fn each_path_line(
    path: &lyon_path::Path,
    tolerance: f32,
    interval: f32,
    mut on_point: impl FnMut(bool, Option<(f32, f32)>),
) {
    if interval > 0.0 {
        //间隔采样
        let path_vec = split_path_contours(path);
        for path in path_vec.iter() {
            let start = 0.0;
            let mut pattern = RegularPattern {
                callback: &mut |event: WalkerEvent| {
                    //if (event.distance == start) {
                    //每个点, 自成一段线段
                    on_point(true, None);
                    //}
                    on_point(false, Some((event.position.x, event.position.y)));
                    true // Return true to continue walking the path.
                },
                // Invoke the callback above at a regular interval of 3 units.
                interval,
            };
            walk_along_path(path.iter(), start, tolerance, &mut pattern);
            //尾部会有interval范围内的长度接不上
            //let length = approximate_length(path.iter(), tolerance);
            //walk_along_path(path.iter(), length , tolerance, &mut pattern);
        }
    } else {
        path.iter()
            .flattened(tolerance)
            .for_each(|event| match event {
                lyon_path::Event::Begin { at } => {
                    on_point(true, None);
                    on_point(false, Some((at.x, at.y)));
                }
                lyon_path::Event::Line { from, to } => {
                    on_point(false, Some((to.x, to.y)));
                }
                _ => {}
            });
    }
}
