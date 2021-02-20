#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::f64::consts::PI;
use crate::{CanvasContext, DataTable, Point};

/// Converts [angle] in radians to degrees.
fn rad2deg(angle: f64) -> f64 {
    angle * 180.0 / PI
}

/// Converts [angle] in degrees to radians.
fn deg2rad(angle: f64) -> f64 {
    angle * PI / 180.0
}

// /// Returns the base-10 logarithm of [value].
// fn log10(value: f64) -> f64 {
//   log(value) / f64::LN_10// ln10
// }

/// Returns a linear interpolated value based on the start value [start], the
/// end value [end], and the interpolation factor [f].
///
/// [start] and [end] can be of any type which defines three operators +, - , *.
fn lerp(start: f64, end: f64, f: f64) -> f64 {
    start + (end - start) * f
}

/// Tests if [value] is in range [min]..[max], inclusively.
fn is_in_range(value: f64, min: f64, max: f64) -> bool {
    value >= min && value <= max
}

// fn polarToCartesian(center: Point, radius: f64, angle: f64) -> Point<f64> {
//   let x = center.x + radius * (angle).cos();
//   let y = center.y + radius * (angle).sin();
//   return Point<f64>(x, y);
// }

/// Rounds [value] to [places] decimal places.
fn round_to_places(value: f64, places: usize) -> f64 {
    let p = f64::powf(10.0, places as f64);
    let value = value * p;
    return value.round() / p;
}

/// Converts [hexColor] and [alpha] to an RGBA color string.
fn hex_to_rgba(hex_color: String, alpha: f64) -> String {
    // let componentLength = hexColor.length ~/ 3;
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
fn hyphenate(s: String) -> String {
    // return s.replaceAllMapped(RegExp("[A-Z]"), (Match m) {
    //   return "-" + m[0].toLowerCase();
    // });
    unimplemented!();
}

/// Returns the maximum value in a [DataTable].
fn find_max_value(table: DataTable) -> f64 {
    // let maxValue = double.negativeInfinity;
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
fn find_min_value(table: DataTable) -> f64 {
    // let minValue = double.infinity;
    // for (let row in table.rows) {
    //   for (let col in table.columns) {
    //     let value = row[col.index];
    //     if (value is num && value < minValue) minValue = value;
    //   }
    // }
    // minValue
    unimplemented!();
}

// /// Calculates a nice axis interval given
// /// - the axis range [range]
// /// - the desired number of steps [targetSteps]
// /// - and the minimum interval [minInterval]
// fn calculateInterval(range: f64, targetSteps: usize, minInterval: f64) -> f64 {
//   let interval = range / targetSteps;
//   let mag = log10(interval).floor();
//   let magPow = (10.0, mag).pow() as f64;
//   if minInterval != null {
//     magPow = (magPow, minInterval).max();
//   }
//   let msd = (interval / magPow).round();
//   if msd > 5 {
//     msd = 10;
//   } else if msd > 2 {
//     msd = 5;
//   } else if msd == 0 {
//     msd = 1;
//   }
//   return msd * magPow;
// }

// fn calculateMaxTextWidth(context: C, font: String, texts: Vec<String>) -> f64 {
//   // let result = 0.0;
//   // context.font = font;
//   // for (let text in texts) {
//   //   let width = context.measureText(text).width;
//   //   if (result < width) result = width;
//   // }
//   // result
//   unimplemented!();
// }

/// Calculates the controls for [p2] given the previous poi64 [p1], the next
/// poi64 [p3], and the curve tension [t];
///
/// Returns a list that contains two control points for [p2].
///
/// Credit: Rob Spencer (http://scaledinnovation.com/analytics/splines/aboutSplines.html)
fn calculate_control_points(p1: Point, p2: Point, p3: Point, t: f64) -> Vec<Point> {
    // let d21 = p2.distanceTo(p1);
    // let d23 = p2.distanceTo(p3);
    // let fa = t * d21 / (d21 + d23);
    // let fb = t * d23 / (d21 + d23);
    // let v13 = p3 - p1;
    // let cp1 = p2 - v13 * fa;
    // let cp2 = p2 + v13 * fb;
    // vec![cp1, cp2]
    unimplemented!()
}

/// Returns the number of decimal digits of [value].
fn get_decimal_places(value: i64) -> i64 {
    // if (value % 1 == 0) return 0;
    // // See https://code.google.com/p/dart/issues/detail?id=1533
    // return "$value.0".split(".")[1].length;
    unimplemented!()
}

/// Deeply merges [map1] and [map2] into a new [Map].
///
/// [map1] must not be `null`.
///
/// If [map2] is `null`, returns [map1].
fn merge_maps(
    map1: HashMap<String, String>,
    map2: HashMap<String, String>,
) -> HashMap<String, String> {
    // if (map2 == null) {
    //   return map1;
    // }

    // let result = {};
    // cb(k, v) {
    //   result[k] = v is Map ? mergeMaps(result[k], v) : v;
    // }

    // map1?.forEach(cb);
    // map2?.forEach(cb);
    // result
    unimplemented!()
}

// struct StreamSubscriptionTracker {
//     subs: Vec<StreamSubscription>,
// }

// impl StreamSubscriptionTracker {
//     fn add(sub: StreamSubscription) {
//         // subs.add(sub);
//     }

//     fn clear() {
//         // for (let sub in subs) {
//         //   sub.cancel();
//         // }
//         // subs.clear();
//     }
// }
