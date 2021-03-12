#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};
use ux_animate::easing::{get_easing, Easing};
use ux_dataflow::*;
use ux_primitives::{
    canvas::CanvasContext,
    color::Color,
    geom::{Point, Rect, Size},
    text::{BaseLine, TextAlign, TextStyle, TextWeight},
};

use crate::*;

#[derive(Default, Clone)]
pub struct BarEntity {
    color: Color,
    highlight_color: Color,
    // formatted_value: String,
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
    fn draw(&self, ctx: &C, percent: f64, highlight: bool) {
        let x = lerp(self.old_left, self.left, percent);
        let h = lerp(self.old_height, self.height, percent);
        let w = lerp(self.old_width, self.width, percent);
        ctx.set_fill_style_color(self.color);
        ctx.fill_rect(x, self.bottom - h, w, h);
        if highlight {
            ctx.set_fill_color_rgb(255, 255, 255, 0.25);
            ctx.fill_rect(x, self.bottom - h, w, h);
        }
    }
}

impl Entity for BarEntity {
    fn free(&mut self) {}

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
    xlabel_max_width: f64,
    ylabel_max_width: f64,
    xlabel_rotation: f64, // 0..90
    xlabel_step: i64,
    /// Distance between two consecutive x-axis labels.
    xlabel_hop: f64,
    /// Distance between two consecutive x-axis labels.
    ylabel_hop: f64,
    x_title_box: Rect<f64>,
    y_title_box: Rect<f64>,
    x_title_center: Option<Point<f64>>,
    y_title_center: Option<Point<f64>>,
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
    fn xlabel_x(&self, index: usize) -> f64 {
        let props = self.props.borrow();
        props.y_axis_left + props.xlabel_hop * ((index as f64) + props.xlabel_offset_factor)
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

    fn data_cell_changed(&self, record: DataCellChangeRecord<D>) {
        let mut props = self.props.borrow_mut();
        if record.column_index == 0 {
            props.xlabels[record.row_index] = format!("{}", record.new_value);
        } else {
            self.base.data_cell_changed(record);
        }
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
            let index = (dx / props.xlabel_hop - props.xlabel_offset_factor).round() as usize;
            // If there is at least one visible point in the current point group...
            if let Some(_) = props.average_y_values.get(index) {
                return index as i64;
            }
        }
        return -1;
    }

    fn get_bar_left(&self, series_index: usize, bar_index: usize) -> f64 {
        let props = self.props.borrow();
        self.xlabel_x(bar_index) - 0.5 * props.bar_group_width
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

    /// Calculates average y values for the visible series to help position the tooltip
    ///
    /// If [index] is given, calculates the average y value for the entity group
    /// at [index] only.
    ///
    fn calculate_average_y_values(&self, index: usize) {
        if !self.base.options.tooltip.enabled {
            return;
        }

        // let entity_count = self.base.series_list.first.entities.len();
        // let start = index ?? 0;
        // let end = index == null ? entity_count : index + 1;

        // average_y_values ??= <num>[];
        // average_y_values.len() = entity_count;

        // for (let i = start; i < end; i++) {
        //   let sum = 0.0;
        //   let count = 0;
        //   for (let j = series_list.len() - 1; j >= 0; j--) {
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
    // TODO: Separate y-axis stuff into a separate method.
    fn calculate_drawing_sizes(&self) {
        self.base.calculate_drawing_sizes();
        let mut props = self.props.borrow_mut();
        props.bar_group_width = 0.618 * props.xlabel_hop; // Golden ratio.
        props.tooltip_offset = 0.5 * props.xlabel_hop + 4.;
        self.update_bar_width();

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
        props.ylabels = Vec::new(); //<String>[];
        props.ylabel_formatter = self.base.options.y_axis.labels.formatter;

        if let None = props.ylabel_formatter {
            // TODO:
            // let max_decimal_places =
            //     max(utils::get_decimal_places(props.y_interval), utils::get_decimal_places(props.y_min_value));
            // let numberFormat = NumberFormat.decimalPattern()
            // ..maximumFractionDigits = maxDecimalPlaces
            // ..minimumFractionDigits = maxDecimalPlaces;
            // ylabel_formatter = numberFormat.format;
        }

        let value = props.y_min_value;
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
        //         (self.base.options.y_axis.labels.style.font_size / 2.).trunc();
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

        // self.base.start_animation();
        self.draw_frame(ctx, None);
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
        println!("BarChart draw_axes_and_grid");
        // x-axis title.
        let props = self.props.borrow();
        if let Some(x_title_center) = props.x_title_center {
            let opt = &self.base.options.x_axis.title;

            if let Some(text) = opt.text {
                let style = &opt.style;
                ctx.save();
                ctx.set_fill_style_color(style.color);

                ctx.set_font(
                    &style.font_family.unwrap_or(DEFAULT_FONT_FAMILY),
                    style.font_style.unwrap_or(TextStyle::Normal),
                    TextWeight::Normal,
                    style.font_size.unwrap_or(12.),
                );

                ctx.set_text_align(TextAlign::Center);
                ctx.set_text_baseline(BaseLine::Middle);
                ctx.fill_text(text, x_title_center.x, x_title_center.y);
                ctx.restore();
            }
        }

        // y-axis title.
        if let Some(y_title_center) = props.y_title_center {
            let opt = &self.base.options.y_axis.title;
            if let Some(text) = opt.text {
                let style = &opt.style;
                ctx.save();
                ctx.set_fill_style_color(style.color);

                ctx.set_font(
                    &style.font_family.unwrap_or(DEFAULT_FONT_FAMILY),
                    style.font_style.unwrap_or(TextStyle::Normal),
                    TextWeight::Normal,
                    style.font_size.unwrap_or(12.),
                );

                ctx.translate(y_title_center.x, y_title_center.y);
                ctx.rotate(-std::f64::consts::FRAC_PI_2);
                ctx.set_text_align(TextAlign::Center);
                ctx.set_text_baseline(BaseLine::Middle);
                ctx.fill_text(text, 0., 0.);
                ctx.restore();
            }
        }

        // x-axis labels.
        let opt = &self.base.options.x_axis.labels;
        let style = &opt.style;
        ctx.set_fill_style_color(style.color);

        ctx.set_font(
            &style.font_family.unwrap_or(DEFAULT_FONT_FAMILY),
            style.font_style.unwrap_or(TextStyle::Normal),
            TextWeight::Normal,
            style.font_size.unwrap_or(12.),
        );

        let mut x = self.xlabel_x(0);
        let mut y = props.x_axis_top + AXIS_LABEL_MARGIN as f64 + style.font_size.unwrap_or(12.);
        let scaled_label_hop = props.xlabel_step as f64 * props.xlabel_hop;

        if props.xlabel_rotation == 0. {
            ctx.set_text_align(TextAlign::Center);
            ctx.set_text_baseline(BaseLine::Alphabetic);

            let mut idx = 0;
            while idx < props.xlabels.len() {
                ctx.fill_text(props.xlabels.get(idx).unwrap().as_str(), x, y);
                x += scaled_label_hop;
                idx += props.xlabel_step as usize;
            }
        } else {
            ctx.set_text_align(if props.xlabel_rotation < 0. {
                TextAlign::Right
            } else {
                TextAlign::Left
            });
            ctx.set_text_baseline(BaseLine::Middle);
            if props.xlabel_rotation == 90. {
                x += props.xlabel_rotation.signum()
                    * ((style.font_size.unwrap_or(12.) / 8.).trunc());
            }
            let angle = utils::deg2rad(props.xlabel_rotation);

            let mut idx = 0;
            while idx < props.xlabels.len() {
                ctx.save();
                ctx.translate(x, y);
                ctx.rotate(angle);
                ctx.fill_text(props.xlabels.get(idx).unwrap().as_str(), 0., 0.);
                ctx.restore();
                x += scaled_label_hop;
                idx += props.xlabel_step as usize;
            }
        }

        // y-axis labels.
        let opt = &self.base.options.y_axis.labels;
        let style = &opt.style;
        ctx.set_fill_style_color(opt.style.color);

        ctx.set_font(
            &style.font_family.unwrap_or(DEFAULT_FONT_FAMILY),
            style.font_style.unwrap_or(TextStyle::Normal),
            TextWeight::Normal,
            style.font_size.unwrap_or(12.),
        );

        ctx.set_text_align(TextAlign::Right);
        ctx.set_text_baseline(BaseLine::Middle);
        x = props.y_axis_left - AXIS_LABEL_MARGIN as f64;
        y = props.x_axis_top - (style.font_size.unwrap_or(12.) / 8.).trunc();
        for label in props.ylabels.iter() {
            ctx.fill_text(label.as_str(), x, y);
            y -= props.ylabel_hop;
        }

        // x grid lines - draw bottom up.
        let opt = &self.base.options.x_axis;
        if opt.grid_line_width > 0. {
            ctx.set_line_width(opt.grid_line_width);
            ctx.set_stroke_style_color(opt.grid_line_color);
            ctx.begin_path();
            y = props.x_axis_top - props.ylabel_hop;
            // TODO: should draw 2 and len - 1 lines
            for idx in 0..props.ylabels.len() {
                ctx.move_to(props.y_axis_left, y);
                ctx.line_to(props.y_axis_left + props.x_axis_length, y);
                y -= props.ylabel_hop;
            }
            ctx.stroke();
        }

        // y grid lines or x-axis ticks - draw from left to right.
        let opt = &self.base.options.y_axis;
        let mut line_width = opt.grid_line_width;
        x = props.y_axis_left;

        if props.xlabel_step > 1 {
            x = self.xlabel_x(0);
        }

        if line_width > 0. {
            y = props.x_axis_top - props.y_axis_length;
        } else {
            line_width = 1.;
            y = props.x_axis_top + AXIS_LABEL_MARGIN as f64;
        }

        ctx.set_line_width(line_width);
        ctx.set_stroke_style_color(opt.grid_line_color);
        ctx.begin_path();
        let mut idx = 0;
        while idx < props.xlabels.len() {
            ctx.move_to(x, y);
            ctx.line_to(x, props.x_axis_top);
            x += scaled_label_hop;
            idx += props.xlabel_step as usize;
        }
        ctx.stroke();

        // x-axis itself.
        let opt = &self.base.options.x_axis;
        if opt.line_width > 0. {
            ctx.set_line_width(opt.line_width);
            ctx.set_stroke_style_color(opt.line_color);
            ctx.begin_path();
            ctx.move_to(props.y_axis_left, props.x_axis_top);
            ctx.line_to(props.y_axis_left + props.x_axis_length, props.x_axis_top);
            ctx.stroke();
        }

        // y-axis itself.
        let opt = &self.base.options.y_axis;
        if opt.line_width > 0. {
            ctx.set_line_width(opt.line_width);
            ctx.set_stroke_style_color(opt.line_color);
            ctx.begin_path();
            ctx.move_to(props.y_axis_left, props.x_axis_top - props.y_axis_length);
            ctx.line_to(props.y_axis_left, props.x_axis_top);
            ctx.stroke();
        }
    }

    /// Draws the current animation frame.
    ///
    /// If [time] is `null`, draws the last frame (i.e. no animation).
    fn draw_frame(&self, ctx: &C, time: Option<i64>) {
        println!("BarChart draw_frame");
        self.base.draw_frame(ctx, time);

        // self.ctx.clearRect(0, 0, self.width, self.height);
        self.draw_axes_and_grid(ctx);

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

        let ease = match props.easing_function {
            Some(val) => val,
            None => get_easing(Easing::Linear),
        };

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
        println!("BarChart draw_series");
        let series_list = self.base.series_list.borrow();
        let series_states = &self.base.props.borrow().series_states;
        let focused_entity_index = self.base.props.borrow().focused_entity_index;

        let crosshair = &self.base.options.x_axis.crosshair;
        let labels = &self.base.options.series.labels;
        let props = self.props.borrow();

        for idx in 0..series_list.len() {
            if series_states[idx] == Visibility::Hidden {
                continue;
            }

            let series = series_list.get(idx).unwrap();

            // Draw the bars.
            for entity in series.entities.iter() {
                if entity.value == 0. {
                    continue;
                }
                entity.draw(ctx, percent, false);
            }

            if let Some(crosshair) = crosshair {
                if focused_entity_index >= 0 {
                    ctx.set_fill_style_color(crosshair.color);
                    ctx.fill_rect(
                        props.y_axis_left + props.xlabel_hop * focused_entity_index as f64,
                        props.x_axis_top - props.y_axis_length,
                        props.xlabel_hop,
                        props.y_axis_length,
                    );
                }

                // Draw the labels.
                if percent == 1.0 {
                    if let Some(labels) = labels {
                        ctx.set_fill_style_color(labels.color);
                        ctx.set_font(labels.font_family.unwrap_or("Roboto"), labels.font_style.unwrap_or(TextStyle::Normal), 
                        TextWeight::Normal,
                        labels.font_size.unwrap_or(12.));
                        ctx.set_text_align(TextAlign::Center);
                        ctx.set_text_baseline(BaseLine::Alphabetic);

                        for entity in series.entities.iter() {
                          if entity.value == 0. {
                              continue;
                          }
                          let x = entity.left + 0.5 * entity.width;
                          let y = props.x_axis_top - entity.height - 5.;
                          // TODO: bar.formatted_value
                          let formatted_value = format!("{}", entity.value);
                          ctx.fill_text(formatted_value.as_str(), x, y);
                        }
                    }
                }
            }
        }

        return false;
    }

    fn update_series(&self, index: usize) {
        let entity_count = self.base.data_table.frames.len();
        let mut series_list = self.base.series_list.borrow_mut();
        let props = self.props.borrow();
        let series_states = &self.base.props.borrow().series_states;

        for idx in 0..series_list.len() {
            let mut series = series_list.get_mut(idx).unwrap();
            let mut left = self.get_bar_left(idx, 0);
            let mut bar_width = 0.0;

            let series_state = series_states[idx];

            if series_state == Visibility::Showing || series_state == Visibility::Shown {
                bar_width = bar_width;
            }

            let color = self.base.get_color(idx);
            let highlight_color = self.base.get_highlight_color(color);
            series.color = color;
            series.highlight_color = highlight_color;

            for jdx in 0..entity_count {
                let mut entity = series.entities.get_mut(jdx).unwrap();
                entity.index = jdx;
                entity.color = color;
                entity.highlight_color = highlight_color;
                entity.left = left;
                entity.bottom = props.x_axis_top;
                entity.height = self.value_to_bar_height(entity.value);
                entity.width = bar_width;
                left += props.xlabel_hop;
            }
        }
    }

    fn create_entity(
        &self,
        series_index: usize,
        entity_index: usize,
        value: f64,
        color: Color,
        highlight_color: Color,
    ) -> BarEntity {
        let left = self.get_bar_left(series_index, entity_index);
        let old_left = left;
        let height = self.value_to_bar_height(value);

        // Animate width.
        let mut old_height = height;
        let mut old_width = 0.;

        let props = self.props.borrow();
        let series_list = self.base.series_list.borrow();
        if series_list.len() == 0 {
            // Data table changed. Animate height.
            old_height = 0.;
            old_width = props.bar_width;
        }

        BarEntity {
            index: entity_index,
            old_value: 0.,
            value,
            //   formatted_value: value != null ? entity_value_formatter(value) : null
            color,
            highlight_color,
            bottom: props.x_axis_top,
            old_left,
            left,
            old_height,
            height,
            old_width,
            width: props.bar_width,
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
