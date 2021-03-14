#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::{collections::HashMap, f64::consts::PI, fmt};

use super::StyleOption;
use dataflow::*;
use primitives::{CanvasContext, Point};

/// Converts [angle] in radians to degrees.
pub fn rad2deg(angle: f64) -> f64 {
    angle * 180.0 / PI
}

/// Converts [angle] in degrees to radians.
pub fn deg2rad(angle: f64) -> f64 {
    angle * PI / 180.0
}

/// Returns a linear interpolated value based on the start value [start], the
/// end value [end], and the interpolation factor [f].
///
/// [start] and [end] can be of any type which defines three operators +, - , *.
pub fn lerp(start: f64, end: f64, f: f64) -> f64 {
    start + (end - start) * f
}

/// Tests if [value] is in range [min]..[max], inclusively.
pub fn is_in_range(value: f64, min: f64, max: f64) -> bool {
    value >= min && value <= max
}

pub fn polar2cartesian(center: &Point<f64>, radius: f64, angle: f64) -> Point<f64> {
    let x = center.x + radius * (angle).cos();
    let y = center.y + radius * (angle).sin();
    Point::new(x, y)
}

/// Rounds [value] to [places] decimal places.
pub fn round2places(value: f64, places: usize) -> f64 {
    let p = f64::powf(10.0, places as f64);
    let value = value * p;
    value.round() / p
}

/// Converts [hexColor] and [alpha] to an RGBA color string.
pub fn hex2rgba(hex_color: &str, alpha: f64) -> String {
    // let componentLength = (hexColor.len() / 3).trunc();
    // let i = 1 + componentLength;
    // let j = i + componentLength;
    // let r = int.parse(hexColor.substring(1, i), radix: 16);
    // let g = int.parse(hexColor.substring(i, j), radix: 16);
    // let b = int.parse(hexColor.substring(j), radix: 16);
    // if (componentLength == 1) {
    //   r += r << 4;
    //   g += g << 4;
    //   b += b << 4;
    // }
    // return "rgba($r, $g, $b, $alpha)";
    unimplemented!();
}

/// Returns the hyphenated version of [s].
pub fn hyphenate(s: &str) -> String {
    // return s.replaceAllMapped(RegExp("[A-Z]"), (Match m) {
    //   return "-" + m[0].toLowerCase();
    // });
    unimplemented!();
}

/// Returns the maximum value in a [DataTable].
pub fn find_max_value<'a, M, D>(table: &DataStream<'a, M, D>) -> D
where
    M: fmt::Display,
    D: fmt::Display,
{
    // let maxValue = f64::NEG_INFINITY;
    // for (let row in table.rows) {
    //   for (let col in table.columns) {
    //     let value = row[col.index];
    //     if (value is num && value > maxValue) maxValue = value;
    //   }
    // }
    // maxValue
    unimplemented!();
}

/// Returns the minimum value in a [DataTable].
pub fn find_min_value<'a, M: fmt::Display, D: fmt::Display>(table: DataStream<'a, M, D>) -> f64 {
    // let minValue = f64::INFINITY;
    // for (let row in table.rows) {
    //   for (let col in table.columns) {
    //     let value = row[col.index];
    //     if (value is num && value < minValue) minValue = value;
    //   }
    // }
    // minValue
    unimplemented!();
}

/// Calculates a nice axis interval given
/// - the axis range [range]
/// - the desired number of steps [targetSteps]
/// - and the minimum interval [min_interval]
pub fn calculate_interval(range: f64, target_steps: usize, min_interval: Option<f64>) -> f64 {
    let interval = range / target_steps as f64;
    let mag = interval.log10().floor();
    let mut mag_pow = f64::powf(10.0, mag);
    if let Some(min_interval) = min_interval {
        mag_pow = mag_pow.max(min_interval);
    }
    let mut msd = (interval / mag_pow).round();
    if msd > 5. {
        msd = 10.;
    } else if msd > 2. {
        msd = 5.;
    } else if msd == 0. {
        msd = 1.;
    }
    msd * mag_pow
}

pub fn calculate_max_text_width<C: CanvasContext>(
    ctx: C,
    font: String,
    texts: Vec<String>,
) -> f64 {
    // let result = 0.0;
    // ctx.font = font;
    // for (let text in texts) {
    //   let width = ctx.measure_text(text).width;
    //   if (result < width) result = width;
    // }
    // result
    unimplemented!();
}

/// Calculates the controls for [p2] given the previous point [p1], the next
/// point [p3], and the curve tension [t];
///
/// Returns a list that contains two control points for [p2].
///
/// Credit: Rob Spencer (http://scaledinnovation.com/analytics/splines/aboutSplines.html)
pub fn calculate_control_points(
    p1: Point<f64>,
    p2: Point<f64>,
    p3: Point<f64>,
    t: f64,
) -> Vec<Point<f64>> {
    let d21 = p2.distance_to(p1);
    let d23 = p2.distance_to(p3);
    let fa = t * d21 / (d21 + d23);
    let fb = t * d23 / (d21 + d23);
    let v13 = p3 - p1;
    let cp1 = p2 - v13 * fa;
    let cp2 = p2 + v13 * fb;
    vec![cp1, cp2]
}

/// Returns the number of decimal digits of [value].
pub fn get_decimal_places(value: f64) -> usize {
    if value.fract() == 0. {
        return 0;
    }
    // See https://code.google.com/p/dart/issues/detail?id=1533
    // return "$value.0".split(".")[1].len();
    unimplemented!()
}

// /// Returns a CSS font string given a map that contains at least three keys:
// /// `fontStyle`, `font_size`, and `fontFamily`.
// pub fn get_font(opt: &StyleOption) -> String {
//     if let Some(style) = opt.font_style {
//         if let Some(size) = opt.font_size {
//             if let Some(family) = opt.font_family {
//                 return format!("{} {}px {}", style, size, family);
//             }
//             return format!("{} {}px", style, size);
//         }
//         return format!("{}", style);
//     }

//     if let Some(size) = opt.font_size {
//         if let Some(family) = opt.font_family {
//             return format!("{}px {}", size, family);
//         }
//         return format!("{}px", size);
//     }

//     if let Some(family) = opt.font_family {
//         return format!("{}", family);
//     }

//     "".into()
// }

// pub struct StreamSubscriptionTracker {
//     subs: Vec<StreamSubscription>,
// }

// impl StreamSubscriptionTracker {
//     pub fn add(sub: StreamSubscription) {
//         // subs.add(sub);
//     }

//     pub fn clear() {
//         // for (let sub in subs) {
//         //   sub.cancel();
//         // }
//         // subs.clear();
//     }
// }
