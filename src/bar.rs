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
pub struct BarEntity<D> {
    color: Color,
    highlight_color: Color,
    // formatted_value: String,
    index: usize,
    old_value: Option<D>,
    value: Option<D>,

    old_left: f64,
    old_width: f64,
    old_height: f64,
    bottom: f64,
    left: f64,
    width: f64,
    height: f64,
}

impl<D> BarEntity<D> {
    pub fn get_right(&self) -> f64 {
        self.left + self.width
    }
}

impl<C, D> Drawable<C> for BarEntity<D>
where
    C: CanvasContext,
{
    fn draw(&self, ctx: &C, percent: f64, highlight: bool) {
        let x = utils::lerp(self.old_left, self.left, percent);
        let h = utils::lerp(self.old_height, self.height, percent);
        //FIXME:
        let w = utils::lerp(self.old_width, 10. /*self.width*/, percent);
        let y = self.bottom - h;
        // info!("draw entity {}, {}, {}, {}", x.round(), y.round(), w.round(), h.round());

        ctx.set_fill_color(self.color);
        ctx.fill_rect(x, y, w, h);
        if highlight {
            ctx.set_fill_color(Color::RGBA(255, 255, 255, 25));
            ctx.fill_rect(x, y, w, h);
        }
    }
}

impl<D> Entity for BarEntity<D> {
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
    yinterval: f64,
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
    D: fmt::Display + Copy,
{
    props: RefCell<BarChartProperties>,
    base: BaseChart<'a, C, BarEntity<D>, M, D, BarChartOptions<'a>>,
}

impl<'a, C, M, D> BarChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display + Copy + Into<f64> + Ord + Default,
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
    fn value_to_y(&self, value: Option<D>) -> f64 {
        let props = self.props.borrow();
        match value {
            Some(value) => {
                props.x_axis_top
                    - (value.into() - props.y_min_value) / props.y_range * props.y_axis_length
            }
            None => props.x_axis_top,
        }
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

    fn update_bar_width(&self) {
        let count = self.count_visible_channel(None);
        let mut props = self.props.borrow_mut();
        if count > 0 {
            props.bar_width =
                (props.bar_group_width + props.bar_spacing) / (count as f64) - props.bar_spacing;
        } else {
            props.bar_width = 0.;
        }
    }

    fn get_bar_left(&self, channel_index: usize, bar_index: usize) -> f64 {
        let props = self.props.borrow();
        self.xlabel_x(bar_index) - 0.5 * props.bar_group_width
            + (self.count_visible_channel(Some(channel_index)) as f64)
                * (props.bar_width + props.bar_spacing)
    }

    /// Counts the number of visible channel up to (but not including) the [end]th
    /// channel.
    pub fn count_visible_channel(&self, end: Option<usize>) -> usize {
        let channels = self.base.channels.borrow();

        let end = match end {
            Some(end) => end,
            None => channels.len(),
        };

        channels
            .iter()
            .take(end)
            .filter(|&channel| {
                channel.state == Visibility::Showing || channel.state == Visibility::Shown
            })
            .count()
    }

    fn value_to_bar_height(&self, value: Option<D>) -> f64 {
        match value {
            None => 0.0,
            Some(_) => {
                let props = self.props.borrow();
                props.x_axis_top - self.value_to_y(value)
            }
        }
    }

    /// Calculates average y values for the visible channel to help position the tooltip
    ///
    /// If [index] is given, calculates the average y value for the entity group
    /// at [index] only.
    ///
    fn calculate_average_y_values(&self, index: usize) {
        if !self.base.options.tooltip.enabled {
            return;
        }

        let channels = self.base.channels.borrow();

        if !channels.is_empty() {
            let mut props = self.props.borrow_mut();
            let entity_count = channels.first().unwrap().entities.len();
            let start = if index == 0 { index } else { 0 };
            let end = if index == 0 { entity_count } else { index + 1 };

            props
                .average_y_values
                .resize(entity_count, Default::default());

            let channels = self.base.channels.borrow();
            // let channel_states = &self.base.props.borrow().channel_states;

            for idx in start..end {
                let mut sum = 0.0;
                let mut count = 0;
                for channel in channels.iter() {
                    if channel.state == Visibility::Hidden || channel.state == Visibility::Hiding {
                        continue;
                    }

                    let bar = channel.entities.get(idx).unwrap();
                    if bar.value.is_none() {
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
    }

    fn channel_visibility_changed(&self, index: usize) {
        self.update_bar_width();
        self.update_channel(0);
        self.calculate_average_y_values(0);
    }

    /// Called when [data_table] has been changed.
    fn data_table_changed(&self) {
        info!("data_table_changed");
        // self.calculate_drawing_sizes(ctx);
        self.create_channels(0, self.base.data_table.meta.len());
    }

    fn get_channel_lefts(&self) -> Vec<f64> {
        let mut result = Vec::new();
        let channels = self.base.channels.borrow();
        let mut idx = 0;
        for channel in channels.iter() {
            result.push(self.get_bar_left(idx, 0));
            idx += 1;
        }
        result
    }
}

impl<'a, C, M, D> Chart<'a, C, M, D, BarEntity<D>> for BarChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display + Copy + Into<f64> + Ord + Default,
{
    // TODO: Separate y-axis stuff into a separate method.
    fn calculate_drawing_sizes(&self, ctx: &C) {
        info!("calculate_drawing_sizes");
        self.base.calculate_drawing_sizes(ctx);

        {
            let mut props = self.props.borrow_mut();
            props.bar_group_width = 0.618 * props.xlabel_hop; // Golden ratio.
            props.tooltip_offset = 0.5 * props.xlabel_hop + 4.;
        }

        self.update_bar_width();

        let mut props = self.props.borrow_mut();
        let mut baseprops = self.base.props.borrow_mut();
        let options = &self.base.options;

        // y-axis min-max.
        props.y_max_value = if let Some(value) = options.y_axis.max_value {
            value as f64
        } else {
            f64::NEG_INFINITY
        };

        props.y_max_value = props
            .y_max_value
            .max(utils::find_max_value(&self.base.data_table).into());

        if props.y_max_value == f64::NEG_INFINITY {
            props.y_max_value = 0.;
        }

        props.y_min_value = if let Some(value) = options.y_axis.min_value {
            value as f64
        } else {
            f64::INFINITY
        };
        
        props.y_min_value = props
            .y_min_value
            .min(utils::find_min_value(&self.base.data_table).into());

        if props.y_min_value == f64::INFINITY {
            props.y_min_value = 0.;
        }

        if let Some(value) = options.y_axis.interval {
            props.yinterval = value
        } else {
            let min_interval = options.y_axis.min_interval;
            if props.y_min_value == props.y_max_value {
                if props.y_min_value == 0. {
                    props.y_max_value = 1.;
                    props.yinterval = 1.;
                } else if props.y_min_value == 1. {
                    props.y_min_value = 0.;
                    props.yinterval = 1.;
                } else {
                    props.yinterval = props.y_min_value * 0.25;
                    props.y_min_value -= props.yinterval;
                    props.y_max_value += props.yinterval;
                }
                if let Some(value) = min_interval {
                    props.yinterval = props.yinterval.max(value as f64);
                }
            } else {
                props.yinterval = utils::calculate_interval(
                    props.y_max_value - props.y_min_value,
                    5,
                    min_interval,
                );
            }
        }

        let val = props.y_min_value / props.yinterval;

        props.y_min_value = (props.y_min_value / props.yinterval).floor() * props.yinterval;
        props.y_max_value = (props.y_max_value / props.yinterval).ceil() * props.yinterval;

        props.y_range = props.y_max_value - props.y_min_value;

        // y-axis labels
        props.ylabels = Vec::new();
        props.ylabel_formatter = options.y_axis.labels.formatter;

        if let None = props.ylabel_formatter {
            // let max_decimal_places =
            //     max(utils::get_decimal_places(props.yinterval), utils::get_decimal_places(props.y_min_value));
            // let numberFormat = NumberFormat.decimalPattern()
            // ..maximumFractionDigits = max_decimal_places
            // ..minimumFractionDigits = max_decimal_places;
            // ylabel_formatter = numberFormat.format;
            let a = |x: f64| -> String { x.to_string() };
            props.ylabel_formatter = Some(a);
        }

        if let Some(ylabel_formatter) = props.ylabel_formatter {
            let mut value = props.y_min_value;
            while value <= props.y_max_value {
                let ylabel_formatter = ylabel_formatter;
                props.ylabels.push(ylabel_formatter(value));
                value += props.yinterval;
            }
        } else {
            error!("NO Y LABEL FORMATTER");
        }

        props.ylabel_max_width = utils::calculate_max_text_width(
            ctx,
            &options.y_axis.labels.style,
            &props.ylabels,
        );

        baseprops.entity_value_formatter = props.ylabel_formatter;

        // Tooltip
        baseprops.tooltip_value_formatter = if let Some(formater) = options.tooltip.value_formatter
        {
            Some(formater)
        } else {
            props.ylabel_formatter
        };

        let channel_and_axes_box = &baseprops.channel_and_axes_box;

        // x-axis title
        let mut xtitle_left = 0.;
        let mut xtitle_top = 0.;
        let mut xtitle_width = 0.;
        let mut xtitle_height = 0.;
        let xtitle = &options.x_axis.title;

        if let Some(text) = xtitle.text {
            let style = &xtitle.style;
            ctx.set_font(
                style.font_family.unwrap_or(DEFAULT_FONT_FAMILY),
                style.font_style.unwrap_or(TextStyle::Normal),
                TextWeight::Normal,
                style.font_size.unwrap_or(12.),
            );
            xtitle_width = ctx.measure_text(text).width.round() + 2. * TITLE_PADDING;
            xtitle_height = xtitle.style.font_size.unwrap_or(12.) + 2. * TITLE_PADDING;
            xtitle_top =
                channel_and_axes_box.origin.y + channel_and_axes_box.size.height - xtitle_height;
        }

        // y-axis title
        let mut ytitle_left = 0.;
        let ytitle_top = 0.;
        let mut ytitle_width = 0.;
        let mut ytitle_height = 0.;
        let ytitle = &options.y_axis.title;

        if let Some(text) = ytitle.text {
            let style = &ytitle.style;
            ctx.set_font(
                style.font_family.unwrap_or(DEFAULT_FONT_FAMILY),
                style.font_style.unwrap_or(TextStyle::Normal),
                TextWeight::Normal,
                style.font_size.unwrap_or(12.),
            );
            ytitle_height = ctx.measure_text(text).width.round() + 2. * TITLE_PADDING;
            ytitle_width = ytitle.style.font_size.unwrap_or(12.) + 2. * TITLE_PADDING;
            ytitle_left = channel_and_axes_box.origin.x;
        }

        // Axes" size and position
        props.y_axis_left =
            channel_and_axes_box.origin.x + props.ylabel_max_width + AXIS_LABEL_MARGIN as f64;
        if ytitle_width > 0. {
            props.y_axis_left += ytitle_width + CHART_TITLE_MARGIN;
        } else {
            props.y_axis_left += AXIS_LABEL_MARGIN as f64;
        }

        props.x_axis_length =
            (channel_and_axes_box.origin.x + channel_and_axes_box.size.width) - props.y_axis_left;

        props.x_axis_top = channel_and_axes_box.origin.y + channel_and_axes_box.size.height;

        if xtitle_height > 0. {
            props.x_axis_top -= xtitle_height + CHART_TITLE_MARGIN;
        } else {
            props.x_axis_top -= AXIS_LABEL_MARGIN as f64;
        }

        props.x_axis_top -= AXIS_LABEL_MARGIN as f64;

        // x-axis labels and x-axis"s position.
        props.xlabels = Vec::new();

        for frame in self.base.data_table.frames.iter() {
            props.xlabels.push(frame.metric.to_string());
        }

        props.xlabel_max_width = utils::calculate_max_text_width(
            ctx,
            &options.x_axis.labels.style,
            &props.xlabels,
        );

        let row_count = self.base.data_table.frames.len() + 1;
        props.xlabel_hop = if props.xlabel_offset_factor > 0. && row_count > 1 {
            props.x_axis_length / row_count as f64
        } else if row_count > 1 {
            props.x_axis_length / (row_count - 1) as f64
        } else {
            props.x_axis_length
        };

        props.xlabel_rotation = 0.;

        let font_size = options.x_axis.labels.style.font_size.unwrap();
        let max_rotation = options.x_axis.labels.max_rotation;
        let min_rotation = options.x_axis.labels.min_rotation;
        let angles = [0, -45, 45, -90, 90];
        
        props.xlabel_step = 1;

        // // outer:
        // for step in 1..row_count {
        //     let scaled_label_hop = step as f64 * props.xlabel_hop;
        //     let min_spacing = (0.1 * scaled_label_hop as f64).max(10.);
        //     // props.xlabel_step = step as i64;
        //     props.xlabel_step = 1;

        //     for angle in angles.iter() {
        //         let angle = *angle;
        //         if angle > max_rotation || angle < min_rotation {
        //             continue;
        //         }

        //         let abs_angle_rad = utils::deg2rad(angle as f64).abs();
        //         let label_spacing = if angle == 0 {
        //             scaled_label_hop - props.xlabel_max_width
        //         } else {
        //             scaled_label_hop * abs_angle_rad.sin() - font_size
        //         };

        //         if label_spacing < min_spacing {
        //             continue;
        //         }

        //         // props.xlabel_rotation = angle as f64;

        //         // FIXME:
        //         // props.x_axis_top -=
        //         //     props.xlabel_max_width * abs_angle_rad.sin() + font_size * abs_angle_rad.cos();
        //         // TODO: fixme
        //         // break outer;
        //     }
        // }

        // warn!("LAST-X AXIS {}", props.x_axis_top);

        // Wrap up.
        props.y_axis_length = props.x_axis_top
            - channel_and_axes_box.origin.y
            - (options.y_axis.labels.style.font_size.unwrap() / 2.).trunc();
        props.ylabel_hop = props.y_axis_length / props.ylabels.len() as f64;

        xtitle_left = props.y_axis_left + ((props.x_axis_length - xtitle_width) / 2.).trunc();

        let ytitle_top =
            channel_and_axes_box.origin.y + ((props.y_axis_length - ytitle_height) / 2.).trunc();

        if xtitle_height > 0. {
            props.x_title_box = Rect::new(
                Point::new(xtitle_left, xtitle_top),
                Size::new(xtitle_width, xtitle_height),
            );
            props.x_title_center = Some(Point::new(
                xtitle_left + (xtitle_width / 2.).trunc(),
                xtitle_top + (xtitle_height / 2.).trunc(),
            ));
        } else {
            props.x_title_center = None;
        }

        if ytitle_height > 0. {
            props.y_title_box = Rect::new(
                Point::new(ytitle_left, ytitle_top),
                Size::new(ytitle_width, ytitle_height),
            );
            props.y_title_center = Some(Point::new(
                ytitle_left + (ytitle_width / 2.).trunc(),
                ytitle_top + (ytitle_height / 2.).trunc(),
            ));
        } else {
            props.y_title_center = None;
        }
    }

    fn set_stream(&mut self, stream: DataStream<'a, M, D>) {
        self.base.data_table = stream;
    }

    fn draw(&self, ctx: &C) {
        info!("draw");
        self.base.dispose();
        // data_tableSubscriptionTracker
        //   ..add(dataTable.onCellChange.listen(data_cell_changed))
        //   ..add(dataTable.onColumnsChange.listen(dataColumnsChanged))
        //   ..add(dataTable.onRowsChange.listen(data_rows_changed));
        // self.easing = get_easing(self.options.animation().easing);
        self.base.initialize_legend();
        self.base.initialize_tooltip();

        self.base.draw(ctx);

        self.base.stop_animation();
        self.data_table_changed();
        self.base.position_legend();

        // This call is redundant for row and column changes but necessary for
        // cell changes.
        self.calculate_drawing_sizes(ctx);
        self.update_channel(0);

        self.calculate_average_y_values(0);

        self.base.start_animation();
        self.draw_frame(ctx, None);
    }

    fn resize(&self, w: f64, h: f64) {
        self.base.resize(w, h);
    }

    /// Draws the axes and the grid.
    ///
    fn draw_axes_and_grid(&self, ctx: &C) {
        info!("draw_axes_and_grid");
        let options = &self.base.options;
        // x-axis title.
        let props = self.props.borrow();
        if let Some(x_title_center) = props.x_title_center {
            info!("== draw x title");
            let opt = &options.x_axis.title;

            if let Some(text) = opt.text {
                let style = &opt.style;
                ctx.save();
                ctx.set_fill_color(style.color);

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
            info!("== draw y title");
            let opt = &options.y_axis.title;
            if let Some(text) = opt.text {
                let style = &opt.style;
                ctx.save();
                ctx.set_fill_color(style.color);

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
        let x_axis = &options.x_axis;
        let style = &x_axis.labels.style;
        ctx.set_fill_color(style.color);

        ctx.set_font(
            &style.font_family.unwrap_or(DEFAULT_FONT_FAMILY),
            style.font_style.unwrap_or(TextStyle::Normal),
            TextWeight::Normal,
            style.font_size.unwrap_or(12.),
        );

        let mut x = self.xlabel_x(0);
        let mut y = props.x_axis_top + AXIS_LABEL_MARGIN as f64 + style.font_size.unwrap_or(12.);
        let scaled_label_hop = props.xlabel_step as f64 * props.xlabel_hop;

        debug!(
            "xlabel rotation [{}] and labels is [{}] {} {}",
            props.xlabel_rotation,
            props.xlabels.len(),
            props.xlabel_step,
            props.xlabel_hop.round()
        );

        if props.xlabel_rotation == 0. {
            ctx.set_text_align(TextAlign::Center);
            ctx.set_text_baseline(BaseLine::Alphabetic);

            let mut idx = 0;
            for xlabel in props.xlabels.iter() {
                let text = xlabel.as_str();
                let w = ctx.measure_text(text).width;
                let offset = (scaled_label_hop - w) / 2.;
                ctx.fill_text(xlabel.as_str(), x + offset, y);
                x += scaled_label_hop;
                idx += props.xlabel_step as usize;
            }
        } else {
            warn!("X for label {}", x);
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
            warn!("1X for label {}", x);

            let mut idx = 0;
            while idx < props.xlabels.len() {
                if let Some(text) = props.xlabels.get(idx) {
                    // info!("XLABEL: [{}] [{}:{}]", text, x.round(), y.round());
                    ctx.save();
                    ctx.translate(x, y);
                    ctx.rotate(angle);
                    ctx.fill_text(text.as_str(), 0., 0.);
                    ctx.restore();
                } else {
                    error!("No xlabel at [{}]", idx)
                }

                x += scaled_label_hop;
                idx += props.xlabel_step as usize;
            }
        }

        // y-axis labels.
        let y_axis = &options.y_axis;
        let style = &y_axis.labels.style;
        ctx.set_fill_color(y_axis.labels.style.color);

        ctx.set_font(
            &style.font_family.unwrap_or(DEFAULT_FONT_FAMILY),
            style.font_style.unwrap_or(TextStyle::Normal),
            TextWeight::Normal,
            style.font_size.unwrap_or(12.),
        );

        {
            // seems y-labels
            ctx.set_text_align(TextAlign::Right);
            ctx.set_text_baseline(BaseLine::Middle);
            x = props.y_axis_left - AXIS_LABEL_MARGIN as f64;
            y = props.x_axis_top - (style.font_size.unwrap_or(12.) / 8.).trunc();
            for label in props.ylabels.iter() {
                ctx.fill_text(label.as_str(), x, y);
                y -= props.ylabel_hop;
            }
        }

        // x grid lines - draw bottom up.
        let x_axis = &options.x_axis;
        if x_axis.grid_line_width > 0. {
            ctx.set_line_width(x_axis.grid_line_width);
            ctx.set_stroke_color(x_axis.grid_line_color);
            ctx.begin_path();
            
            // skip zero line
            y = props.x_axis_top - props.ylabel_hop;
            for idx in 1..props.ylabels.len() {
                ctx.move_to(props.y_axis_left, y);
                ctx.line_to(props.y_axis_left + props.x_axis_length, y);
                y -= props.ylabel_hop;
            }
            ctx.stroke();
        }

        // y grid lines or x-axis ticks - draw from left to right.
        let y_axis = &options.y_axis;
        let mut line_width = y_axis.grid_line_width;
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

        {
            // seems it ticks
            ctx.set_line_width(line_width);
            ctx.set_stroke_color(y_axis.grid_line_color);
            ctx.begin_path();
            let mut idx = 0;
            // draw ticks with final tick
            while idx < props.xlabels.len() + 1 {
                ctx.move_to(x, y);
                ctx.line_to(x, props.x_axis_top);
                x += scaled_label_hop;
                idx += props.xlabel_step as usize;
            }
            ctx.stroke();
        }

        // x-axis itself.
        if x_axis.line_width > 0. {
            // warn!("DRAW X-AXIS");
            ctx.set_line_width(x_axis.line_width);
            ctx.set_stroke_color(x_axis.line_color);
            ctx.begin_path();
            ctx.move_to(props.y_axis_left, props.x_axis_top);
            ctx.line_to(props.y_axis_left + props.x_axis_length, props.x_axis_top);
            ctx.stroke();
        }

        // y-axis itself.
        if y_axis.line_width > 0. {
            // warn!("DRAW Y-AXIS");
            ctx.set_line_width(y_axis.line_width);
            ctx.set_stroke_color(y_axis.line_color);
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
        // clear surface
        self.base.draw_frame(ctx, time);

        self.draw_axes_and_grid(ctx);

        let mut percent = self.base.calculate_percent(time);

        if percent >= 1.0 {
            percent = 1.0;

            // Update the visibility states of all channel before the last frame.
            let mut channels = self.base.channels.borrow_mut();

            for channel in channels.iter_mut() {
                if channel.state == Visibility::Showing {
                    channel.state = Visibility::Shown;
                } else if channel.state == Visibility::Hiding {
                    channel.state = Visibility::Hidden;
                }
            }
        }

        let props = self.base.props.borrow();

        let ease = match props.easing {
            Some(val) => val,
            None => get_easing(Easing::Linear),
        };

        self.draw_channels(ctx, ease(percent));
        // ctx.drawImageScaled(ctx.canvas, 0, 0, width, height);
        // ctx.drawImageScaled(ctx.canvas, 0, 0, width, height);
        self.base.draw_title(ctx);

        if percent < 1.0 {
            // animation_frame_id = window.requestAnimationFrame(draw_frame);
        } else if time.is_some() {
            self.base.animation_end();
        }
    }

    fn draw_channels(&self, ctx: &C, percent: f64) -> bool {
        info!("draw_channels");
        let channels = self.base.channels.borrow();
        let focused_entity_index = self.base.props.borrow().focused_entity_index;

        let crosshair = &self.base.options.x_axis.crosshair;
        let labels = &self.base.options.channel.labels;
        let props = self.props.borrow();

        for channel in channels.iter() {
            if channel.state == Visibility::Hidden {
                info!("skip draw_channel: {}", channel.name);
                continue;
            }

            info!("draw channel: {} {}", channel.name, percent);

            // Draw the bars.
            for entity in channel.entities.iter() {
                if entity.value.is_none() {
                    continue;
                }
                entity.draw(ctx, percent, false);
            }

            if let Some(crosshair) = crosshair {
                if focused_entity_index >= 0 {
                    ctx.set_fill_color(crosshair.color);
                    ctx.fill_rect(
                        props.y_axis_left + props.xlabel_hop * focused_entity_index as f64,
                        props.x_axis_top - props.y_axis_length,
                        props.xlabel_hop,
                        props.y_axis_length,
                    );
                }

                // Draw the labels
                if percent == 1.0 {
                    info!("Draw the labels");
                    if let Some(labels) = labels {
                        ctx.set_fill_color(labels.color);
                        ctx.set_font(
                            labels.font_family.unwrap_or(DEFAULT_FONT_FAMILY),
                            labels.font_style.unwrap_or(TextStyle::Normal),
                            TextWeight::Normal,
                            labels.font_size.unwrap_or(12.),
                        );
                        ctx.set_text_align(TextAlign::Center);
                        ctx.set_text_baseline(BaseLine::Alphabetic);

                        for entity in channel.entities.iter() {
                            if entity.value.is_none() {
                                continue;
                            }
                            let x = entity.left + 0.5 * entity.width;
                            let y = props.x_axis_top - entity.height - 5.;
                            // TODO: bar.formatted_value
                            let formatted_value = format!("{}", entity.value.unwrap());
                            ctx.fill_text(formatted_value.as_str(), x, y);
                        }
                    }
                }
            }
        }

        return false;
    }

    fn update_channel(&self, _: usize) {
        let entity_count = self.base.data_table.frames.len();

        let lefts = self.get_channel_lefts();
        let props = self.props.borrow();

        let mut channels = self.base.channels.borrow_mut();
        let mut idx = 0;
        for channel in channels.iter_mut() {
            let mut left = *lefts.get(idx).unwrap();
            let mut bar_width = 0.0;

            if channel.state == Visibility::Showing || channel.state == Visibility::Shown {
                bar_width = bar_width;
            }

            let color = self.base.get_color(idx);
            let highlight_color = self.base.get_highlight_color(color);
            channel.color = color;
            channel.highlight = highlight_color;

            for jdx in 0..entity_count {
                let mut entity = channel.entities.get_mut(jdx).unwrap();
                entity.index = jdx;
                entity.color = color;
                entity.highlight_color = highlight_color;
                entity.left = left;
                entity.bottom = props.x_axis_top;
                entity.height = self.value_to_bar_height(entity.value);
                entity.width = bar_width;
                left += props.xlabel_hop;
            }
            idx += 1;
        }
    }

    fn create_entity(
        &self,
        channel_index: usize,
        entity_index: usize,
        value: Option<D>,
        color: Color,
        highlight_color: Color,
    ) -> BarEntity<D> {
        let left = self.get_bar_left(channel_index, entity_index);
        let old_left = left;
        let height = self.value_to_bar_height(value);

        // Animate width.
        let mut old_height = height;
        let mut old_width = 0.;

        let props = self.props.borrow();
        let channels = self.base.channels.borrow();
        if channels.len() == 0 {
            // Data table changed. Animate height.
            old_height = 0.;
            old_width = props.bar_width;
        }

        BarEntity {
            index: entity_index,
            old_value: None,
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

    fn create_channels(&self, start: usize, end: usize) {
        let mut start = start;
        let mut result = Vec::new();
        let count = self.base.data_table.frames.len();
        let meta = &self.base.data_table.meta;
        while start < end {
            let channel = meta.get(start).unwrap();
            let name = channel.name;
            let color = self.base.get_color(start);
            let highlight = self.base.get_highlight_color(color);

            let entities = self.create_entities(start, 0, count, color, highlight);
            result.push(ChartChannel::new(name, color, highlight, entities));
            start += 1;
        }

        let mut channels = self.base.channels.borrow_mut();
        *channels = result;
    }

    fn create_entities(
        &self,
        channel_index: usize,
        start: usize,
        end: usize,
        color: Color,
        highlight: Color,
    ) -> Vec<BarEntity<D>> {
        let mut start = start;
        let mut result = Vec::new();
        while start < end {
            let frame = self.base.data_table.frames.get(start).unwrap();
            let value = frame.data.get(channel_index as u64);
            let entity = match frame.data.get(channel_index as u64) {
                Some(value) => {
                    let value = value.clone();
                    self.create_entity(channel_index, start, Some(value), color, highlight)
                }
                None => self.create_entity(channel_index, start, None, color, highlight),
            };

            //   e.chart = this;
            result.push(entity);
            start += 1;
        }
        result
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
