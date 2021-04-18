#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::needless_lifetimes)]

use std::{
    f64::consts::PI,
    fmt,
    ops::{Add, Mul, Sub},
};

use crate::DEFAULT_FONT_FAMILY;

use super::StyleOption;
use dataflow::*;
use animate::Pattern;
use primitives::{CanvasContext, Point, TextStyle, TextWeight};

/// Converts [angle] in radians to degrees.
pub fn rad2deg(angle: f64) -> f64 {
    angle * 180.0 / PI
}

/// Converts [angle] in degrees to radians.
pub fn deg2rad(angle: f64) -> f64 {
    angle * PI / 180.0
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
pub fn find_max_value<'a, M, D>(stream: &DataStream<'a, M, D>) -> D
where
    M: fmt::Display,
    D: fmt::Display + Copy + Into<f64> + Ord + Default,
{
    let mut result: Option<D> = None;
    for channel in stream.meta.iter() {
        let channel_index = channel.tag as u64;
        for frame in stream.frames.iter() {
            if let Some(value) = frame.data.get(channel_index) {
                match result {
                    Some(max_value) => {
                        if *value > max_value {
                            result = Some(*value);
                        }
                    }
                    None => result = Some(*value),
                }
            }
        }
    }

    result.unwrap_or_default()
}

/// Returns the minimum value in a [DataTable].
pub fn find_min_value<'a, M, D>(stream: &DataStream<'a, M, D>) -> D
where
    M: fmt::Display,
    D: fmt::Display + Copy + Into<f64> + Ord + Default,
{
    let mut result: Option<D> = None;
    for channel in stream.meta.iter() {
        let channel_index = channel.tag as u64;
        for frame in stream.frames.iter() {
            if let Some(value) = frame.data.get(channel_index) {
                match result {
                    Some(min_value) => {
                        if *value < min_value {
                            error!("ASSIGN MIN VALUE {}", value);
                            result = Some(*value);
                        }
                    }
                    None => result = Some(*value),
                }
            }
        }
    }

    result.unwrap_or_default()
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

pub fn calculate_max_text_width<C>(
    ctx: &C,
    style: &StyleOption,
    texts: &[String],
) -> f64 
where
    C: CanvasContext<Pattern>
{
    let mut result = 0.0;
    ctx.set_font(
        style.fontfamily.unwrap_or(DEFAULT_FONT_FAMILY),
        style.fontstyle.unwrap_or(TextStyle::Normal),
        TextWeight::Normal,
        style.fontsize.unwrap_or(12.),
    );

    for text in texts.iter() {
        let width = ctx.measure_text(text).width;
        if result < width {
            result = width
        }
    }
    result
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
) -> (Point<f64>, Point<f64>) {
    let d21 = p2.distance_to(p1);
    let d23 = p2.distance_to(p3);
    let fa = t * d21 / (d21 + d23);
    let fb = t * d23 / (d21 + d23);
    let v13 = p3 - p1;
    let cp1 = p2 - v13 * fa;
    let cp2 = p2 + v13 * fb;
    (cp1, cp2)
}

/// Returns the number of decimal digits of [value].
pub fn get_decimal_places(value: f64) -> usize {
    if value.fract() == 0. {
        return 0;
    }

    // See https://code.google.com/p/dart/issues/detail?id=1533
    let tmp = format!("{}", value);
    let split: Vec<&str> = tmp.split('.').collect();
    split.get(1).unwrap().len()
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
