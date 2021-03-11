#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};
use ux_dataflow::*;
use ux_primitives::{
    canvas::CanvasContext,
    color::Color,
    geom::{Point, Rect},
    text::{BaseLine, TextAlign},
};

use crate::*;

#[derive(Default, Clone)]
struct LinePoint {
    color: Color,
    highlight_color: Color,
    // formatted_value: String,
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

impl LinePoint {
    fn as_point(&self) -> Point<f64> {
        Point::new(self.x, self.y)
    }
}

/// A point in a line chart.
impl<C> Drawable<C> for LinePoint
where
    C: CanvasContext,
{
    fn draw(&self, ctx: &C, percent: f64, highlight: bool) {
        let cx = lerp(self.old_x, self.x, percent);
        let cy = lerp(self.old_y, self.y, percent);
        let pr = lerp(self.old_point_radius, self.point_radius, percent);
        if highlight {
          ctx.set_fill_style_color(self.highlight_color);
          ctx.begin_path();
          ctx.arc(cx, cy, 2. * pr, 0., TAU, false);
          ctx.fill();
        }

        ctx.begin_path();
        ctx.arc(cx, cy, pr, 0., TAU, false);
        ctx.fill();
        ctx.stroke();
    }
}

impl Entity for LinePoint {
    fn free(&mut self) {
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

#[derive(Default, Clone)]
struct LineChartProperties {
    x_axis_top: f64,
    y_axis_left: f64,
    x_axis_length: f64,
    y_axis_length: f64,
    xlabel_max_width: f64,
    ylabel_max_width: f64,
    xlabel_rotation: f64, // 0..90
    xlabel_step: i64,
    xlabel_hop: f64, // Distance between two consecutive x-axis labels.
    ylabel_hop: f64, // Distance between two consecutive x-axis labels.
    x_title_box: Rect<f64>,
    y_title_box: Rect<f64>,
    x_title_center: Point<f64>,
    y_title_center: Point<f64>,
    xlabels: Vec<String>,
    ylabels: Vec<String>,
    y_interval: f64,
    y_max_value: f64,
    y_min_value: f64,
    y_range: f64,

    /// The horizontal offset of the tooltip with respect to the vertical line
    /// passing through an x-axis label.
    tooltip_offset: f64,

    ylabel_formatter: Option<ValueFormatter>,
    average_y_values: Vec<f64>,

    xlabel_offset_factor: f64, // = .5;
}

pub struct LineChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
{
    props: RefCell<LineChartProperties>,
    base: BaseChart<'a, C, LinePoint, M, D, LineChartOptions<'a>>,
}

impl<'a, C, M, D> LineChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
{
    pub fn new(options: LineChartOptions<'a>) -> Self {
        Self {
            props: Default::default(),
            base: BaseChart::new(options),
        }
    }

    /// Returns the x coordinate of the x-axis label at [index].
    fn xlabel_x(&self, index: usize) -> f64 {
        let props = self.props.borrow();
        props.y_axis_left + props.xlabel_hop * ((index as f64) + props.xlabel_offset_factor)
    }

    /// Returns the y-coordinate corresponding to the data point [value] and
    /// the animation percent [percent].
    fn value_to_y(&self, value: f64) -> f64 {
        // value != null
        //   ? x_axis_top - (value - y_min_value) / y_range * y_axis_length
        //   : x_axis_top;
        unimplemented!()
    }

    fn data_cell_changed(&self, record: DataCellChangeRecord<D>) {
        if record.column_index == 0 {
            //   xlabels[record.rowIndex] = record.newValue;
        } else {
            self.base.data_cell_changed(record);
        }
    }

    fn get_entity_group_index(&self, x: f64, num: f64) -> i64 {
        // let dx = x - y_axis_left;
        // // If (x, y) is inside the rectangle defined by the two axes.
        // if (y > x_axis_top - y_axis_length &&
        //     y < x_axis_top &&
        //     dx > 0 &&
        //     dx < x_axis_length) {
        //   let index = (dx / xlabel_hop - xlabel_offset_factor).round();
        //   // If there is at least one visible point in the current point group...
        //   if (average_y_values[index] != null) return index;
        // }
        // return -1;
        unimplemented!()
    }

    /// Calculates average y values for the visible series to help position the
    /// tooltip.
    ///
    /// If [index] is given, calculates the average y value for the entity group
    /// at [index] only.
    ///
    fn calculate_average_y_values(&self, index: usize) {
        // if !self.base.options.tooltip.enabled return;

        // let entity_count = self.base.data_table.rows.len();
        // let start = index ?? 0;
        // let end = index == null ? entity_count : index + 1;

        // average_y_values.len() = entity_count;

        // for (let i = start; i < end; i++) {
        //   let sum = 0.0;
        //   let count = 0;
        //   for (let j = series_list.len() - 1; j >= 0; j--) {
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

    fn lerp_points(&self, points: Vec<LinePoint>, percent: f64) -> Vec<LinePoint> {
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

impl<'a, C, M, D> Chart<'a, C, M, D, LinePoint> for LineChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
{
    // let num xlabel_offset_factor = 0;

    // TODO: Separate y-axis stuff into a separate method.
    fn calculate_drawing_sizes(&self) {
        self.base.calculate_drawing_sizes();
        // tooltip_offset = self.base.options.series.markers.size * 2 + 5;

        //     // y-axis min-max.

        //     self.y_max_value = self.base.options.y_axis.max_value ?? f64::NEG_INFINITY;
        //     y_max_value = max(y_max_value, find_max_value(self.base.data_table));
        //     if (y_max_value == f64::NEG_INFINITY) y_max_value = 0.0;

        //     y_min_value = self.base.options.y_axis.min_value ?? f64::INFINITY;
        //     y_min_value = min(y_min_value, findMinValue(self.base.data_table));
        //     if (y_min_value == f64::INFINITY) y_min_value = 0.0;

        //     y_interval = self.base.options.y_axis.interval;
        //     let min_interval = self.base.options.y_axis.min_interval;

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
        //         if (min_interval != null) {
        //           y_interval = max(y_interval, min_interval);
        //         }
        //       } else {
        //         y_interval = utils::calculate_interval(y_max_value - y_min_value, 5, min_interval);
        //       }
        //     }

        //     y_min_value = (y_min_value / y_interval).floorToDouble() * y_interval;
        //     y_max_value = (y_max_value / y_interval).ceilToDouble() * y_interval;
        //     yRange = y_max_value - y_min_value;

        //     // y-axis labels.

        //     ylabels = <String>[];
        //     ylabel_formatter = self.base.options.y_axis.labels.formatter;
        //     if (ylabel_formatter == null) {
        //       let maxDecimalPlaces =
        //           max(utils::get_decimal_places(y_interval), utils::get_decimal_places(y_min_value));
        //       let numberFormat = NumberFormat.decimalPattern()
        //         ..maximumFractionDigits = maxDecimalPlaces
        //         ..minimumFractionDigits = maxDecimalPlaces;
        //       ylabel_formatter = numberFormat.format;
        //     }
        //     let value = y_min_value;
        //     while (value <= y_max_value) {
        //       ylabels.add(ylabel_formatter(value));
        //       value += y_interval;
        //     }
        //     ylabel_max_width = calculateMaxTextWidth(
        //             context, get_font(self.base.options.y_axis.labels.style), ylabels)
        //         .round();

        //     entity_value_formatter = ylabel_formatter;

        //     // Tooltip.

        //     tooltip_value_formatter =
        //         self.base.options.tooltip.value_formatter ?? ylabel_formatter;

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
        //       xTitleHeight = xTitle["style"]["font_size"] + 2 * TITLE_PADDING;
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
        //       yTitleWidth = yTitle["style"]["font_size"] + 2 * TITLE_PADDING;
        //       yTitleLeft = self.base.series_and_axes_box.left;
        //     }

        //     // Axes" size and position.

        //     y_axis_left = self.base.series_and_axes_box.left + ylabel_max_width + AXIS_LABEL_MARGIN;
        //     if (yTitleWidth > 0) {
        //       y_axis_left += yTitleWidth + CHART_TITLE_MARGIN;
        //     } else {
        //       y_axis_left += AXIS_LABEL_MARGIN;
        //     }

        //     x_axis_length = self.base.series_and_axes_box.right - y_axis_left;

        //     x_axis_top = self.base.series_and_axes_box.bottom;
        //     if (xTitleHeight > 0) {
        //       x_axis_top -= xTitleHeight + CHART_TITLE_MARGIN;
        //     } else {
        //       x_axis_top -= AXIS_LABEL_MARGIN;
        //     }
        //     x_axis_top -= AXIS_LABEL_MARGIN;

        //     // x-axis labels and x-axis"s position.

        //     let rowCount = self.base.data_table.rows.len();
        //     xlabels = <String>[];
        //     for (let i = 0; i < rowCount; i++) {
        //       xlabels.add(self.base.data_table.rows[i][0].to_string());
        //     }
        //     xlabel_max_width = calculateMaxTextWidth(
        //         context, get_font(self.base.options.x_axis.labels.style), xlabels);
        //     if (xlabel_offset_factor > 0 && rowCount > 1) {
        //       xlabel_hop = x_axis_length / rowCount;
        //     } else if (rowCount > 1) {
        //       xlabel_hop = x_axis_length / (rowCount - 1);
        //     } else {
        //       xlabel_hop = x_axis_length;
        //     }
        //     xlabel_rotation = 0;

        //     let font_size = self.base.options.x_axis.labels.style.font_size;
        //     let maxRotation = self.base.options.x_axis.labels.max_rotation;
        //     let minRotation = self.base.options.x_axis.labels.min_rotation;
        //     const angles = [0, -45, 45, -90, 90];

        //     outer:
        //     for (let step = 1; step <= rowCount; step++) {
        //       let scaledLabelHop = step * xlabel_hop;
        //       let minSpacing = max(.1 * scaledLabelHop, 10);
        //       for (let angle in angles) {
        //         if (angle > maxRotation) continue;
        //         if (angle < minRotation) continue;

        //         let absAngleRad = deg2rad(angle).abs();
        //         let labelSpacing = angle == 0
        //             ? scaledLabelHop - xlabel_max_width
        //             : scaledLabelHop * sin(absAngleRad) - font_size;
        //         if (labelSpacing < minSpacing) continue;

        //         xlabel_rotation = angle;
        //         xlabel_step = step;
        //         x_axis_top -=
        //             xlabel_max_width * sin(absAngleRad) + font_size * cos(absAngleRad);
        //         break outer;
        //       }
        //     }

        //     // Wrap up.

        //     y_axis_length = x_axis_top -
        //         self.base.series_and_axes_box.top -
        //         (self.base.options.y_axis.labels.style.font_size / 2).trunc();
        //     ylabel_hop = y_axis_length / (ylabels.len() - 1);

        //     xTitleLeft = y_axis_left + ((x_axis_length - xTitleWidth) / 2).trunc();
        //     yTitleTop = self.base.series_and_axes_box.top + ((y_axis_length - yTitleHeight) / 2).trunc();

        //     if (xTitleHeight > 0) {
        // //      x_title_box =
        // //          Rectangle(xTitleLeft, xTitleTop, xTitleWidth, xTitleHeight);
        //       x_title_center =
        //           Point(xTitleLeft + (xTitleWidth / 2).trunc(), xTitleTop + (xTitleHeight / 2).trunc());
        //     } else {
        // //      x_title_box = null;
        //       x_title_center = null;
        //     }

        //     if (yTitleHeight > 0) {
        // //      y_title_box =
        // //          Rectangle(yTitleLeft, yTitleTop, yTitleWidth, yTitleHeight);
        //       y_title_center =
        //           Point(yTitleLeft + (yTitleWidth / 2).trunc(), yTitleTop + (yTitleHeight / 2).trunc());
        //     } else {
        // //      y_title_box = null;
        //       y_title_center = null;
        //     }
        unimplemented!()
    }

    fn set_stream(&self, stream: DataStream<'a, M, D>) {}

    fn draw(&self, ctx: &C) {
        self.base.dispose();
        // data_tableSubscriptionTracker
        //   ..add(dataTable.onCellChange.listen(data_cell_changed))
        //   ..add(dataTable.onColumnsChange.listen(dataColumnsChanged))
        //   ..add(dataTable.onRowsChange.listen(data_rows_changed));
        // self.easing_function = get_easing(self.options.animation().easing);
        self.base.initialize_legend();
        self.base.initialize_tooltip();

        // self.axes_context.clearRect(0, 0, self.width, self.height);
        self.draw_axes_and_grid(ctx);
        self.base.start_animation();
    }

    fn update(&self, ctx: &C) {
        self.base.update(ctx);
        self.calculate_average_y_values(0);
    }

    fn resize(&self, w: f64, h: f64) {
        self.base.resize(w, h);
    }

    /// Draws the axes and the grid.
    ///
    fn draw_axes_and_grid(&self, ctx: &C) {
        self.base.draw_axes_and_grid(ctx);
        // TODO: if ok then remove below commented code
        // coz it implemented in basis

        // // x-axis title.

        // if (x_title_center != null) {
        //   let opt = self.base.options.x_axis.title;
        //   ctx.set_fill_style_color(opt.style.color);
        //   ctx.set_font(utils::get_font(opt.style))
        //   ctx.set_text_align(TextAlign::Center);
        //   ctx.set_text_baseline(BaseLine::Middle);
        //   ctx.fill_text(opt.text, x_title_center.x, x_title_center.y);
        // }

        // // y-axis title.

        // if (y_title_center != null) {
        //   let opt = self.base.options.y_axis.title;
        //   axes_context
        //     ..save()
        //     ..fillStyle = opt.style.["color"]
        //     ..font = get_font(opt.style.)
        //     ..translate(y_title_center.x, y_title_center.y)
        //     ..rotate(-PI_2)
        //     ..textAlign = TextAlign::Center
        //     ..textBaseline = BaseLine::Middle
        //     ..fill_text(opt["text"], 0, 0)
        //     ..restore();
        // }

        // // x-axis labels.

        // let opt = self.base.options.x_axis.labels;
        // axes_context.fillStyle = opt.style.["color"];
        // axes_context.font = get_font(opt.style.);
        // let x = xlabel_x(0);
        // let y = x_axis_top + AXIS_LABEL_MARGIN + opt.style.["font_size"];
        // let scaledLabelHop = xlabel_step * xlabel_hop;

        // if (xlabel_rotation == 0) {
        //   axes_context.textAlign = TextAlign::Center;
        //   axes_context.textBaseline = "alphabetic";
        //   for (let i = 0; i < xlabels.len(); i += xlabel_step) {
        //     axes_context.fill_text(xlabels[i], x, y);
        //     x += scaledLabelHop;
        //   }
        // } else {
        //   axes_context.textAlign = xlabel_rotation < 0 ? "right" : "left";
        //   axes_context.textBaseline = BaseLine::Middle;
        //   if (xlabel_rotation == 90) {
        //     x += xlabel_rotation.sign * (opt.style.["font_size"] / 8).trunc();
        //   }
        //   let angle = deg2rad(xlabel_rotation);
        //   for (let i = 0; i < xlabels.len(); i += xlabel_step) {
        //     axes_context
        //       ..save()
        //       ..translate(x, y)
        //       ..rotate(angle)
        //       ..fill_text(xlabels[i], 0, 0)
        //       ..restore();
        //     x += scaledLabelHop;
        //   }
        // }

        // // y-axis labels.

        // axes_context
        //   ..fillStyle = self.base.options.y_axis.labels.style.color
        //   ..font = get_font(self.base.options.y_axis.labels.style)
        //   ..textAlign = "right"
        //   ..textBaseline = BaseLine::Middle;
        // x = y_axis_left - AXIS_LABEL_MARGIN;
        // y = x_axis_top - (self.base.options.y_axis.labels.style.font_size / 8).trunc();
        // for (let label in ylabels) {
        //   axes_context.fill_text(label, x, y);
        //   y -= ylabel_hop;
        // }

        // // x grid lines - draw bottom up.

        // if (self.base.options.x_axis.grid_line_width > 0) {
        //   axes_context
        //     ..lineWidth = self.base.options.x_axis.grid_line_width
        //     ..strokeStyle = self.base.options.x_axis.grid_line_color
        //     ..begin_path();
        //   y = x_axis_top - ylabel_hop;
        //   for (let i = ylabels.len() - 1; i >= 1; i--) {
        //     axes_context.moveTo(y_axis_left, y);
        //     axes_context.lineTo(y_axis_left + x_axis_length, y);
        //     y -= ylabel_hop;
        //   }
        //   axes_context.stroke();
        // }

        // // y grid lines or x-axis ticks - draw from left to right.

        // let lineWidth = self.base.options.y_axis.grid_line_width;
        // x = y_axis_left;
        // if (xlabel_step > 1) {
        //   x = xlabel_x(0);
        // }
        // if (lineWidth > 0) {
        //   y = x_axis_top - y_axis_length;
        // } else {
        //   lineWidth = 1;
        //   y = x_axis_top + AXIS_LABEL_MARGIN;
        // }
        // axes_context
        //   ..lineWidth = lineWidth
        //   ..strokeStyle = self.base.options.y_axis.grid_line_color
        //   ..begin_path();
        // for (let i = 0; i < xlabels.len(); i += xlabel_step) {
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

    /// Draws the current animation frame.
    ///
    /// If [time] is `null`, draws the last frame (i.e. no animation).
    fn draw_frame(&self, ctx: &C, time: Option<i64>) {
        self.base.draw_frame(ctx, time);

        let mut percent = self.base.calculate_percent(time);

        if percent >= 1.0 {
            percent = 1.0;

            // Update the visibility states of all series before the last frame.
            // for (let i = series_states.len() - 1; i >= 0; i--) {
            //     if (series_states[i] == Visibility::showing) {
            //         series_states[i] = Visibility::shown;
            //     } else if (series_states[i] == Visibility::hiding) {
            //         series_states[i] = Visibility::hidden;
            //     }
            // }
        }

        let props = self.base.props.borrow();

        let ease = props.easing_function.unwrap();
        self.draw_series(ctx, ease(percent));
        // context.drawImageScaled(axes_context.canvas, 0, 0, width, height);
        // context.drawImageScaled(series_context.canvas, 0, 0, width, height);
        self.base.draw_title(ctx);

        if percent < 1.0 {
            // animation_frame_id = window.requestAnimationFrame(draw_frame);
        } else if time.is_some() {
            self.base.animation_end();
        }
    }

    fn draw_series(&self, ctx: &C, percent: f64) -> bool {
        fn curve_to(cp1: Point<f64>, cp2: Point<f64>, p: LinePoint) {
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

        //   let series_count = series_list.len();
        //   let entity_count = self.base.data_table.rows.len();
        //   let fill_opacity = self.base.options.series.fill_opacity;
        //   let series_line_width = self.base.options.series.line_width;
        //   let marker_options = self.base.options.series.markers;
        //   let marker_size = marker_options["size"];

        //   for (let i = 0; i < series_count; i++) {
        //     if (series_states[i] == Visibility::hidden) continue;

        //     let series = series_list[i];
        //     let points = lerpPoints(series.entities.cast<_Point>(), percent);
        //     let scale = (i != focused_series_index) ? 1 : 2;

        //     series_context.lineJoin = "round";

        //     // Draw series with filling.

        //     if (fill_opacity > 0.0) {
        //       let color = change_color_alpha(series.color, fill_opacity);
        //       series_context.fillStyle = color;
        //       series_context.strokeStyle = color;
        //       let j = 0;
        //       while (true) {
        //         // Skip points with a null value.
        //         while (j < entity_count && points[j].value == null) j++;

        //         // Stop if we have reached the end of the series.
        //         if (j == entity_count) break;

        //         // Connect a series of contiguous points with a non-null value and
        //         // fill the area between them and the x-axis.
        //         let p = points[j];
        //         series_context
        //           ..begin_path()
        //           ..moveTo(p.x, x_axis_top)
        //           ..lineTo(p.x, p.y);
        //         let lastPoint = p;
        //         let count = 1;
        //         while (++j < entity_count && points[j].value != null) {
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

        //     if (series_line_width > 0) {
        //       let lastPoint = Point();
        //       series_context
        //         ..lineWidth = scale * series_line_width
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

        //     if (marker_size > 0) {
        //       let fillColor = marker_options["fillColor"] ?? series.color;
        //       let strokeColor = marker_options["strokeColor"] ?? series.color;
        //       series_context
        //         ..fillStyle = fillColor
        //         ..lineWidth = scale * marker_options["lineWidth"]
        //         ..strokeStyle = strokeColor;
        //       for (let p in points) {
        //         if (p.value != null) {
        //           if (marker_options["enabled"]) {
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
        //       ..textAlign = TextAlign::Center
        //       ..textBaseline = "alphabetic";
        //     for (let i = 0; i < series_count; i++) {
        //       if (series_states[i] != Visibility::shown) continue;

        //       let points = series_list[i].entities;
        //       for (Point p in points) {
        //         if (p.value != null) {
        //           let y = p.y - marker_size - 5;
        //           series_context.fill_text(p.formatted_value, p.x, y);
        //         }
        //       }
        //     }
        //   }

        false
    }

    fn update_series(&self, index: usize) {
        // let entity_count = self.base.data_table.rows.len();
        // let marker_size = self.base.options.series.markers.size;
        // let curve_tension = self.base.options.series.curve_tension;
        // let curve = curve_tension > 0 && entity_count > 2;

        // let start = index ?? 0;
        // let end = (index == null) ? series_list.len() : index + 1;
        // for (let i = start; i < end; i++) {
        //   let visible = series_states[i].index >= Visibility::showing.index;
        //   let series = series_list[i];
        //   let entities = series.entities;
        //   let color = get_color(i);
        //   let highlight_color = get_highlight_color(color);
        //   series.color = color;
        //   series.highlight_color = highlight_color;

        //   for (let j = 0; j < entity_count; j++) {
        //     let e = entities[j] as Point;
        //     e.index = j;
        //     e.color = color;
        //     e.highlight_color = highlight_color;
        //     e.x = xlabel_x(j);
        //     e.y = visible ? value_to_y(e.value) : x_axis_top;
        //     e.pointRadius = visible ? marker_size : 0;
        //   }

        //   if (!curve) continue;

        //   let e1;
        //   let e2 = entities[0] as Point;
        //   let e3 = entities[1] as Point;
        //   for (let j = 2; j < entity_count; j++) {
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
        value: f64,
        color: Color,
        highlight_color: Color,
    ) -> LinePoint {
        // let x = xlabel_x(entity_index);
        // let oldY = x_axis_top;
        // // oldCp1 and oldCp2 are calculated in [update_series].
        // return Point()
        //   ..index = entity_index
        //   ..value = value
        //   ..formatted_value = value != null ? entity_value_formatter(value) : null
        //   ..color = color
        //   ..highlight_color = highlight_color
        //   ..oldX = x
        //   ..oldY = oldY
        //   ..oldPointRadius = 09
        //   ..x = x
        //   ..y = value_to_y(value)
        //   ..pointRadius = self.base.options.series.markers.size;
        unimplemented!()
    }

    fn get_tooltip_position(&self, tooltip_width: f64, tooltip_height: f64) -> Point<f64> {
        let props = self.props.borrow();
        let focused_entity_index = self.base.props.borrow().focused_entity_index;

        // FIXME: as usuze
        let x = self.xlabel_x(focused_entity_index as usize) + props.tooltip_offset;
        // let y = max(x_axis_top - y_axis_length,
        //     average_y_values[focused_entity_index] - (tooltip.offset_height / 2).trunc());
        // if (x + tooltip.offset_width > width) {
        //   x -= tooltip.offset_width + 2 * tooltip_offset;
        //   x = max(x, y_axis_left);
        // }
        // return Point(x, y);
        unimplemented!()
    }
}
