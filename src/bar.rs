#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use animate::easing::{get_easing, Easing};
use dataflow::*;
use primitives::{
    BaseLine, CanvasContext, Color, Point, Rect, Size, TextAlign, TextStyle, TextWeight,
};
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

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
        let x = utils::lerp(self.old_left, self.left, percent);
        let h = utils::lerp(self.old_height, self.height, percent);
        let w = utils::lerp(self.old_width, self.width, percent);
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

        let mut props = self.props.borrow_mut();
        let entity_count = self
            .base
            .series_list
            .borrow()
            .first()
            .unwrap()
            .entities
            .len();
        let start = if index == 0 { index } else { 0 };
        let end = if index == 0 { entity_count } else { index + 1 };

        props
            .average_y_values
            .resize(entity_count, Default::default());

        let series_list = self.base.series_list.borrow();
        let series_states = &self.base.props.borrow().series_states;

        for idx in start..end {
            let mut sum = 0.0;
            let mut count = 0;
            for jdx in series_list.len()..0 {
                let series_state = series_states[idx];

                if series_state == Visibility::Hidden || series_state == Visibility::Hiding {
                    continue;
                }

                let series = series_list.get(jdx).unwrap();

                let bar = series.entities.get(idx).unwrap();
                if bar.value != 0. {
                    sum += bar.height;
                    count += 1;
                }
            }
            props.average_y_values[idx] = if count > 0 {
                props.x_axis_top - sum / count as f64
            } else {
                0.
            };
        }
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
        let mut baseprops = self.base.props.borrow_mut();
        props.bar_group_width = 0.618 * props.xlabel_hop; // Golden ratio.
        props.tooltip_offset = 0.5 * props.xlabel_hop + 4.;
        self.update_bar_width();

        // y-axis min-max.
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

        // y-axis labels
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

        let mut value = props.y_min_value;
        while value <= props.y_max_value {
            let ylabel_formatter = props.ylabel_formatter.unwrap();
            props.ylabels.push(ylabel_formatter(value));
            value += props.y_interval;
        }

        // TODO: fix me
        // props.ylabel_max_width = utils::calculate_max_text_width(
        //         context, get_font(self.base.options.y_axis.labels.style), ylabels)
        //     .round();

        baseprops.entity_value_formatter = props.ylabel_formatter;

        // Tooltip
        let options = &self.base.options;
        baseprops.tooltip_value_formatter = if let Some(formater) = options.tooltip.value_formatter
        {
            Some(formater)
        } else {
            props.ylabel_formatter
        };

        let series_and_axes_box = &baseprops.series_and_axes_box;

        // x-axis title
        let mut xtitle_left = 0.;
        let xtitle_top = 0.;
        let xtitle_width = 0.;
        let xtitle_height = 0.;
        let xtitle = &self.base.options.x_axis.title;

        // if xtitle.text != null {
        // ctx.set_font(
        //     xtitle.font_family.unwrap_or(DEFAULT_FONT_FAMILY),
        //     xtitle.font_style.unwrap_or(TextStyle::Normal),
        //     TextWeight::Normal,
        //     xtitle.font_size.unwrap_or(12.),
        // );
        //     xtitle_width = context.measure_text(xtitle.text).width.round() +
        //         2 * TITLE_PADDING;
        //     xtitle_height = xtitle.style.font_size + 2 * TITLE_PADDING;
        //     xtitle_top = baseprops.series_and_axes_box.bottom - xtitle_height;
        // }

        // y-axis title
        let ytitle_left = 0.;
        let ytitle_top = 0.;
        let ytitle_width = 0.;
        let ytitle_height = 0.;
        let ytitle = &self.base.options.y_axis.title;

        // if ytitle.text != null {
        // ctx.set_font(
        //     ytitle.font_family.unwrap_or(DEFAULT_FONT_FAMILY),
        //     ytitle.font_style.unwrap_or(TextStyle::Normal),
        //     TextWeight::Normal,
        //     ytitle.font_size.unwrap_or(12.),
        // );
        //     ytitle_height = context.measure_text(ytitle.text).width.round() +
        //         2 * TITLE_PADDING;
        //     ytitle_width = ytitle.style.font_size + 2 * TITLE_PADDING;
        //     ytitle_left = series_and_axes_box.left;
        // }

        // Axes" size and position
        props.y_axis_left =
            series_and_axes_box.origin.x + props.ylabel_max_width + AXIS_LABEL_MARGIN as f64;
        if ytitle_width > 0. {
            props.y_axis_left += ytitle_width + CHART_TITLE_MARGIN;
        } else {
            props.y_axis_left += AXIS_LABEL_MARGIN as f64;
        }

        props.x_axis_length =
            (series_and_axes_box.origin.x + series_and_axes_box.size.width) - props.y_axis_left;

        props.x_axis_top = series_and_axes_box.origin.y + series_and_axes_box.size.height;
        if xtitle_height > 0. {
            props.x_axis_top -= xtitle_height + CHART_TITLE_MARGIN;
        } else {
            props.x_axis_top -= AXIS_LABEL_MARGIN as f64;
        }
        props.x_axis_top -= AXIS_LABEL_MARGIN as f64;

        // x-axis labels and x-axis"s position.
        let row_count = self.base.data_table.meta.len();
        props.xlabels = Vec::new();
        for idx in 0..row_count {
            let row = self.base.data_table.meta.get(idx).unwrap();
            props.xlabels.push(row.name.to_string());
        }

        // TODO: fix me
        // props.xlabel_max_width = utils::calculate_max_text_width(
        //     context,
        //     get_font(self.base.options.x_axis.labels.style),
        //     props.xlabels,
        // );
        if props.xlabel_offset_factor > 0. && row_count > 1 {
            props.xlabel_hop = props.x_axis_length / row_count as f64;
        } else if row_count > 1 {
            props.xlabel_hop = props.x_axis_length / (row_count - 1) as f64;
        } else {
            props.xlabel_hop = props.x_axis_length;
        }

        props.xlabel_rotation = 0.;

        let font_size = self.base.options.x_axis.labels.style.font_size.unwrap();
        let max_rotation = self.base.options.x_axis.labels.max_rotation;
        let min_rotation = self.base.options.x_axis.labels.min_rotation;
        let angles = vec![0, -45, 45, -90, 90];

        // outer:
        for step in 1..row_count {
            let scaled_label_hop = step as f64 * props.xlabel_hop;
            let min_spacing = (0.1 * scaled_label_hop as f64).max(10.);
            for angle in angles.iter() {
                let angle = *angle;
                if angle > max_rotation || angle < min_rotation {
                    continue;
                }

                let abs_angle_rad = utils::deg2rad(angle as f64).abs();
                let label_spacing = if angle == 0 {
                    scaled_label_hop - props.xlabel_max_width
                } else {
                    scaled_label_hop * abs_angle_rad.sin() - font_size
                };

                if label_spacing < min_spacing {
                    continue;
                }

                props.xlabel_rotation = angle as f64;
                props.xlabel_step = step as i64;
                props.x_axis_top -=
                    props.xlabel_max_width * abs_angle_rad.sin() + font_size * abs_angle_rad.cos();
                // TODO: fixme
                // break outer;
            }
        }

        // Wrap up.
        props.y_axis_length = props.x_axis_top
            - series_and_axes_box.origin.y
            - (self.base.options.y_axis.labels.style.font_size.unwrap() / 2.).trunc();
        props.ylabel_hop = props.y_axis_length / (props.ylabels.len() - 1) as f64;

        xtitle_left = props.y_axis_left + ((props.x_axis_length - xtitle_width) / 2.).trunc();

        let ytitle_top =
            series_and_axes_box.origin.y + ((props.y_axis_length - ytitle_height) / 2.).trunc();

        if xtitle_height > 0. {
            //      x_title_box =
            //          Rectangle(xTitleLeft, xTitleTop, xTitleWidth, xTitleHeight);
            props.x_title_center = Some(Point::new(
                xtitle_left + (xtitle_width / 2.).trunc(),
                xtitle_top + (xtitle_height / 2.).trunc(),
            ));
        } else {
            //      x_title_box = null;
            props.x_title_center = None;
        }

        if ytitle_height > 0. {
            //      y_title_box =
            //          Rectangle(yTitleLeft, yTitleTop, yTitleWidth, yTitleHeight);
            props.y_title_center = Some(Point::new(
                ytitle_left + (ytitle_width / 2.).trunc(),
                ytitle_top + (ytitle_height / 2.).trunc(),
            ));
        } else {
            //      y_title_box = null;
            props.y_title_center = None;
        }
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
                        ctx.set_font(
                            labels.font_family.unwrap_or(DEFAULT_FONT_FAMILY),
                            labels.font_style.unwrap_or(TextStyle::Normal),
                            TextWeight::Normal,
                            labels.font_size.unwrap_or(12.),
                        );
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
