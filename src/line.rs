#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};
use ux_dataflow::*;
use ux_primitives::{
    canvas::{CanvasContext, LineJoin},
    color::Color,
    geom::{Point, Rect},
    text::{BaseLine, TextAlign, TextStyle, TextWeight},
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
    old_cp1: Point<f64>,
    old_cp2: Point<f64>,
    old_point_radius: f64,

    /// The first control point.
    cp1: Point<f64>,

    /// The second control point.
    cp2: Point<f64>,
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
        let cx = utils::lerp(self.old_x, self.x, percent);
        let cy = utils::lerp(self.old_y, self.y, percent);
        let pr = utils::lerp(self.old_point_radius, self.point_radius, percent);
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
    fn free(&mut self) {}

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

    fn get_entity_group_index(&self, x: f64, y: f64) -> i64 {
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

    fn lerp_points(&self, points: &Vec<LinePoint>, percent: f64) -> Vec<LinePoint> {
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

    fn curve_to(&self, ctx: &C, cp1: Option<Point<f64>>, cp2: Option<Point<f64>>, p: &LinePoint) {
        if cp2.is_none() && cp1.is_none() {
            ctx.line_to(p.x, p.y);
        } else if cp2.is_none() {
            let cp = cp1.unwrap();
            ctx.quadratic_curve_to(cp.x, cp.y, p.x, p.y);
        } else if cp1.is_none() {
            let cp = cp2.unwrap();
            ctx.quadratic_curve_to(cp.x, cp.y, p.x, p.y);
        } else {
            let cp1 = cp1.unwrap();
            let cp2 = cp2.unwrap();
            ctx.bezier_curve_to(cp1.x, cp1.y, cp2.x, cp2.y, p.x, p.y);
        }
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
        //     ylabel_max_width = calculate_max_text_width(
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
        //       xTitleWidth = context.measure_text(xTitle["text"]).width.round() +
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
        //       yTitleHeight = context.measure_text(yTitle["text"]).width.round() +
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
        //     xlabel_max_width = calculate_max_text_width(
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

        // self.ctx.clearRect(0, 0, self.width, self.height);
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
        //   ctx
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
        // ctx.fillStyle = opt.style.["color"];
        // ctx.font = get_font(opt.style.);
        // let x = xlabel_x(0);
        // let y = x_axis_top + AXIS_LABEL_MARGIN + opt.style.["font_size"];
        // let scaledLabelHop = xlabel_step * xlabel_hop;

        // if (xlabel_rotation == 0) {
        //   ctx.textAlign = TextAlign::Center;
        //   ctx.textBaseline = "alphabetic";
        //   for (let i = 0; i < xlabels.len(); i += xlabel_step) {
        //     ctx.fill_text(xlabels[i], x, y);
        //     x += scaledLabelHop;
        //   }
        // } else {
        //   ctx.textAlign = xlabel_rotation < 0 ? "right" : "left";
        //   ctx.textBaseline = BaseLine::Middle;
        //   if (xlabel_rotation == 90) {
        //     x += xlabel_rotation.sign * (opt.style.["font_size"] / 8).trunc();
        //   }
        //   let angle = deg2rad(xlabel_rotation);
        //   for (let i = 0; i < xlabels.len(); i += xlabel_step) {
        //     ctx
        //       ..save()
        //       ..translate(x, y)
        //       ..rotate(angle)
        //       ..fill_text(xlabels[i], 0, 0)
        //       ..restore();
        //     x += scaledLabelHop;
        //   }
        // }

        // // y-axis labels.

        // ctx
        //   ..fillStyle = self.base.options.y_axis.labels.style.color
        //   ..font = get_font(self.base.options.y_axis.labels.style)
        //   ..textAlign = "right"
        //   ..textBaseline = BaseLine::Middle;
        // x = y_axis_left - AXIS_LABEL_MARGIN;
        // y = x_axis_top - (self.base.options.y_axis.labels.style.font_size / 8).trunc();
        // for (let label in ylabels) {
        //   ctx.fill_text(label, x, y);
        //   y -= ylabel_hop;
        // }

        // // x grid lines - draw bottom up.

        // if (self.base.options.x_axis.grid_line_width > 0) {
        //   ctx
        //     ..line_width = self.base.options.x_axis.grid_line_width
        //     ..strokeStyle = self.base.options.x_axis.grid_line_color
        //     ..begin_path();
        //   y = x_axis_top - ylabel_hop;
        //   for (let i = ylabels.len() - 1; i >= 1; i--) {
        //     ctx.move_to(y_axis_left, y);
        //     ctx.line_to(y_axis_left + x_axis_length, y);
        //     y -= ylabel_hop;
        //   }
        //   ctx.stroke();
        // }

        // // y grid lines or x-axis ticks - draw from left to right.

        // let line_width = self.base.options.y_axis.grid_line_width;
        // x = y_axis_left;
        // if (xlabel_step > 1) {
        //   x = xlabel_x(0);
        // }
        // if (line_width > 0) {
        //   y = x_axis_top - y_axis_length;
        // } else {
        //   line_width = 1;
        //   y = x_axis_top + AXIS_LABEL_MARGIN;
        // }
        // ctx
        //   ..line_width = line_width
        //   ..strokeStyle = self.base.options.y_axis.grid_line_color
        //   ..begin_path();
        // for (let i = 0; i < xlabels.len(); i += xlabel_step) {
        //   ctx.move_to(x, y);
        //   ctx.line_to(x, x_axis_top);
        //   x += scaledLabelHop;
        // }
        // ctx.stroke();

        // // x-axis itself.

        // if (self.base.options.x_axis.line_width > 0) {
        //   ctx
        //     ..line_width = self.base.options.x_axis.line_width
        //     ..strokeStyle = self.base.options.x_axis.line_color
        //     ..begin_path()
        //     ..move_to(y_axis_left, x_axis_top)
        //     ..line_to(y_axis_left + x_axis_length, x_axis_top)
        //     ..stroke();
        // }

        // // y-axis itself.

        // if (self.base.options.y_axis.line_width > 0) {
        //   ctx
        //     ..line_width = self.base.options.y_axis.line_width
        //     ..strokeStyle = self.base.options.y_axis.line_color
        //     ..begin_path()
        //     ..move_to(y_axis_left, x_axis_top - y_axis_length)
        //     ..line_to(y_axis_left, x_axis_top)
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
            let mut props = self.base.props.borrow_mut();

            for idx in props.series_states.len()..0 {
                if props.series_states[idx] == Visibility::Showing {
                    props.series_states[idx] = Visibility::Shown;
                } else if props.series_states[idx] == Visibility::Hiding {
                    props.series_states[idx] = Visibility::Hidden;
                }
            }
        }

        let props = self.base.props.borrow();

        let ease = props.easing_function.unwrap();
        self.draw_series(ctx, ease(percent));
        // context.drawImageScaled(ctx.canvas, 0, 0, width, height);
        // context.drawImageScaled(ctx.canvas, 0, 0, width, height);
        self.base.draw_title(ctx);

        if percent < 1.0 {
            // animation_frame_id = window.requestAnimationFrame(draw_frame);
        } else if time.is_some() {
            self.base.animation_end();
        }
    }

    fn draw_series(&self, ctx: &C, percent: f64) -> bool {
        let series_list = self.base.series_list.borrow();
        let series_count = series_list.len();
        let entity_count = self.base.data_table.frames.len();
        let fill_opacity = self.base.options.series.fill_opacity;
        let series_line_width = self.base.options.series.line_width;
        let marker_options = &self.base.options.series.markers;
        let marker_size = marker_options.size;
        let series_states = &self.base.props.borrow().series_states;
        let focused_series_index = self.base.props.borrow().focused_series_index;
        let focused_entity_index = self.base.props.borrow().focused_entity_index as usize;
        let props = self.props.borrow();
        let label_options = &self.base.options.series.labels;

        for idx in 0..series_count {
            if series_states[idx] == Visibility::Hidden {
                continue;
            }

            let series = series_list.get(idx).unwrap();
            let entities = self.lerp_points(&series.entities, percent);
            let scale = if idx as i64 != focused_series_index {
                1.
            } else {
                2.
            };

            ctx.set_line_join(LineJoin::Round);

            // Draw series with filling.
            if fill_opacity > 0.0 {
                let color = self.base.change_color_alpha(series.color, fill_opacity);
                ctx.set_fill_style_color(color);
                ctx.set_stroke_style_color(color);
                let mut jdx = 0;
                loop {
                    // Skip points with a null value.
                    while jdx < entity_count && entities[jdx].value == 0. {
                        jdx += 1;
                    }

                    // Stop if we have reached the end of the series.
                    if jdx == entity_count {
                        break;
                    }

                    // Connect a series of contiguous points with a non-null value and
                    // fill the area between them and the x-axis.
                    let mut entity = entities.get(jdx).unwrap();
                    ctx.begin_path();
                    ctx.move_to(entity.x, props.x_axis_top);
                    ctx.line_to(entity.x, entity.y);
                    let mut last_point = entity;
                    let mut count = 1;
                    while jdx < entity_count && entities[jdx].value != 0. {
                        entity = entities.get(jdx).unwrap();
                        self.curve_to(ctx, Some(last_point.cp2), Some(entity.cp1), entity);
                        last_point = entity;
                        count += 1;
                        jdx += 1;
                    }
                    if count >= 2 {
                        ctx.line_to(last_point.x, props.x_axis_top);
                        ctx.close_path();
                        ctx.fill();
                    }
                }
            }

            // Draw series without filling.
            if series_line_width > 0. {
                let mut last_point: LinePoint = Default::default();
                ctx.set_line_width(scale * series_line_width);
                ctx.set_stroke_style_color(series.color);
                ctx.begin_path();
                for entity in entities.iter() {
                    if entity.value != 0. {
                        if last_point.value != 0. {
                            self.curve_to(ctx, Some(last_point.cp2), Some(entity.cp1), entity);
                        } else {
                            ctx.move_to(entity.x, entity.y);
                        }
                    }
                    last_point = entity.clone();
                }
                ctx.stroke();
            }

            // Draw markers.
            if marker_size > 0. {
                let fill_color = if let Some(color) = marker_options.fill_color {
                    color
                } else {
                    series.color
                };

                let stroke_color = if let Some(color) = marker_options.stroke_color {
                    color
                } else {
                    series.color
                };
                ctx.set_fill_style_color(fill_color);
                ctx.set_line_width(scale * marker_options.line_width as f64);
                ctx.set_stroke_style_color(stroke_color);
                for entity in entities.iter() {
                    if entity.value != 0. {
                        if marker_options.enabled {
                            entity.draw(ctx, 1.0, entity.index == focused_entity_index);
                        } else if entity.index == focused_entity_index {
                            // Only draw marker on hover.
                            entity.draw(ctx, 1.0, true);
                        }
                    }
                }
            }
        }

        // Draw labels only on the last frame.
        if let Some(label_options) = label_options {
            if percent == 1.0 {
                ctx.set_fill_style_color(label_options.color);
                ctx.set_font(
                    label_options.font_family.unwrap_or(DEFAULT_FONT_FAMILY),
                    label_options.font_style.unwrap_or(TextStyle::Normal),
                    TextWeight::Normal,
                    label_options.font_size.unwrap_or(12.),
                );
                ctx.set_text_align(TextAlign::Center);
                ctx.set_text_baseline(BaseLine::Alphabetic);
                for idx in 0..series_count {
                    if series_states[idx] != Visibility::Shown {
                        continue;
                    }

                    let entities = &series_list.get(idx).unwrap().entities;
                    for entity in entities.iter() {
                        if entity.value != 0. {
                            let y = entity.y - marker_size - 5.;
                            // TODO: bar.formatted_value
                            let formatted_value = format!("{}", entity.value);
                            ctx.fill_text(formatted_value.as_str(), entity.x, y);
                        }
                    }
                }
            }
        }

        false
    }

    fn update_series(&self, index: usize) {
        let entity_count = self.base.data_table.frames.len();
        let marker_size = self.base.options.series.markers.size;
        let curve_tension = self.base.options.series.curve_tension;
        let curve = curve_tension > 0. && entity_count > 2;

        let start = if index != 0 { index } else { 0 };

        let mut series_list = self.base.series_list.borrow_mut();
        let end = if index == 0 {
            series_list.len()
        } else {
            index + 1
        };

        let series_states = &self.base.props.borrow().series_states;
        let props = self.props.borrow();
        for idx in start..end {
            let series_state = series_states[idx];
            let visible = series_state == Visibility::Showing || series_state == Visibility::Shown;

            let series = series_list.get_mut(idx).unwrap();

            let color = self.base.get_color(idx);
            let highlight_color = self.base.get_highlight_color(color);
            series.color = color;
            series.highlight_color = highlight_color;

            for jdx in 0..entity_count {
                let entity = series.entities.get_mut(jdx).unwrap();
                entity.index = jdx;
                entity.color = color;
                entity.highlight_color = highlight_color;
                entity.x = self.xlabel_x(jdx);
                entity.y = if visible {
                    self.value_to_y(entity.value)
                } else {
                    props.x_axis_top
                };
                entity.point_radius = if visible { marker_size } else { 0. };
            }

            if !curve {
                continue;
            }

            // // TODO: complete it
            // let mut e1;
            // let mut e2 = series.entities.get_mut(0).unwrap();
            // let mut e3 = series.entities.get_mut(1).unwrap();
            // for jdx in 2..entity_count {
            //     e1 = e2;
            //     e2 = e3;
            //     e3 = series.entities.get_mut(jdx).unwrap();
            //     if e1.value == 0. {
            //         continue;
            //     }
            //     if e2.value == 0. {
            //         continue;
            //     }
            //     if e3.value == 0. {
            //         continue;
            //     }

            //     let list = utils::calculate_control_points(
            //         e1.as_point(),
            //         e2.as_point(),
            //         e3.as_point(),
            //         curve_tension,
            //     );
            //     e2.cp1 = list[0];
            //     e2.cp2 = list[1];
            //     // ??= - Assign the value if the variable is null
            //     e2.oldCp1?? = Point::new(e2.cp1.x, x_axis_top);
            //     e2.oldCp2?? = Point::new(e2.cp2.x, x_axis_top);
            // }
        }
    }

    fn create_entity(
        &self,
        series_index: usize,
        entity_index: usize,
        value: f64,
        color: Color,
        highlight_color: Color,
    ) -> LinePoint {
        let x = self.xlabel_x(entity_index);

        let props = self.props.borrow();
        let old_y = props.x_axis_top;
        // oldCp1 and oldCp2 are calculated in [update_series].

        // let formatted_value = if value != 0 {
        //     entity_value_formatter(value)
        // } else {
        //     null
        // };

        LinePoint {
            index: entity_index,
            old_value: 0.,
            value,
            //   formatted_value,
            color,
            highlight_color,
            old_x: x,
            old_y,
            old_cp1: Default::default(),
            old_cp2: Default::default(),
            cp1: Default::default(),
            cp2: Default::default(),
            old_point_radius: 9.,
            x,
            y: self.value_to_y(value),
            point_radius: self.base.options.series.markers.size,
        }
    }

    fn get_tooltip_position(&self, tooltip_width: f64, tooltip_height: f64) -> Point<f64> {
        let props = self.props.borrow();
        let focused_entity_index = self.base.props.borrow().focused_entity_index as usize;

        let mut x = self.xlabel_x(focused_entity_index) + props.tooltip_offset;
        let y = f64::max(
            props.x_axis_top - props.y_axis_length,
            props.average_y_values[focused_entity_index] - (tooltip_height / 2.).trunc(),
        );

        let width = self.base.props.borrow().width;
        if x + tooltip_width > width {
            x -= tooltip_width + 2. * props.tooltip_offset;
            x = x.max(props.y_axis_left);
        }

        Point::new(x, y)
    }
}
