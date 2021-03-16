#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use animate::easing::{get_easing, Easing};
use dataflow::*;
use primitives::{
    BaseLine, CanvasContext, Color, LineJoin, Point, Rect, Size, TextAlign, TextStyle, TextWeight,
};
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::*;

#[derive(Default, Clone)]
pub struct LinePoint<D> {
    color: Color,
    highlight_color: Color,
    // formatted_value: String,
    index: usize,
    old_value: Option<D>,
    value: Option<D>,

    old_x: f64,
    old_y: f64,
    old_cp1: Option<Point<f64>>,
    old_cp2: Option<Point<f64>>,
    old_point_radius: f64,

    /// The first control point.
    cp1: Point<f64>,

    /// The second control point.
    cp2: Point<f64>,
    x: f64,
    y: f64,

    point_radius: f64,
}

impl<D> LinePoint<D> {
    fn as_point(&self) -> Point<f64> {
        Point::new(self.x, self.y)
    }
}

/// A point in a line chart.
impl<C, D> Drawable<C> for LinePoint<D>
where
    C: CanvasContext,
{
    fn draw(&self, ctx: &C, percent: f64, highlight: bool) {
        let cx = utils::lerp(self.old_x, self.x, percent);
        let cy = utils::lerp(self.old_y, self.y, percent);
        let pr = utils::lerp(self.old_point_radius, self.point_radius, percent);
        if highlight {
            ctx.set_fill_color(self.highlight_color);
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

impl<D> Entity for LinePoint<D> {
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
    xaxis_top: f64,
    yaxis_left: f64,
    xaxis_length: f64,
    yaxis_length: f64,
    xlabel_max_width: f64,
    ylabel_max_width: f64,
    xlabel_rotation: f64, // 0..90
    xlabel_step: i64,
    xlabel_hop: f64, // Distance between two consecutive x-axis labels.
    ylabel_hop: f64, // Distance between two consecutive x-axis labels.
    x_title_box: Rect<f64>,
    y_title_box: Rect<f64>,
    x_title_center: Option<Point<f64>>,
    y_title_center: Option<Point<f64>>,
    xlabels: Vec<String>,
    ylabels: Vec<String>,
    yinterval: f64,
    ymax_value: f64,
    ymin_value: f64,
    yrange: f64,

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
    D: fmt::Display + Copy,
{
    props: RefCell<LineChartProperties>,
    base: BaseChart<'a, C, LinePoint<D>, M, D, LineChartOptions<'a>>,
}

impl<'a, C, M, D> LineChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display + Copy + Into<f64> + Ord + Default,
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
        props.yaxis_left + props.xlabel_hop * ((index as f64) + props.xlabel_offset_factor)
    }

    /// Returns the y-coordinate corresponding to the data point [value] and
    /// the animation percent [percent].
    fn value_to_y(&self, value: Option<D>) -> f64 {
        let props = self.props.borrow();
        match value {
            Some(value) => {
                props.xaxis_top
                    - (value.into() - props.ymin_value) / props.yrange * props.yaxis_length
            }
            None => props.xaxis_top,
        }
    }

    fn data_cell_changed(&self, record: DataCellChangeRecord<D>) {
        if record.column_index == 0 {
            //   xlabels[record.rowIndex] = record.newValue;
        } else {
            self.base.data_cell_changed(record);
        }
    }

    fn get_entity_group_index(&self, x: f64, y: f64) -> i64 {
        let props = self.props.borrow();

        let dx = x - props.yaxis_left;
        // If (x, y) is inside the rectangle defined by the two axes.
        if y > props.xaxis_top - props.yaxis_length
            && y < props.xaxis_top
            && dx > 0.
            && dx < props.xaxis_length
        {
            let index = (dx / props.xlabel_hop - props.xlabel_offset_factor).round() as usize;
            // If there is at least one visible point in the current point group...
            if props.average_y_values.get(index).is_some() {
                return index as i64;
            }
        }
        -1
    }

    /// Calculates average y values for the visible channel to help position the
    /// tooltip.
    ///
    /// If [index] is given, calculates the average y value for the entity group
    /// at [index] only.
    ///
    fn calculate_average_y_values(&self, index: usize) {
        // if !self.base.options.tooltip.enabled {
        //     return;
        // }

        // let entity_count = self.base.data.frames.len();
        // let start = if index != 0 { index } else { 0 };

        // let end = if index == 0 { entity_count } else { index + 1 };

        // let mut props = self.props.borrow_mut();
        // let channels = self.base.channels.borrow();

        // let channel_states = &self.base.props.borrow().channel_states;

        // props
        //     .average_y_values
        //     .resize(entity_count, Default::default());

        // for idx in start..end {
        //     let mut sum = 0.0;
        //     let mut count = 0;
        //     // TODO: check it
        //     for jdx in channels.len()..0 {
        //         let channel_state = channel_states.get(jdx).unwrap();
        //         if *channel_state == Visibility::Hidden || *channel_state == Visibility::Hiding {
        //             continue;
        //         }

        //         let channel = channels.get(jdx).unwrap();
        //         let point = channel.entities.get(idx).unwrap();
        //         if point.value != 0. {
        //             sum += point.y;
        //             count += 1;
        //         }
        //     }
        //     props.average_y_values[idx] = if count > 0 { sum / count as f64 } else { 0. }
        // }
    }

    fn lerp_points(&self, points: &Vec<LinePoint<D>>, percent: f64) -> Vec<LinePoint<D>> {
        points
            .iter()
            .map(|p| {
                let x = utils::lerp(p.old_x, p.x, percent);
                let y = utils::lerp(p.old_y, p.y, percent);

                let cp1 = if p.cp1 != Default::default() {
                    // FIXME:
                    // utils::lerp(p.old_cp1, p.cp1, percent)
                    Default::default()
                } else {
                    Default::default()
                };

                let cp2 = if p.cp2 != Default::default() {
                    // FIXME:
                    // utils::lerp(p.old_cp2, p.cp2, percent)
                    Default::default()
                } else {
                    Default::default()
                };

                LinePoint {
                    index: p.index,
                    old_value: None,
                    value: p.value,
                    color: p.color,
                    highlight_color: p.highlight_color,
                    old_point_radius: p.old_point_radius,
                    old_cp1: Default::default(),
                    old_cp2: Default::default(),
                    old_x: p.old_x,
                    old_y: p.old_y,
                    point_radius: p.point_radius,
                    x,
                    y,
                    cp1,
                    cp2,
                }
            })
            .collect()
    }

    fn channel_visibility_changed(&self, index: usize) {
        self.update_channel(index);
        self.calculate_average_y_values(0);
    }

    fn curve_to(
        &self,
        ctx: &C,
        cp1: Option<Point<f64>>,
        cp2: Option<Point<f64>>,
        p: &LinePoint<D>,
    ) {
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

    /// Called when [data_table] has been changed.
    fn data_changed(&self) {
        info!("data_changed");
        // self.calculate_drawing_sizes(ctx);
        self.create_channels(0, self.base.data.meta.len());
    }
}

impl<'a, C, M, D> Chart<'a, C, M, D, LinePoint<D>> for LineChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display + Copy + Into<f64> + Ord + Default,
{
    // let num xlabel_offset_factor = 0;

    // TODO: Separate y-axis stuff into a separate method.
    fn calculate_drawing_sizes(&self, ctx: &C) {
        info!("calculate_drawing_sizes");
        self.base.calculate_drawing_sizes(ctx);
        let mut props = self.props.borrow_mut();
        let mut baseprops = self.base.props.borrow_mut();
        let options = &self.base.options;

        props.tooltip_offset = options.channel.markers.size * 2. + 5.;

        // y-axis min-max
        props.ymax_value = if let Some(value) = options.yaxis.max_value {
            value as f64
        } else {
            f64::NEG_INFINITY
        };

        props.ymax_value = props
            .ymax_value
            .max(utils::find_max_value(&self.base.data).into());

        if props.ymax_value == f64::NEG_INFINITY {
            props.ymax_value = 0.0;
        }

        props.ymin_value = if let Some(value) = self.base.options.yaxis.min_value {
            value as f64
        } else {
            f64::INFINITY
        };

        props.ymin_value = props
            .ymin_value
            .min(utils::find_min_value(&self.base.data).into());

        if props.ymin_value == f64::INFINITY {
            props.ymin_value = 0.0;
        }

        if let Some(value) = options.yaxis.interval {
            props.yinterval = value
        } else {
            let min_interval = self.base.options.yaxis.min_interval;
            if props.ymin_value == props.ymax_value {
                if props.ymin_value == 0. {
                    props.ymax_value = 1.;
                    props.yinterval = 1.;
                } else if props.ymin_value == 1. {
                    props.ymin_value = 0.;
                    props.yinterval = 1.;
                } else {
                    props.yinterval = props.ymin_value * 0.25;
                    props.ymin_value -= props.yinterval;
                    props.ymax_value += props.yinterval;
                }

                if let Some(value) = min_interval {
                    props.yinterval = props.yinterval.max(value as f64);
                }
            } else {
                props.yinterval =
                    utils::calculate_interval(props.ymax_value - props.ymin_value, 5, min_interval);
            }
        }

        let val = props.ymin_value / props.yinterval;

        props.ymin_value = (props.ymin_value / props.yinterval).floor() * props.yinterval;
        props.ymax_value = (props.ymax_value / props.yinterval).ceil() * props.yinterval;
        props.yrange = props.ymax_value - props.ymin_value;

        // y-axis labels
        props.ylabel_formatter = self.base.options.yaxis.labels.formatter;
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

        let mut value = props.ymin_value;
        while value <= props.ymax_value {
            let ylabel_formatter = props.ylabel_formatter.unwrap();
            props.ylabels.push(ylabel_formatter(value));
            value += props.yinterval;
        }

        let options = &self.base.options;
        props.ylabel_max_width =
            utils::calculate_max_text_width(ctx, &options.yaxis.labels.style, &props.ylabels);

        baseprops.entity_value_formatter = props.ylabel_formatter;

        // Tooltip
        baseprops.tooltip_value_formatter = if let Some(formater) = options.tooltip.value_formatter
        {
            Some(formater)
        } else {
            props.ylabel_formatter
        };

        let area = &baseprops.area;

        // x-axis title
        #[allow(unused_assignments)]
        let mut xtitle_left = 0.;
        let mut xtitle_top = 0.;
        let mut xtitle_width = 0.;
        let mut xtitle_height = 0.;
        let xtitle = &self.base.options.xaxis.title;

        if let Some(text) = xtitle.text {
            let style = &xtitle.style;
            ctx.set_font(
                style.fontfamily.unwrap_or(DEFAULT_FONT_FAMILY),
                style.fontstyle.unwrap_or(TextStyle::Normal),
                TextWeight::Normal,
                style.fontsize.unwrap_or(12.),
            );
            xtitle_width = ctx.measure_text(text).width.round() + 2. * TITLE_PADDING;
            xtitle_height = xtitle.style.fontsize.unwrap_or(12.) + 2. * TITLE_PADDING;
            xtitle_top = area.origin.y + area.size.height - xtitle_height;
        }

        // y-axis title.
        let mut ytitle_left = 0.;
        let ytitle_top = 0.;
        let mut ytitle_width = 0.;
        let mut ytitle_height = 0.;
        let ytitle = &self.base.options.yaxis.title;

        if let Some(text) = ytitle.text {
            let style = &ytitle.style;
            ctx.set_font(
                style.fontfamily.unwrap_or(DEFAULT_FONT_FAMILY),
                style.fontstyle.unwrap_or(TextStyle::Normal),
                TextWeight::Normal,
                style.fontsize.unwrap_or(12.),
            );
            ytitle_height = ctx.measure_text(text).width.round() + 2. * TITLE_PADDING;
            ytitle_width = ytitle.style.fontsize.unwrap_or(12.) + 2. * TITLE_PADDING;
            ytitle_left = area.origin.x;
        }

        // Axes" size and position.
        props.yaxis_left = area.origin.x + props.ylabel_max_width + AXIS_LABEL_MARGIN as f64;
        if ytitle_width > 0. {
            props.yaxis_left += ytitle_width + CHART_TITLE_MARGIN;
        } else {
            props.yaxis_left += AXIS_LABEL_MARGIN as f64;
        }

        props.xaxis_length = (area.origin.x + area.size.width) - props.yaxis_left;

        props.xaxis_top = area.origin.y + area.size.height;
        if xtitle_height > 0. {
            props.xaxis_top -= xtitle_height + CHART_TITLE_MARGIN;
        } else {
            props.xaxis_top -= AXIS_LABEL_MARGIN as f64;
        }
        props.xaxis_top -= AXIS_LABEL_MARGIN as f64;

        // x-axis labels and x-axis"s position.
        let row_count = self.base.data.meta.len();
        props.xlabels = Vec::new();
        for idx in 0..row_count {
            let row = self.base.data.meta.get(idx).unwrap();
            props.xlabels.push(row.name.to_string());
        }

        props.xlabel_max_width =
            utils::calculate_max_text_width(ctx, &options.xaxis.labels.style, &props.xlabels);

        if props.xlabel_offset_factor > 0. && row_count > 1 {
            props.xlabel_hop = props.xaxis_length / row_count as f64;
        } else if row_count > 1 {
            props.xlabel_hop = props.xaxis_length / (row_count - 1) as f64;
        } else {
            props.xlabel_hop = props.xaxis_length;
        }
        props.xlabel_rotation = 0.;

        let font_size = self.base.options.xaxis.labels.style.fontsize.unwrap();
        let max_rotation = self.base.options.xaxis.labels.max_rotation;
        let min_rotation = self.base.options.xaxis.labels.min_rotation;
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
                props.xaxis_top -=
                    props.xlabel_max_width * abs_angle_rad.sin() + font_size * abs_angle_rad.cos();
                // TODO: fixme
                // break outer;
            }
        }

        // Wrap up.
        props.yaxis_length = props.xaxis_top
            - area.origin.y
            - (self.base.options.yaxis.labels.style.fontsize.unwrap() / 2.).trunc();
        props.ylabel_hop = props.yaxis_length / props.ylabels.len() as f64;

        xtitle_left = props.yaxis_left + ((props.xaxis_length - xtitle_width) / 2.).trunc();

        let ytitle_top = area.origin.y + ((props.yaxis_length - ytitle_height) / 2.).trunc();

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
        self.base.data = stream;
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
        self.data_changed();
        self.base.position_legend();

        // This call is redundant for row and column changes but necessary for
        // cell changes.
        self.calculate_drawing_sizes(ctx);
        self.update_channel(0);

        self.calculate_average_y_values(0);

        // self.ctx.clearRect(0, 0, self.width, self.height);
        self.draw_axes_and_grid(ctx);
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
        // x-axis title.
        let props = self.props.borrow();
        if let Some(x_title_center) = props.x_title_center {
            let opt = &self.base.options.xaxis.title;
            if let Some(text) = opt.text {
                let style = &opt.style;
                ctx.save();
                ctx.set_fill_color(opt.style.color);
                ctx.set_font(
                    &style.fontfamily.unwrap_or(DEFAULT_FONT_FAMILY),
                    style.fontstyle.unwrap_or(TextStyle::Normal),
                    TextWeight::Normal,
                    style.fontsize.unwrap_or(12.),
                );
                ctx.set_text_align(TextAlign::Center);
                ctx.set_text_baseline(BaseLine::Middle);
                ctx.fill_text(opt.text.unwrap(), x_title_center.x, x_title_center.y);
                ctx.restore();
            }
        }

        // y-axis title.
        if let Some(y_title_center) = props.y_title_center {
            let opt = &self.base.options.yaxis.title;
            if let Some(text) = opt.text {
                let style = &opt.style;
                ctx.save();
                ctx.set_fill_color(style.color);
                ctx.set_font(
                    &style.fontfamily.unwrap_or(DEFAULT_FONT_FAMILY),
                    style.fontstyle.unwrap_or(TextStyle::Normal),
                    TextWeight::Normal,
                    style.fontsize.unwrap_or(12.),
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
        let opt = &self.base.options.xaxis.labels;
        let style = &opt.style;
        ctx.set_fill_color(style.color);

        ctx.set_font(
            &style.fontfamily.unwrap_or(DEFAULT_FONT_FAMILY),
            style.fontstyle.unwrap_or(TextStyle::Normal),
            TextWeight::Normal,
            style.fontsize.unwrap_or(12.),
        );

        let mut x = self.xlabel_x(0);
        let mut y = props.xaxis_top + AXIS_LABEL_MARGIN as f64 + style.fontsize.unwrap_or(12.);
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
                x +=
                    props.xlabel_rotation.signum() * ((style.fontsize.unwrap_or(12.) / 8.).trunc());
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
        let opt = &self.base.options.yaxis.labels;
        let style = &opt.style;
        ctx.set_fill_color(opt.style.color);

        ctx.set_font(
            &style.fontfamily.unwrap_or(DEFAULT_FONT_FAMILY),
            style.fontstyle.unwrap_or(TextStyle::Normal),
            TextWeight::Normal,
            style.fontsize.unwrap_or(12.),
        );

        ctx.set_text_align(TextAlign::Right);
        ctx.set_text_baseline(BaseLine::Middle);
        x = props.yaxis_left - AXIS_LABEL_MARGIN as f64;
        y = props.xaxis_top - (style.fontsize.unwrap_or(12.) / 8.).trunc();
        for label in props.ylabels.iter() {
            ctx.fill_text(label.as_str(), x, y);
            y -= props.ylabel_hop;
        }

        // x grid lines - draw bottom up.
        let opt = &self.base.options.xaxis;
        if opt.grid_line_width > 0. {
            ctx.set_line_width(opt.grid_line_width);
            ctx.set_stroke_color(opt.grid_line_color);
            ctx.begin_path();
            y = props.xaxis_top - props.ylabel_hop;
            // TODO: should draw 2 and len - 1 lines
            for idx in 0..props.ylabels.len() {
                ctx.move_to(props.yaxis_left, y);
                ctx.line_to(props.yaxis_left + props.xaxis_length, y);
                y -= props.ylabel_hop;
            }
            ctx.stroke();
        }

        // y grid lines or x-axis ticks - draw from left to right.
        let opt = &self.base.options.yaxis;
        let mut line_width = opt.grid_line_width;
        x = props.yaxis_left;

        if props.xlabel_step > 1 {
            x = self.xlabel_x(0);
        }

        if line_width > 0. {
            y = props.xaxis_top - props.yaxis_length;
        } else {
            line_width = 1.;
            y = props.xaxis_top + AXIS_LABEL_MARGIN as f64;
        }

        ctx.set_line_width(line_width);
        ctx.set_stroke_color(opt.grid_line_color);
        ctx.begin_path();
        let mut idx = 0;
        while idx < props.xlabels.len() {
            ctx.move_to(x, y);
            ctx.line_to(x, props.xaxis_top);
            x += scaled_label_hop;
            idx += props.xlabel_step as usize;
        }
        ctx.stroke();

        // x-axis itself.
        let opt = &self.base.options.xaxis;
        if opt.line_width > 0. {
            ctx.set_line_width(opt.line_width);
            ctx.set_stroke_color(opt.line_color);
            ctx.begin_path();
            ctx.move_to(props.yaxis_left, props.xaxis_top);
            ctx.line_to(props.yaxis_left + props.xaxis_length, props.xaxis_top);
            ctx.stroke();
        }

        // y-axis itself.
        let opt = &self.base.options.yaxis;
        if opt.line_width > 0. {
            ctx.set_line_width(opt.line_width);
            ctx.set_stroke_color(opt.line_color);
            ctx.begin_path();
            ctx.move_to(props.yaxis_left, props.xaxis_top - props.yaxis_length);
            ctx.line_to(props.yaxis_left, props.xaxis_top);
            ctx.stroke();
        }
    }

    /// Draws the current animation frame.
    ///
    /// If [time] is `null`, draws the last frame (i.e. no animation).
    fn draw_frame(&self, ctx: &C, time: Option<i64>) {
        self.base.draw_frame(ctx, time);

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
        let channel_count = channels.len();

        let options = &self.base.options;

        let entity_count = self.base.data.frames.len();
        let fill_opacity = options.channel.fill_opacity;
        let channel_line_width = options.channel.line_width;
        let marker_options = &options.channel.markers;
        let marker_size = marker_options.size;

        let focused_channel_index = self.base.props.borrow().focused_channel_index;
        let focused_entity_index = self.base.props.borrow().focused_entity_index as usize;
        let props = self.props.borrow();
        let label_options = &options.channel.labels;

        let mut idx = 0;
        for channel in channels.iter() {
            if channel.state == Visibility::Hidden {
                continue;
            }

            let entities = self.lerp_points(&channel.entities, percent);
            let scale = if idx as i64 != focused_channel_index {
                1.
            } else {
                2.
            };

            ctx.set_line_join(LineJoin::Round);

            // Draw channel with filling.
            if fill_opacity > 0.0 {
                let color = self.base.change_color_alpha(channel.color, fill_opacity);
                ctx.set_fill_color(color);
                ctx.set_stroke_color(color);
                let mut jdx = 0;
                loop {
                    // Skip points with a null value.
                    while jdx < entity_count && entities[jdx].value.is_none() {
                        jdx += 1;
                    }

                    // Stop if we have reached the end of the channel.
                    if jdx == entity_count {
                        break;
                    }

                    // Connect a channel of contiguous points with a non-null value and
                    // fill the area between them and the x-axis.
                    let mut entity = entities.get(jdx).unwrap();
                    ctx.begin_path();
                    ctx.move_to(entity.x, props.xaxis_top);
                    ctx.line_to(entity.x, entity.y);
                    let mut last_point = entity;
                    let mut count = 1;
                    while jdx < entity_count && entities[jdx].value.is_some() {
                        entity = entities.get(jdx).unwrap();
                        self.curve_to(ctx, Some(last_point.cp2), Some(entity.cp1), entity);
                        last_point = entity;
                        count += 1;
                        jdx += 1;
                    }
                    if count >= 2 {
                        ctx.line_to(last_point.x, props.xaxis_top);
                        ctx.close_path();
                        ctx.fill();
                    }
                }
            }

            // Draw channel without filling.
            if channel_line_width > 0. {
                let mut last_point: LinePoint<D> = Default::default();
                ctx.set_line_width(scale * channel_line_width);
                ctx.set_stroke_color(channel.color);
                ctx.begin_path();
                for entity in entities.iter() {
                    if entity.value.is_some() {
                        if last_point.value.is_some() {
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
                    channel.color
                };

                let stroke_color = if let Some(color) = marker_options.stroke_color {
                    color
                } else {
                    channel.color
                };
                ctx.set_fill_color(fill_color);
                ctx.set_line_width(scale * marker_options.line_width as f64);
                ctx.set_stroke_color(stroke_color);
                for entity in entities.iter() {
                    if entity.value.is_some() {
                        if marker_options.enabled {
                            entity.draw(ctx, 1.0, entity.index == focused_entity_index);
                        } else if entity.index == focused_entity_index {
                            // Only draw marker on hover.
                            entity.draw(ctx, 1.0, true);
                        }
                    }
                }
            }
            idx += 1;
        }

        // Draw labels only on the last frame.
        if let Some(label_options) = label_options {
            if percent == 1.0 {
                ctx.set_fill_color(label_options.color);
                ctx.set_font(
                    label_options.fontfamily.unwrap_or(DEFAULT_FONT_FAMILY),
                    label_options.fontstyle.unwrap_or(TextStyle::Normal),
                    TextWeight::Normal,
                    label_options.fontsize.unwrap_or(12.),
                );
                ctx.set_text_align(TextAlign::Center);
                ctx.set_text_baseline(BaseLine::Alphabetic);
                for channel in channels.iter() {
                    if channel.state != Visibility::Shown {
                        continue;
                    }

                    let entities = &channel.entities;
                    for entity in entities.iter() {
                        match entity.value {
                            Some(value) => {
                                let y = entity.y - marker_size - 5.;
                                // TODO: bar.formatted_value
                                let value: f64 = value.into();
                                let formatted_value = format!("{}", value);
                                ctx.fill_text(formatted_value.as_str(), entity.x, y);
                            }
                            None => {}
                        }
                    }
                }
            }
        }

        false
    }

    fn update_channel(&self, index: usize) {
        info!("update_channel");
        let entity_count = self.base.data.frames.len();

        let marker_size = self.base.options.channel.markers.size;
        let curve_tension = self.base.options.channel.curve_tension;
        let curve = curve_tension > 0. && entity_count > 2;

        let start = if index != 0 { index } else { 0 };

        let mut channels = self.base.channels.borrow_mut();
        let end = if index == 0 {
            channels.len()
        } else {
            index + 1
        };

        let props = self.props.borrow();

        let mut idx = 0;
        for channel in channels.iter_mut() {
            let visible =
                channel.state == Visibility::Showing || channel.state == Visibility::Shown;

            let color = self.base.get_color(idx);
            let highlight_color = self.base.get_highlight_color(color);
            channel.color = color;
            channel.highlight = highlight_color;

            for jdx in 0..entity_count {
                let entity = channel.entities.get_mut(jdx).unwrap();
                entity.index = jdx;
                entity.color = color;
                entity.highlight_color = highlight_color;
                entity.x = self.xlabel_x(jdx);
                entity.y = if visible {
                    self.value_to_y(entity.value)
                } else {
                    props.xaxis_top
                };
                entity.point_radius = if visible { marker_size } else { 0. };
            }

            if !curve {
                continue;
            }

            // TODO: complete it
            // let mut e1;
            // let mut e2 = channel.entities.get_mut(0).unwrap();
            // let mut e3 = channel.entities.get_mut(1).unwrap();
            // for jdx in 2..entity_count {
            //     e1 = e2;
            //     e2 = e3;
            //     e3 = channel.entities.get_mut(jdx).unwrap();

            //     if e1.value.is_none() || e2.value.is_none() || e3.value.is_none() {
            //         continue;
            //     }

            //     let (cp1, cp2) = utils::calculate_control_points(
            //         e1.as_point(),
            //         e2.as_point(),
            //         e3.as_point(),
            //         curve_tension,
            //     );

            //     e2.cp1 = cp1;
            //     e2.cp2 = cp2;

            //     if e2.old_cp1.is_none() {
            //         e2.old_cp1 = Some(Point::new(e2.cp1.x, props.xaxis_top));
            //     }

            //     if e2.old_cp2.is_none() {
            //         e2.old_cp2 = Some(Point::new(e2.cp2.x, props.xaxis_top));
            //     }
            // }
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
    ) -> LinePoint<D> {
        let x = self.xlabel_x(entity_index);

        let props = self.props.borrow();
        let old_y = props.xaxis_top;
        // oldCp1 and oldCp2 are calculated in [update_channel].

        // let formatted_value = if value != 0 {
        //     entity_value_formatter(value)
        // } else {
        //     null
        // };

        LinePoint {
            index: entity_index,
            old_value: None,
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
            point_radius: self.base.options.channel.markers.size,
        }
    }

    fn create_channels(&self, start: usize, end: usize) {
        let mut start = start;
        let mut result = Vec::new();
        let count = self.base.data.frames.len();
        let meta = &self.base.data.meta;
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
    ) -> Vec<LinePoint<D>> {
        let mut start = start;
        let mut result = Vec::new();
        while start < end {
            let frame = self.base.data.frames.get(start).unwrap();
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
            props.xaxis_top - props.yaxis_length,
            props.average_y_values[focused_entity_index] - (tooltip_height / 2.).trunc(),
        );

        let width = self.base.props.borrow().width;
        if x + tooltip_width > width {
            x -= tooltip_width + 2. * props.tooltip_offset;
            x = x.max(props.yaxis_left);
        }

        Point::new(x, y)
    }
}
