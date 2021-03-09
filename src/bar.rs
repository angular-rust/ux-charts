#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};
use ux_dataflow::*;
use ux_primitives::{
    canvas::CanvasContext,
    geom::{Point, Rect, Size},
};

use crate::*;

#[derive(Default, Clone)]
pub struct BarEntity {
    // Chart chart,
    color: String,
    highlight_color: String,
    formatted_value: String,
    index: usize,
    old_value: f64,
    value: f64,

    old_left: f64,
    old_width: f64,
    old_height: f64,
    bottom: f64,
    left: f64,
    width: f64,
    height: f64,
}

impl BarEntity {
    pub fn get_right(&self) -> f64 {
        self.left + self.width
    }
}

impl<C> Drawable<C> for BarEntity
where
    C: CanvasContext,
{
    fn draw(&self, ctx: C, percent: f64, highlight: bool) {
        let x = lerp(self.old_left, self.left, percent);
        let h = lerp(self.old_height, self.height, percent);
        let w = lerp(self.old_width, self.width, percent);
        // ctx.fillStyle = color;
        // ctx.fillRect(x, bottom - h, w, h);
        if highlight {
            //   ctx.fillStyle = "rgba(255, 255, 255, .25)";
            ctx.fill_rect(x, self.bottom - h, w, h);
        }
        unimplemented!()
    }
}

impl Entity for BarEntity {
    fn free(&mut self) {
        // chart = null;
    }

    fn save(&self) {
        // self.old_left = self.left;
        // self.old_width = self.width;
        // self.old_height = self.height;
        // self.old_value = self.value;
    }
}

#[derive(Default, Clone)]
struct BarChartProperties {
    x_axis_top: f64,
    y_axis_left: f64,
    x_axis_length: f64,
    y_axis_length: f64,
    x_label_max_width: f64,
    y_label_max_width: f64,
    x_label_rotation: f64, // 0..90
    x_label_step: i64,
    /// Distance between two consecutive x-axis labels.
    x_label_hop: f64,
    /// Distance between two consecutive x-axis labels.
    y_label_hop: f64,
    x_title_box: Rect<f64>,
    y_title_box: Rect<f64>,
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

    bar_width: f64,
    bar_spacing: f64,
    bar_group_width: f64,
}

pub struct BarChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
{
    props: RefCell<BarChartProperties>,
    base: BaseChart<'a, C, BarEntity, M, D, BarChartOptions<'a>>,
}

impl<'a, C, M, D> BarChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
{
    pub fn new(options: BarChartOptions<'a>) -> Self {
        Self {
            props: Default::default(),
            base: BaseChart::new(options),
        }
    }

    /// Returns the x coordinate of the x-axis label at [index].
    fn x_label_x(&self, index: usize) -> f64 {
        let props = self.props.borrow();
        props.y_axis_left + props.x_label_hop * ((index as f64) + props.x_label_offset_factor)
    }

    /// Returns the y-coordinate corresponding to the data point [value] and
    /// the animation percent [percent].
    fn value_to_y(&self, value: f64) -> f64 {
        let props = self.props.borrow();
        if value != 0.0 {
            return props.x_axis_top
                - (value - props.y_min_value) / props.y_range * props.y_axis_length;
        }
        props.x_axis_top
    }

    // TODO: Separate y-axis stuff into a separate method.
    fn calculate_drawing_sizes(&self) {
        self.base.calculate_drawing_sizes();

        // y-axis min-max.
        let mut props = self.props.borrow_mut();

        props.y_max_value = if let Some(value) = self.base.options.y_axis.max_value {
            value as f64
        } else {
            f64::NEG_INFINITY
        };

        // FIXME:
        // props.y_max_value = props
        //     .y_max_value
        //     .max(utils::find_max_value(&self.base.data_table));

        if props.y_max_value == f64::NEG_INFINITY {
            props.y_max_value = 0.;
        }

        props.y_min_value = if let Some(value) = self.base.options.y_axis.min_value {
            value as f64
        } else {
            f64::INFINITY
        };

        // FIXME:
        // props.y_min_value = props
        //     .y_min_value
        //     .min(utils::find_min_value(&self.base.data_table));

        if props.y_min_value == f64::INFINITY {
            props.y_min_value = 0.;
        }

        props.y_interval = self.base.options.y_axis.interval.unwrap();
        let min_interval = self.base.options.y_axis.min_interval;

        if props.y_interval == 0. {
            if props.y_min_value == props.y_max_value {
                if props.y_min_value == 0. {
                    props.y_max_value = 1.;
                    props.y_interval = 1.;
                } else if props.y_min_value == 1. {
                    props.y_min_value = 0.;
                    props.y_interval = 1.;
                } else {
                    props.y_interval = props.y_min_value * 0.25;
                    props.y_min_value -= props.y_interval;
                    props.y_max_value += props.y_interval;
                }
                if let Some(value) = min_interval {
                    props.y_interval = props.y_interval.max(value as f64);
                }
            } else {
                props.y_interval = utils::calculate_interval(
                    props.y_max_value - props.y_min_value,
                    5,
                    min_interval.unwrap() as f64,
                );
            }
        }

        let val = props.y_min_value / props.y_interval;
        props.y_min_value = val.floor() * props.y_interval;
        props.y_max_value = val.ceil() * props.y_interval;
        props.y_range = props.y_max_value - props.y_min_value;

        // y-axis labels.
        props.y_labels = Vec::new(); //<String>[];
        props.y_label_formatter = self.base.options.y_axis.labels.formatter;

        if let None = props.y_label_formatter {
            // TODO:
            // let max_decimal_places =
            //     max(utils::get_decimal_places(props.y_interval), utils::get_decimal_places(props.y_min_value));
            // let numberFormat = NumberFormat.decimalPattern()
            // ..maximumFractionDigits = maxDecimalPlaces
            // ..minimumFractionDigits = maxDecimalPlaces;
            // y_label_formatter = numberFormat.format;
        }

        let value = props.y_min_value;
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
    }

    fn data_cell_changed(&self, record: DataCellChangeRecord<D>) {
        let mut props = self.props.borrow_mut();
        if record.column_index == 0 {
            props.x_labels[record.row_index] = format!("{}", record.new_value);
        } else {
            self.base.data_cell_changed(record);
        }
    }

    fn draw_axes_and_grid(&self, axes_context: C) {
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
        //     ..rotate(-f64::FRAC_PI_2)
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

    fn get_entity_group_index(&self, x: f64, y: f64) -> i64 {
        let props = self.props.borrow();
        let dx = x - props.y_axis_left;
        // If (x, y) is inside the rectangle defined by the two axes.
        if y > props.x_axis_top - props.y_axis_length
            && y < props.x_axis_top
            && dx > 0.
            && dx < props.x_axis_length
        {
            let index = (dx / props.x_label_hop - props.x_label_offset_factor).round() as usize;
            // If there is at least one visible point in the current point group...
            if let Some(_) = props.average_y_values.get(index) {
                return index as i64;
            }
        }
        return -1;
    }

    fn update(&self, options: HashMap<String, String>) {
        self.base.update(options);
        self.calculate_average_y_values(0);
    }

    fn get_bar_left(&self, series_index: usize, bar_index: usize) -> f64 {
        let props = self.props.borrow();
        self.x_label_x(bar_index) - 0.5 * props.bar_group_width
            + (self.base.count_visible_series(Some(series_index)) as f64)
                * (props.bar_width + props.bar_spacing)
    }

    fn update_bar_width(&self) {
        let count = self.base.count_visible_series(None);
        let mut props = self.props.borrow_mut();
        if count > 0 {
            props.bar_width =
                (props.bar_group_width + props.bar_spacing) / (count as f64) - props.bar_spacing;
        } else {
            props.bar_width = 0.;
        }
    }

    fn value_to_bar_height(&self, value: f64) -> f64 {
        if value == 0. {
            return 0.;
        }
        let props = self.props.borrow();
        props.x_axis_top - self.value_to_y(value)
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
        if !self.base.options.tooltip.enabled {
            return;
        }

        // let entity_count = self.base.series_list.first.entities.length;
        // let start = index ?? 0;
        // let end = index == null ? entityCount : index + 1;

        // average_y_values ??= <num>[];
        // average_y_values.length = entityCount;

        // for (let i = start; i < end; i++) {
        //   let sum = 0.0;
        //   let count = 0;
        //   for (let j = series_list.length - 1; j >= 0; j--) {
        //     let state = seriesStates[j];
        //     if (state == Visibility::hidden) continue;
        //     if (state == Visibility::hiding) continue;

        //     let bar = series_list[j].entities[i] as Bar;
        //     if (bar.value != null) {
        //       sum += bar.height;
        //       count++;
        //     }
        //   }
        //   average_y_values[i] = (count > 0) ? xAxisTop - sum / count : null;
        // }
    }

    fn series_visibility_changed(&self, index: usize) {
        self.update_bar_width();
        self.update_series(0);
        self.calculate_average_y_values(0);
    }
}

impl<'a, C, M, D> Chart<'a, C, M, D, BarEntity> for BarChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
{
    fn calculate_drawing_sizes(&self) {
        self.base.calculate_drawing_sizes();
        let mut props = self.props.borrow_mut();
        props.bar_group_width = 0.618 * props.x_label_hop; // Golden ratio.
        props.tooltip_offset = 0.5 * props.x_label_hop + 4.;
        self.update_bar_width();
    }

    fn set_stream(&self, stream: DataStream<'a, M, D>) {}

    fn draw(&self, ctx: C) {}

    fn draw_series(&self, percent: f64) -> bool {
        // for (let i = 0, n = series_list.length; i < n; i++) {
        //   if (series_states[i] == Visibility::hidden) continue;

        //   let series = series_list[i];

        //   // Draw the bars.
        //   for (Bar bar in series.entities) {
        //     if (bar.value == null) continue;
        //     bar.draw(series_context, percent, false);
        //   }

        //   let opt = self.base.options.x_axis.crosshair;
        //   if (focused_entity_index >= 0 && opt["enabled"]) {
        //     series_context
        //       ..fillStyle = opt["color"]
        //       ..fillRect(y_axis_left + x_label_hop * focused_entity_index,
        //           x_axis_top - y_axis_length, x_label_hop, y_axis_length);
        //   }

        //   // Draw the labels.
        //   if (percent == 1.0) {
        //     opt = self.base.options.series.labels;
        //     if (!opt["enabled"]) continue;
        //     series_context
        //       ..fillStyle = opt["style"]["color"]
        //       ..font = get_font(opt["style"])
        //       ..textAlign = "center"
        //       ..textBaseline = "alphabetic";
        //     for (Bar bar in series.entities) {
        //       if (bar.value == null) continue;
        //       let x = bar.left + .5 * bar.width;
        //       let y = x_axis_top - bar.height - 5;
        //       series_context.fill_text(bar.formatted_value, x, y);
        //     }
        //   }
        // }

        return false;
    }

    fn update_series(&self, index: usize) {
        // let entityCount = data_table.frames.length;
        // for (let i = 0; i < series_list.length; i++) {
        //   let series = series_list[i];
        //   let left = get_bar_left(i, 0);
        //   let barWidth = 0.0;
        //   if (series_states[i].index >= Visibility::showing.index) {
        //     barWidth = barWidth;
        //   }
        //   let color = get_color(i);
        //   let highlight_color = get_highlight_color(color);
        //   series.color = color;
        //   series.highlight_color = highlight_color;
        //   for (let j = 0; j < entityCount; j++) {
        //     let bar = series.entities[j] as Bar;
        //     bar.index = j;
        //     bar.color = color;
        //     bar.highlight_color = highlight_color;
        //     bar.left = left;
        //     bar.bottom = x_axis_top;
        //     bar.height = valueToBarHeight(bar.value);
        //     bar.width = barWidth;
        //     left += x_label_hop;
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
    ) -> BarEntity {
        // let left = get_bar_left(seriesIndex, entityIndex);
        // let oldLeft = left;
        // let height = valueToBarHeight(value);

        // // Animate width.
        // num oldHeight = height;
        // num oldWidth = 0;

        // if (series_list == null) {
        //   // Data table changed. Animate height.
        //   oldHeight = 0;
        //   oldWidth = barWidth;
        // }

        // BarEntity()
        //   ..index = entityIndex
        //   ..value = value
        //   ..formatted_value = value != null ? entity_value_formatter(value) : null
        //   ..color = color
        //   ..highlight_color = highlight_color
        //   ..bottom = x_axis_top
        //   ..oldLeft = oldLeft
        //   ..left = left
        //   ..oldHeight = oldHeight
        //   ..height = height
        //   ..oldWidth = oldWidth
        //   ..width = barWidth;
        unimplemented!()
    }

    fn get_tooltip_position(&self) -> Point<f64> {
        let props = self.props.borrow();
        let focused_entity_index = self.base.props.borrow().focused_entity_index;

        // FIXME: as usize
        // TODO: tooltip is a Element
        let x = self.x_label_x(focused_entity_index as usize) + props.tooltip_offset;
        // let y = max(x_axis_top - y_axis_length,
        //     average_y_values[focused_entity_index] - tooltip.offset_height ~/ 2);
        // if (x + tooltip.offset_width > width) {
        //   x -= tooltip.offset_width + 2 * tooltip_offset;
        //   x = max(x, y_axis_left);
        // }
        // return Point(x, y);
        unimplemented!()
    }
}
