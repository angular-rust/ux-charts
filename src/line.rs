#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::{collections::HashMap, fmt};
use ux_primitives::{canvas::*, math::*};

use crate::*;

#[derive(Default, Clone)]
struct PointEntity {
    // Chart chart,
    color: String,
    highlight_color: String,
    formatted_value: String,
    index: usize,
    old_value: f64,
    value: f64,

    old_x: f64,
    old_y: f64,
    // oldCp1: Point,
    // oldCp2: Point,
    old_point_radius: f64,

    // /// The first control point.
    // cp1: Point,

    // /// The second control point.
    // cp2: Point,
    x: f64,

    y: f64,

    point_radius: f64,
}

impl PointEntity {
    fn as_point(&self) -> Point<f64> {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

/// A point in a line chart.
impl<C> Drawable<C> for PointEntity
where
    C: CanvasContext,
{
    fn draw(&self, ctx: C, percent: f64, highlight: bool) {
        // let cx = lerp(oldX, x, percent);
        // let cy = lerp(oldY, y, percent);
        // let pr = lerp(oldPointRadius, pointRadius, percent);
        // if (highlight) {
        //   ctx.fillStyle = highlight_color;
        //   ctx.begin_path();
        //   ctx.arc(cx, cy, 2 * pr, 0, TAU);
        //   ctx.fill();
        // }
        // ctx.begin_path();
        // ctx.arc(cx, cy, pr, 0, TAU);
        // ctx.fill();
        // ctx.stroke();
        unimplemented!()
    }
}

impl Entity for PointEntity {
    fn free(&mut self) {
        // chart = null;
    }

    fn save(&self) {
        // self.old_x = self.x;
        // self.old_y = self.y;
        // // self.old_cp1 = self.cp1;
        // // self.old_cp2 = self.cp2;
        // self.old_point_radius = self.point_radius;
        // self.old_value = self.value;
    }
}

pub struct LineChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
{
    x_axis_top: f64,
    y_axis_left: f64,
    x_axis_length: f64,
    y_axis_length: f64,
    x_label_max_width: f64,
    y_label_max_width: f64,
    x_label_rotation: f64, // 0..90
    x_label_step: i64,
    x_label_hop: f64, // Distance between two consecutive x-axis labels.
    y_label_hop: f64, // Distance between two consecutive x-axis labels.
    x_title_box: Rectangle<f64>,
    y_title_box: Rectangle<f64>,
    x_title_center: Point<f64>,
    y_title_center: Point<f64>,
    x_labels: Vec<String>,
    y_labels: Vec<String>,
    y_interval: f64,
    y_max_value: f64,
    y_min_value: f64,
    y_range: f64,

    /// The horizontal offset of the tooltip with respect to the vertical line
    /// passing through an x-axis label.
    tooltip_offset: f64,

    y_label_formatter: Option<ValueFormatter>,
    average_y_values: Vec<f64>,

    x_label_offset_factor: f64, // = .5;

    base: BaseChart<'a, C, PointEntity, M, D, LineChartOptions<'a>>,
}

impl<'a, C, M, D> LineChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
{
    pub fn new(options: LineChartOptions<'a>) -> Self {
        Self {
            x_axis_top: 0.0,
            y_axis_left: 0.0,
            x_axis_length: 0.0,
            y_axis_length: 0.0,
            x_label_max_width: 0.0,
            y_label_max_width: 0.0,
            x_label_rotation: 0.0, // 0..90
            x_label_step: 0,
            x_label_hop: 0.0, // Distance between two consecutive x-axis labels.
            y_label_hop: 0.0, // Distance between two consecutive x-axis labels.
            x_title_box: Default::default(),
            y_title_box: Default::default(),
            x_title_center: Default::default(),
            y_title_center: Default::default(),
            x_labels: Vec::new(),
            y_labels: Vec::new(),
            y_interval: 0.0,
            y_max_value: 0.0,
            y_min_value: 0.0,
            y_range: 0.0,
            tooltip_offset: 0.0,       
            y_label_formatter: None,
            average_y_values: Vec::new(),
            x_label_offset_factor: 0.0, // = .5;
            base: BaseChart::new(options),
        }
    }

    /// Returns the x coordinate of the x-axis label at [index].
    fn x_label_x(&self, index: usize) -> f64 {
        self.y_axis_left + self.x_label_hop * ((index as f64) + self.x_label_offset_factor)
    }

    /// Returns the y-coordinate corresponding to the data point [value] and
    /// the animation percent [percent].
    fn value_to_y(&self, value: f64) -> f64 {
        // value != null
        //   ? x_axis_top - (value - y_min_value) / y_range * y_axis_length
        //   : x_axis_top;
        unimplemented!()
    }

    // TODO: Separate y-axis stuff into a separate method.
    fn calculate_drawing_sizes(&self) {
            self.base.calculate_drawing_sizes();

        //     // y-axis min-max.

        //     self.y_max_value = self.base.options.y_axis.max_value ?? f64::NEG_INFINITY;
        //     y_max_value = max(y_max_value, findMaxValue(data_table));
        //     if (y_max_value == f64::NEG_INFINITY) y_max_value = 0.0;

        //     y_min_value = self.base.options.y_axis.min_value ?? f64::INFINITY;
        //     y_min_value = min(y_min_value, findMinValue(data_table));
        //     if (y_min_value == f64::INFINITY) y_min_value = 0.0;

        //     y_interval = self.base.options.y_axis.interval;
        //     let minInterval = self.base.options.y_axis.min_interval;

        //     if (y_interval == null) {
        //       if (y_min_value == y_max_value) {
        //         if (y_min_value == 0.0) {
        //           y_max_value = 1.0;
        //           y_interval = 1.0;
        //         } else if (y_min_value == 1.0) {
        //           y_min_value = 0.0;
        //           y_interval = 1.0;
        //         } else {
        //           y_interval = y_min_value * .25;
        //           y_min_value -= y_interval;
        //           y_max_value += y_interval;
        //         }
        //         if (minInterval != null) {
        //           y_interval = max(y_interval, minInterval);
        //         }
        //       } else {
        //         y_interval = calculateInterval(y_max_value - y_min_value, 5, minInterval);
        //       }
        //     }

        //     y_min_value = (y_min_value / y_interval).floorToDouble() * y_interval;
        //     y_max_value = (y_max_value / y_interval).ceilToDouble() * y_interval;
        //     yRange = y_max_value - y_min_value;

        //     // y-axis labels.

        //     y_labels = <String>[];
        //     y_label_formatter = self.base.options.y_axis.labels.formatter;
        //     if (y_label_formatter == null) {
        //       let maxDecimalPlaces =
        //           max(getDecimalPlaces(y_interval), getDecimalPlaces(y_min_value));
        //       let numberFormat = NumberFormat.decimalPattern()
        //         ..maximumFractionDigits = maxDecimalPlaces
        //         ..minimumFractionDigits = maxDecimalPlaces;
        //       y_label_formatter = numberFormat.format;
        //     }
        //     let value = y_min_value;
        //     while (value <= y_max_value) {
        //       y_labels.add(y_label_formatter(value));
        //       value += y_interval;
        //     }
        //     y_label_max_width = calculateMaxTextWidth(
        //             context, get_font(self.base.options.y_axis.labels.style), y_labels)
        //         .round();

        //     entity_value_formatter = y_label_formatter;

        //     // Tooltip.

        //     tooltip_value_formatter =
        //         self.base.options.tooltip.value_formatter ?? y_label_formatter;

        //     // x-axis title.

        //     let xTitleLeft = 0;
        //     let xTitleTop = 0;
        //     let xTitleWidth = 0;
        //     let xTitleHeight = 0;
        //     let xTitle = self.base.options.x_axis.title;
        //     if (xTitle["text"] != null) {
        //       context.font = get_font(xTitle["style"]);
        //       xTitleWidth = context.measureText(xTitle["text"]).width.round() +
        //           2 * TITLE_PADDING;
        //       xTitleHeight = xTitle["style"]["fontSize"] + 2 * TITLE_PADDING;
        //       xTitleTop = self.base.series_and_axes_box.bottom - xTitleHeight;
        //     }

        //     // y-axis title.

        //     let yTitleLeft = 0;
        //     let yTitleTop = 0;
        //     let yTitleWidth = 0;
        //     let yTitleHeight = 0;
        //     let yTitle = self.base.options.y_axis.title;
        //     if (yTitle["text"] != null) {
        //       context.font = get_font(yTitle["style"]);
        //       yTitleHeight = context.measureText(yTitle["text"]).width.round() +
        //           2 * TITLE_PADDING;
        //       yTitleWidth = yTitle["style"]["fontSize"] + 2 * TITLE_PADDING;
        //       yTitleLeft = self.base.series_and_axes_box.left;
        //     }

        //     // Axes" size and position.

        //     y_axis_left = self.base.series_and_axes_box.left + y_label_max_width + axis_label_margin;
        //     if (yTitleWidth > 0) {
        //       y_axis_left += yTitleWidth + CHART_TITLE_MARGIN;
        //     } else {
        //       y_axis_left += axis_label_margin;
        //     }

        //     x_axis_length = self.base.series_and_axes_box.right - y_axis_left;

        //     x_axis_top = self.base.series_and_axes_box.bottom;
        //     if (xTitleHeight > 0) {
        //       x_axis_top -= xTitleHeight + CHART_TITLE_MARGIN;
        //     } else {
        //       x_axis_top -= axis_label_margin;
        //     }
        //     x_axis_top -= axis_label_margin;

        //     // x-axis labels and x-axis"s position.

        //     let rowCount = data_table.rows.length;
        //     x_labels = <String>[];
        //     for (let i = 0; i < rowCount; i++) {
        //       x_labels.add(data_table.rows[i][0].to_string());
        //     }
        //     x_label_max_width = calculateMaxTextWidth(
        //         context, get_font(self.base.options.x_axis.labels.style), x_labels);
        //     if (x_label_offset_factor > 0 && rowCount > 1) {
        //       x_label_hop = x_axis_length / rowCount;
        //     } else if (rowCount > 1) {
        //       x_label_hop = x_axis_length / (rowCount - 1);
        //     } else {
        //       x_label_hop = x_axis_length;
        //     }
        //     x_label_rotation = 0;

        //     let fontSize = self.base.options.x_axis.labels.style.font_size;
        //     let maxRotation = self.base.options.x_axis.labels.max_rotation;
        //     let minRotation = self.base.options.x_axis.labels.min_rotation;
        //     const angles = [0, -45, 45, -90, 90];

        //     outer:
        //     for (let step = 1; step <= rowCount; step++) {
        //       let scaledLabelHop = step * x_label_hop;
        //       let minSpacing = max(.1 * scaledLabelHop, 10);
        //       for (let angle in angles) {
        //         if (angle > maxRotation) continue;
        //         if (angle < minRotation) continue;

        //         let absAngleRad = deg2rad(angle).abs();
        //         let labelSpacing = angle == 0
        //             ? scaledLabelHop - x_label_max_width
        //             : scaledLabelHop * sin(absAngleRad) - fontSize;
        //         if (labelSpacing < minSpacing) continue;

        //         x_label_rotation = angle;
        //         x_label_step = step;
        //         x_axis_top -=
        //             x_label_max_width * sin(absAngleRad) + fontSize * cos(absAngleRad);
        //         break outer;
        //       }
        //     }

        //     // Wrap up.

        //     y_axis_length = x_axis_top -
        //         self.base.series_and_axes_box.top -
        //         self.base.options.y_axis.labels.style.font_size ~/ 2;
        //     y_label_hop = y_axis_length / (y_labels.length - 1);

        //     xTitleLeft = y_axis_left + (x_axis_length - xTitleWidth) ~/ 2;
        //     yTitleTop = self.base.series_and_axes_box.top + (y_axis_length - yTitleHeight) ~/ 2;

        //     if (xTitleHeight > 0) {
        // //      x_title_box =
        // //          Rectangle(xTitleLeft, xTitleTop, xTitleWidth, xTitleHeight);
        //       x_title_center =
        //           Point(xTitleLeft + xTitleWidth ~/ 2, xTitleTop + xTitleHeight ~/ 2);
        //     } else {
        // //      x_title_box = null;
        //       x_title_center = null;
        //     }

        //     if (yTitleHeight > 0) {
        // //      y_title_box =
        // //          Rectangle(yTitleLeft, yTitleTop, yTitleWidth, yTitleHeight);
        //       y_title_center =
        //           Point(yTitleLeft + yTitleWidth ~/ 2, yTitleTop + yTitleHeight ~/ 2);
        //     } else {
        // //      y_title_box = null;
        //       y_title_center = null;
        //     }
        unimplemented!()
    }

    // fn data_cell_changed(&self, record: DataCellChangeRecord) {
    //     // if record.columnIndex == 0 {
    //     //   x_labels[record.rowIndex] = record.newValue;
    //     // } else {
    //     //   self.base.data_cell_changed(record);
    //     // }
    // }

    fn draw_axes_and_grid(&self) {
        // // x-axis title.

        // if (x_title_center != null) {
        //   let opt = self.base.options.x_axis.title;
        //   axes_context
        //     ..fillStyle = opt["style"]["color"]
        //     ..font = get_font(opt["style"])
        //     ..textAlign = "center"
        //     ..textBaseline = "middle"
        //     ..fill_text(opt["text"], x_title_center.x, x_title_center.y);
        // }

        // // y-axis title.

        // if (y_title_center != null) {
        //   let opt = self.base.options.y_axis.title;
        //   axes_context
        //     ..save()
        //     ..fillStyle = opt["style"]["color"]
        //     ..font = get_font(opt["style"])
        //     ..translate(y_title_center.x, y_title_center.y)
        //     ..rotate(-PI_2)
        //     ..textAlign = "center"
        //     ..textBaseline = "middle"
        //     ..fill_text(opt["text"], 0, 0)
        //     ..restore();
        // }

        // // x-axis labels.

        // let opt = self.base.options.x_axis.labels;
        // axes_context.fillStyle = opt["style"]["color"];
        // axes_context.font = get_font(opt["style"]);
        // let x = x_label_x(0);
        // let y = x_axis_top + axis_label_margin + opt["style"]["fontSize"];
        // let scaledLabelHop = x_label_step * x_label_hop;

        // if (x_label_rotation == 0) {
        //   axes_context.textAlign = "center";
        //   axes_context.textBaseline = "alphabetic";
        //   for (let i = 0; i < x_labels.length; i += x_label_step) {
        //     axes_context.fill_text(x_labels[i], x, y);
        //     x += scaledLabelHop;
        //   }
        // } else {
        //   axes_context.textAlign = x_label_rotation < 0 ? "right" : "left";
        //   axes_context.textBaseline = "middle";
        //   if (x_label_rotation == 90) {
        //     x += x_label_rotation.sign * (opt["style"]["fontSize"] ~/ 8);
        //   }
        //   let angle = deg2rad(x_label_rotation);
        //   for (let i = 0; i < x_labels.length; i += x_label_step) {
        //     axes_context
        //       ..save()
        //       ..translate(x, y)
        //       ..rotate(angle)
        //       ..fill_text(x_labels[i], 0, 0)
        //       ..restore();
        //     x += scaledLabelHop;
        //   }
        // }

        // // y-axis labels.

        // axes_context
        //   ..fillStyle = self.base.options.y_axis.labels.style.color
        //   ..font = get_font(self.base.options.y_axis.labels.style)
        //   ..textAlign = "right"
        //   ..textBaseline = "middle";
        // x = y_axis_left - axis_label_margin;
        // y = x_axis_top - (self.base.options.y_axis.labels.style.font_size ~/ 8);
        // for (let label in y_labels) {
        //   axes_context.fill_text(label, x, y);
        //   y -= y_label_hop;
        // }

        // // x grid lines - draw bottom up.

        // if (self.base.options.x_axis.grid_line_width > 0) {
        //   axes_context
        //     ..lineWidth = self.base.options.x_axis.grid_line_width
        //     ..strokeStyle = self.base.options.x_axis.grid_line_color
        //     ..begin_path();
        //   y = x_axis_top - y_label_hop;
        //   for (let i = y_labels.length - 1; i >= 1; i--) {
        //     axes_context.moveTo(y_axis_left, y);
        //     axes_context.lineTo(y_axis_left + x_axis_length, y);
        //     y -= y_label_hop;
        //   }
        //   axes_context.stroke();
        // }

        // // y grid lines or x-axis ticks - draw from left to right.

        // let lineWidth = self.base.options.y_axis.grid_line_width;
        // x = y_axis_left;
        // if (x_label_step > 1) {
        //   x = x_label_x(0);
        // }
        // if (lineWidth > 0) {
        //   y = x_axis_top - y_axis_length;
        // } else {
        //   lineWidth = 1;
        //   y = x_axis_top + axis_label_margin;
        // }
        // axes_context
        //   ..lineWidth = lineWidth
        //   ..strokeStyle = self.base.options.y_axis.grid_line_color
        //   ..begin_path();
        // for (let i = 0; i < x_labels.length; i += x_label_step) {
        //   axes_context.moveTo(x, y);
        //   axes_context.lineTo(x, x_axis_top);
        //   x += scaledLabelHop;
        // }
        // axes_context.stroke();

        // // x-axis itself.

        // if (self.base.options.x_axis.line_width > 0) {
        //   axes_context
        //     ..lineWidth = self.base.options.x_axis.line_width
        //     ..strokeStyle = self.base.options.x_axis.line_color
        //     ..begin_path()
        //     ..moveTo(y_axis_left, x_axis_top)
        //     ..lineTo(y_axis_left + x_axis_length, x_axis_top)
        //     ..stroke();
        // }

        // // y-axis itself.

        // if (self.base.options.y_axis.line_width > 0) {
        //   axes_context
        //     ..lineWidth = self.base.options.y_axis.line_width
        //     ..strokeStyle = self.base.options.y_axis.line_color
        //     ..begin_path()
        //     ..moveTo(y_axis_left, x_axis_top - y_axis_length)
        //     ..lineTo(y_axis_left, x_axis_top)
        //     ..stroke();
        // }
    }

    fn get_entity_group_index(&self, x: f64, num: f64) -> i64 {
        // let dx = x - y_axis_left;
        // // If (x, y) is inside the rectangle defined by the two axes.
        // if (y > x_axis_top - y_axis_length &&
        //     y < x_axis_top &&
        //     dx > 0 &&
        //     dx < x_axis_length) {
        //   let index = (dx / x_label_hop - x_label_offset_factor).round();
        //   // If there is at least one visible point in the current point group...
        //   if (average_y_values[index] != null) return index;
        // }
        // return -1;
        unimplemented!()
    }

    fn update(&self, options: HashMap<String, String>) {
        self.base.update(options);
        self.calculate_average_y_values(0);
    }

    /// Calculates average y values for the visible series to help position the
    /// tooltip.
    ///
    /// If [index] is given, calculates the average y value for the entity group
    /// at [index] only.
    ///
    /// To be overridden.
    // index is opt
    fn calculate_average_y_values(&self, index: usize) {
        // if !self.base.options.tooltip.enabled return;

        // let entityCount = data_table.rows.length;
        // let start = index ?? 0;
        // let end = index == null ? entityCount : index + 1;

        // average_y_values.length = entityCount;

        // for (let i = start; i < end; i++) {
        //   let sum = 0.0;
        //   let count = 0;
        //   for (let j = series_list.length - 1; j >= 0; j--) {
        //     if (series_states[j].index <= Visibility::hiding.index) continue;
        //     let point = series_list[j].entities[i] as Point;
        //     if (point.value != null) {
        //       sum += point.y;
        //       count++;
        //     }
        //   }
        //   average_y_values[i] = (count > 0) ? sum / count : null;
        // }
    }

    fn lerp_points(&self, points: Vec<PointEntity>, percent: f64) -> Vec<PointEntity> {
        // return points.map((p) {
        //   let x = lerp(p.oldX, p.x, percent);
        //   let y = lerp(p.oldY, p.y, percent);
        //   let cp1 = (p.cp1 != null) ? lerp(p.oldCp1, p.cp1, percent) : null;
        //   let cp2 = (p.cp2 != null) ? lerp(p.oldCp2, p.cp2, percent) : null;
        //   return Point()
        //     ..index = p.index
        //     ..value = p.value
        //     ..color = p.color
        //     ..highlight_color = p.highlight_color
        //     ..oldPointRadius = p.oldPointRadius
        //     ..oldX = p.oldX
        //     ..oldY = p.oldY
        //     ..pointRadius = p.pointRadius
        //     ..x = x
        //     ..y = y
        //     ..cp1 = cp1
        //     ..cp2 = cp2;
        // }).toList();
        unimplemented!()
    }

    fn series_visibility_changed(&self, index: usize) {
        self.update_series(index);
        self.calculate_average_y_values(0);
    }
}

impl<'a, C, M, D> Chart<PointEntity> for LineChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
{
    // let num x_label_offset_factor = 0;

    fn calculate_drawing_sizes(&self) {
        self.base.calculate_drawing_sizes();
        // tooltip_offset = self.base.options.series.markers.size * 2 + 5;
    }

    fn draw_series(&self, percent: f64) -> bool {
        fn curve_to(cp1: Point<f64>, cp2: Point<f64>, p: PointEntity) {
            //     if cp2 == null && cp1 == null {
            //       series_context.lineTo(p.x, p.y);
            //     } else if cp2 == null {
            //       series_context.quadraticCurveTo(cp1.x, cp1.y, p.x, p.y);
            //     } else if cp1 == null {
            //       series_context.quadraticCurveTo(cp2.x, cp2.y, p.x, p.y);
            //     } else {
            //       series_context.bezierCurveTo(cp1.x, cp1.y, cp2.x, cp2.y, p.x, p.y);
            //     }
        }

        //   let seriesCount = series_list.length;
        //   let entityCount = data_table.rows.length;
        //   let fillOpacity = self.base.options.series.fill_opacity;
        //   let seriesLineWidth = self.base.options.series.line_width;
        //   let markerOptions = self.base.options.series.markers;
        //   let markerSize = markerOptions["size"];

        //   for (let i = 0; i < seriesCount; i++) {
        //     if (series_states[i] == Visibility::hidden) continue;

        //     let series = series_list[i];
        //     let points = lerpPoints(series.entities.cast<_Point>(), percent);
        //     let scale = (i != focused_series_index) ? 1 : 2;

        //     series_context.lineJoin = "round";

        //     // Draw series with filling.

        //     if (fillOpacity > 0.0) {
        //       let color = change_color_alpha(series.color, fillOpacity);
        //       series_context.fillStyle = color;
        //       series_context.strokeStyle = color;
        //       let j = 0;
        //       while (true) {
        //         // Skip points with a null value.
        //         while (j < entityCount && points[j].value == null) j++;

        //         // Stop if we have reached the end of the series.
        //         if (j == entityCount) break;

        //         // Connect a series of contiguous points with a non-null value and
        //         // fill the area between them and the x-axis.
        //         let p = points[j];
        //         series_context
        //           ..begin_path()
        //           ..moveTo(p.x, x_axis_top)
        //           ..lineTo(p.x, p.y);
        //         let lastPoint = p;
        //         let count = 1;
        //         while (++j < entityCount && points[j].value != null) {
        //           p = points[j];
        //           curveTo(lastPoint.cp2, p.cp1, p);
        //           lastPoint = p;
        //           count++;
        //         }
        //         if (count >= 2) {
        //           series_context
        //             ..lineTo(lastPoint.x, x_axis_top)
        //             ..closePath()
        //             ..fill();
        //         }
        //       }
        //     }

        //     // Draw series without filling.

        //     if (seriesLineWidth > 0) {
        //       let lastPoint = Point();
        //       series_context
        //         ..lineWidth = scale * seriesLineWidth
        //         ..strokeStyle = series.color
        //         ..begin_path();
        //       for (let p in points) {
        //         if (p.value != null) {
        //           if (lastPoint.value != null) {
        //             curveTo(lastPoint.cp2, p.cp1, p);
        //           } else {
        //             series_context.moveTo(p.x, p.y);
        //           }
        //         }
        //         lastPoint = p;
        //       }
        //       series_context.stroke();
        //     }

        //     // Draw markers.

        //     if (markerSize > 0) {
        //       let fillColor = markerOptions["fillColor"] ?? series.color;
        //       let strokeColor = markerOptions["strokeColor"] ?? series.color;
        //       series_context
        //         ..fillStyle = fillColor
        //         ..lineWidth = scale * markerOptions["lineWidth"]
        //         ..strokeStyle = strokeColor;
        //       for (let p in points) {
        //         if (p.value != null) {
        //           if (markerOptions["enabled"]) {
        //             p.draw(series_context, 1.0, p.index == focused_entity_index);
        //           } else if (p.index == focused_entity_index) {
        //             // Only draw marker on hover.
        //             p.draw(series_context, 1.0, true);
        //           }
        //         }
        //       }
        //     }
        //   }

        //   // Draw labels only on the last frame.

        //   let labelOptions = self.base.options.series.labels;
        //   if (percent == 1.0 && labelOptions["enabled"]) {
        //     series_context
        //       ..fillStyle = labelOptions["style"]["color"]
        //       ..font = get_font(labelOptions["style"])
        //       ..textAlign = "center"
        //       ..textBaseline = "alphabetic";
        //     for (let i = 0; i < seriesCount; i++) {
        //       if (series_states[i] != Visibility::shown) continue;

        //       let points = series_list[i].entities;
        //       for (Point p in points) {
        //         if (p.value != null) {
        //           let y = p.y - markerSize - 5;
        //           series_context.fill_text(p.formatted_value, p.x, y);
        //         }
        //       }
        //     }
        //   }

        false
    }

    fn update_series(&self, index: usize) {
        // let entityCount = data_table.rows.length;
        // let markerSize = self.base.options.series.markers.size;
        // let curve_tension = self.base.options.series.curve_tension;
        // let curve = curve_tension > 0 && entityCount > 2;

        // let start = index ?? 0;
        // let end = (index == null) ? series_list.length : index + 1;
        // for (let i = start; i < end; i++) {
        //   let visible = series_states[i].index >= Visibility::showing.index;
        //   let series = series_list[i];
        //   let entities = series.entities;
        //   let color = get_color(i);
        //   let highlight_color = get_highlight_color(color);
        //   series.color = color;
        //   series.highlight_color = highlight_color;

        //   for (let j = 0; j < entityCount; j++) {
        //     let e = entities[j] as Point;
        //     e.index = j;
        //     e.color = color;
        //     e.highlight_color = highlight_color;
        //     e.x = x_label_x(j);
        //     e.y = visible ? valueToY(e.value) : x_axis_top;
        //     e.pointRadius = visible ? markerSize : 0;
        //   }

        //   if (!curve) continue;

        //   let e1;
        //   let e2 = entities[0] as Point;
        //   let e3 = entities[1] as Point;
        //   for (let j = 2; j < entityCount; j++) {
        //     e1 = e2;
        //     e2 = e3;
        //     e3 = entities[j];
        //     if (e1.value == null) continue;
        //     if (e2.value == null) continue;
        //     if (e3.value == null) continue;
        //     let list = calculateControlPoints(
        //         e1.asPoint, e2.asPoint, e3.asPoint, curve_tension);
        //     e2.cp1 = list[0];
        //     e2.cp2 = list[1];
        // ??= - Assign the value if the variable is null
        //     e2.oldCp1 ??= Point(e2.cp1.x, x_axis_top);
        //     e2.oldCp2 ??= Point(e2.cp2.x, x_axis_top);
        //   }
        // }
        unimplemented!()
    }

    fn create_entity(
        &self,
        series_index: usize,
        entity_index: usize,
        value: String,
        color: String,
        highlight_color: String,
    ) -> PointEntity {
        // let x = x_label_x(entityIndex);
        // let oldY = x_axis_top;
        // // oldCp1 and oldCp2 are calculated in [update_series].
        // return Point()
        //   ..index = entityIndex
        //   ..value = value
        //   ..formatted_value = value != null ? entity_value_formatter(value) : null
        //   ..color = color
        //   ..highlight_color = highlight_color
        //   ..oldX = x
        //   ..oldY = oldY
        //   ..oldPointRadius = 09
        //   ..x = x
        //   ..y = valueToY(value)
        //   ..pointRadius = self.base.options.series.markers.size;
        unimplemented!()
    }

    fn get_tooltip_position(&self) -> Point<f64> {
        // FIXME: as usuze
        let x = self.x_label_x(self.base.focused_entity_index as usize) + self.tooltip_offset;
        // let y = max(x_axis_top - y_axis_length,
        //     average_y_values[focused_entity_index] - tooltip.offsetHeight ~/ 2);
        // if (x + tooltip.offsetWidth > width) {
        //   x -= tooltip.offsetWidth + 2 * tooltip_offset;
        //   x = max(x, y_axis_left);
        // }
        // return Point(x, y);
        unimplemented!()
    }
}
