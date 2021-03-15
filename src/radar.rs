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
pub struct PolarPoint<D> {
    color: Color,
    highlight_color: Color,
    // formatted_value: String,
    index: usize,
    old_value: Option<D>,
    value: Option<D>,

    old_radius: f64,
    old_angle: f64,
    old_point_radius: f64,

    radius: f64,
    angle: f64,
    point_radius: f64,

    center: Point<f64>,
}

impl<C, D> Drawable<C> for PolarPoint<D>
where
    C: CanvasContext,
{
    fn draw(&self, ctx: &C, percent: f64, highlight: bool) {
        let r = utils::lerp(self.old_radius, self.radius, percent);
        let a = utils::lerp(self.old_angle, self.angle, percent);
        let pr = utils::lerp(self.old_point_radius, self.point_radius, percent);
        let p = utils::polar2cartesian(&self.center, r, a);
        if highlight {
            ctx.set_fill_color(self.highlight_color);
            ctx.begin_path();
            ctx.arc(p.x, p.y, 2. * pr, 0., TAU, false);
            ctx.fill();
        }
        ctx.set_fill_color(self.color);
        ctx.begin_path();
        ctx.arc(p.x, p.y, pr, 0., TAU, false);
        ctx.fill();
        ctx.stroke();
    }
}

impl<D> Entity for PolarPoint<D> {
    fn free(&mut self) {}

    fn save(&self) {
        // self.old_radius = self.radius;
        // self.old_angle = self.angle;
        // self.old_point_radius = self.point_radius;
        // self.old_value = self.value;
    }
}

#[derive(Default, Clone)]
struct RadarChartProperties {
    center: Point<f64>,
    radius: f64,
    angle_interval: f64,
    xlabels: Vec<String>,
    ylabels: Vec<String>,
    y_max_value: f64,
    ylabel_hop: f64,
    ylabel_formatter: Option<ValueFormatter>,
    /// Each element is the bounding box of each entity group.
    /// A `null` element means the group has no visible entities.
    bounding_boxes: Vec<Rect<f64>>,
}

pub struct RadarChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display + Copy,
{
    props: RefCell<RadarChartProperties>,
    base: BaseChart<'a, C, PolarPoint<D>, M, D, RadarChartOptions<'a>>,
}

impl<'a, C, M, D> RadarChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display + Copy + Into<f64> + Ord + Default,
{
    pub fn new(options: RadarChartOptions<'a>) -> Self {
        Self {
            props: Default::default(),
            base: BaseChart::new(options),
        }
    }

    pub fn get_angle(&self, entity_index: usize) -> f64 {
        let props = self.props.borrow();
        (entity_index as f64) * props.angle_interval - PI_2
    }

    pub fn value2radius(&self, value: Option<D>) -> f64 {
        match value {
            Some(value) => {
                let props = self.props.borrow();
                value.into() * props.radius / props.y_max_value
            }
            None => 0.0
        }
    }

    fn calculate_bounding_boxes(&self) {
        if !self.base.options.tooltip.enabled {
            return;
        }

        let channels = self.base.channels.borrow();
        let channel_count = channels.len();

        let entity_count = {
            let channel = channels.get(0).unwrap();
            channel.entities.len()
        };

        let mut props = self.props.borrow_mut();

        // OOCH
        props
            .bounding_boxes
            .resize(entity_count, Default::default());

        for idx in 0..entity_count {
            let mut min_x = f64::MAX;
            let mut min_y = f64::MAX;
            let mut max_x = -f64::MAX;
            let mut max_y = -f64::MAX;
            let mut count = 0;
            for jdx in 0..channel_count {
                let channel = channels.get(jdx).unwrap();
                if channel.state == Visibility::Hidden || channel.state == Visibility::Hiding {
                    continue;
                }

                let channel = channels.get(jdx).unwrap();
                let pp = channel.entities.get(idx).unwrap();

                if pp.value.is_none() {
                    continue;
                }

                let cp = utils::polar2cartesian(&pp.center, pp.radius, pp.angle);
                min_x = min_x.min(cp.x);
                min_y = min_y.min(cp.y);
                max_x = max_x.max(cp.x);
                max_y = max_y.max(cp.y);
                count += 1;
            }

            props.bounding_boxes[idx] = if count > 0 {
                Rect::new(
                    Point::new(min_x, min_y),
                    Size::new(max_x - min_x, max_y - min_y),
                )
            } else {
                unimplemented!()
            };
        }
    }

    fn draw_text(&self, ctx: &C, text: &str, radius: f64, angle: f64, font_size: f64) {
        let props = self.props.borrow();
        let w = ctx.measure_text(text).width;
        let x = props.center.x + angle.cos() * (props.radius + 0.5 * w);
        let y = props.center.y + angle.sin() * (props.radius + 0.5 * font_size);
        ctx.fill_text(text, x, y);
    }

    fn get_entity_group_index(&self, x: f64, y: f64) -> i64 {
        let props = self.props.borrow();
        let p = Point::new(x - props.center.x, y - props.center.y);

        if p.distance_to(Point::zero()) >= props.radius {
            return -1;
        }

        let angle = p.y.atan2(p.x);
        let channels = self.base.channels.borrow();
        let channel = channels.first().unwrap();
        let points = &channel.entities;

        for idx in points.len()..0 {
            if props.bounding_boxes.get(idx).is_none() {
                continue;
            }

            let delta = angle - points[idx].angle;
            if delta.abs() < 0.5 * props.angle_interval {
                return idx as i64;
            }
            if (delta + TAU).abs() < 0.5 * props.angle_interval {
                return idx as i64;
            }
        }
        return -1;
    }

    fn channel_visibility_changed(&self, index: usize) {
        let mut channels = self.base.channels.borrow_mut();
        let channel = channels.get_mut(index).unwrap();

        let visible = channel.state == Visibility::Showing || channel.state == Visibility::Shown;
        let marker_size = self.base.options.channel.markers.size;

        for entity in channel.entities.iter_mut() {
            if visible {
                entity.radius = self.value2radius(entity.value);
                entity.point_radius = marker_size;
            } else {
                entity.radius = 0.0;
                entity.point_radius = 0.;
            }
        }

        self.calculate_bounding_boxes();
    }

    /// Called when [data_table] has been changed.
    fn data_table_changed(&self) {
        info!("data_table_changed");
        // self.calculate_drawing_sizes(ctx);
        self.create_channels(0, self.base.data_table.meta.len());
    }
}

impl<'a, C, M, D> Chart<'a, C, M, D, PolarPoint<D>> for RadarChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display + Copy + Into<f64> + Ord + Default,
{
    fn calculate_drawing_sizes(&self, ctx: &C) {
        info!("calculate_drawing_sizes");
        self.base.calculate_drawing_sizes(ctx);

        let mut props = self.props.borrow_mut();

        props.xlabels = self
            .base
            .data_table
            .meta
            .iter()
            .map(|item| item.name.to_string())
            .collect();

        props.angle_interval = TAU / props.xlabels.len() as f64;

        let xlabel_font_size = self.base.options.x_axis.labels.style.font_size.unwrap();

        // [_radius]*factor equals the height of the largest polygon.
        let factor = 1. + ((props.xlabels.len() >> 1) as f64 * props.angle_interval - PI_2).sin();

        {
            let rect = &self.base.props.borrow().channel_and_axes_box;
            props.radius = rect.size.width.min(rect.size.height) / factor
                - factor * (xlabel_font_size + AXIS_LABEL_MARGIN as f64);
            props.center = Point::new(
                rect.origin.x + rect.size.width / 2.,
                rect.origin.y + rect.size.height / factor,
            );
        }

        // The minimum value on the y-axis is always zero
        let y_axis = &self.base.options.y_axis;
        let yinterval = match y_axis.interval {
            Some(yinterval) => yinterval,
            None => {
                let ymin_interval = y_axis.min_interval.unwrap_or(0.0);

                props.y_max_value = utils::find_max_value(&self.base.data_table).into();

                let yinterval =
                    utils::calculate_interval(props.y_max_value, 3, Some(ymin_interval));
                props.y_max_value = (props.y_max_value / yinterval).ceil() * yinterval;
                yinterval
            }
        };

        props.ylabel_formatter = y_axis.labels.formatter;
        if props.ylabel_formatter.is_none() {
            // let max_decimal_places =
            //     max(utils::get_decimal_places(props.yinterval), utils::get_decimal_places(props.y_min_value));
            // let numberFormat = NumberFormat.decimalPattern()
            // ..maximumFractionDigits = max_decimal_places
            // ..minimumFractionDigits = max_decimal_places;
            // ylabel_formatter = numberFormat.format;
            let a = |x: f64| -> String { x.to_string() };
            props.ylabel_formatter = Some(a);
        }

        let mut baseprops = self.base.props.borrow_mut();
        baseprops.entity_value_formatter = props.ylabel_formatter;

        props.ylabels.clear();
        let ylabel_formatter = props.ylabel_formatter.unwrap();

        let mut value = 0.0;
        while value <= props.y_max_value {
            props.ylabels.push(ylabel_formatter(value));
            value += yinterval;
        }

        props.ylabel_hop = props.radius / (props.ylabels.len() as f64 - 1.);

        // Tooltip.
        baseprops.tooltip_value_formatter =
            if let Some(value_formatter) = self.base.options.tooltip.value_formatter {
                Some(value_formatter)
            } else {
                Some(ylabel_formatter)
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
        // self.easing_function = get_easing(self.options.animation().easing);
        self.base.initialize_legend();
        self.base.initialize_tooltip();

        self.base.draw(ctx);

        self.base.stop_animation();
        self.data_table_changed();
        self.base.position_legend();

        // This call is redundant for row and column changes but necessary for
        // cell changes.
        self.calculate_drawing_sizes(ctx);
        info!("after calculate_drawing_sizes");

        self.update_channel(0);
        info!("after update_channel");

        self.calculate_bounding_boxes();
        info!("after calculate_bounding_boxes");
        self.draw_axes_and_grid(ctx);
        info!("after draw_axes_and_grid");
        self.base.start_animation();
        info!("after start_animation");
        self.draw_frame(ctx, None);
        info!("after draw_frame");
    }

    fn resize(&self, w: f64, h: f64) {
        self.base.resize(w, h);
    }

    fn draw_axes_and_grid(&self, ctx: &C) {
        let props = self.props.borrow();
        let xlabel_count = props.xlabels.len();
        let ylabel_count = props.ylabels.len();

        // x-axis grid lines (i.e. concentric equilateral polygons).
        let mut line_width = self.base.options.x_axis.grid_line_width;
        if line_width > 0. {
            ctx.set_line_width(line_width);
            ctx.set_stroke_color(self.base.options.x_axis.grid_line_color);
            ctx.begin_path();
            let mut radius = props.radius;
            for idx in ylabel_count - 1..1 {
                let mut angle = -PI_2 + props.angle_interval;
                ctx.move_to(props.center.x, props.center.y - radius);
                for jdx in 0..xlabel_count {
                    let point = utils::polar2cartesian(&props.center, radius, angle);
                    ctx.line_to(point.x, point.y);
                    angle += props.angle_interval;
                }
                radius -= props.ylabel_hop;
            }
            ctx.stroke();
        }

        // y-axis grid lines (i.e. radii from the center to the x-axis labels).
        line_width = self.base.options.y_axis.grid_line_width;
        if line_width > 0. {
            ctx.set_line_width(line_width);
            ctx.set_stroke_color(self.base.options.y_axis.grid_line_color);
            ctx.begin_path();
            let mut angle = -PI_2;
            for idx in 0..xlabel_count {
                let point = utils::polar2cartesian(&props.center, props.radius, angle);
                ctx.move_to(props.center.x, props.center.y);
                ctx.line_to(point.x, point.y);
                angle += props.angle_interval;
            }
            ctx.stroke();
        }

        // y-axis labels - don"t draw the first (at center) and the last ones.
        let style = &self.base.options.y_axis.labels.style;
        let x = props.center.x - AXIS_LABEL_MARGIN as f64;
        let mut y = props.center.y - props.ylabel_hop;
        ctx.set_fill_color(style.color);

        ctx.set_font(
            &style.font_family.unwrap_or(DEFAULT_FONT_FAMILY),
            style.font_style.unwrap_or(TextStyle::Normal),
            TextWeight::Normal,
            style.font_size.unwrap_or(12.),
        );

        ctx.set_text_align(TextAlign::Right);
        ctx.set_text_baseline(BaseLine::Middle);
        for idx in 1..ylabel_count - 2 {
            ctx.fill_text(props.ylabels[idx].as_str(), x, y);
            y -= props.ylabel_hop;
        }

        // x-axis labels.
        let style = &self.base.options.x_axis.labels.style;
        ctx.set_fill_color(style.color);

        ctx.set_font(
            &style.font_family.unwrap_or(DEFAULT_FONT_FAMILY),
            style.font_style.unwrap_or(TextStyle::Normal),
            TextWeight::Normal,
            style.font_size.unwrap_or(12.),
        );

        ctx.set_text_align(TextAlign::Center);
        ctx.set_text_baseline(BaseLine::Middle);
        let font_size = style.font_size.unwrap();
        let mut angle = -PI_2;
        let radius = props.radius + AXIS_LABEL_MARGIN as f64;
        for idx in 0..xlabel_count {
            self.draw_text(ctx, props.xlabels[idx].as_str(), radius, angle, font_size);
            angle += props.angle_interval;
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

        let ease = match props.easing_function {
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
        let props = self.props.borrow();

        let focused_channel_index = self.base.props.borrow().focused_channel_index;

        let fill_opacity = self.base.options.channel.fill_opacity;
        let channel_line_width = self.base.options.channel.line_width;
        let marker_options = &self.base.options.channel.markers;
        let marker_size = marker_options.size;
        let point_count = props.xlabels.len();

        let channels = self.base.channels.borrow();
        let focused_entity_index = self.base.props.borrow().focused_entity_index;

        let mut idx = 0;
        for channel in channels.iter() {
            let scale = if idx as i64 != focused_channel_index {
                1.
            } else {
                2.
            };

            idx += 1;
            if channel.state == Visibility::Hidden {
                continue;
            }

            // Draw the polygon.
            ctx.set_line_width(scale * channel_line_width);
            ctx.set_stroke_color(channel.color);
            ctx.begin_path();

            for jdx in 0..point_count {
                let entity = channel.entities.get(jdx).unwrap();
                // TODO: Optimize.
                let radius = utils::lerp(entity.old_radius, entity.radius, percent);
                let angle = utils::lerp(entity.old_angle, entity.angle, percent);
                let p = utils::polar2cartesian(&props.center, radius, angle);
                if jdx > 0 {
                    ctx.line_to(p.x, p.y);
                } else {
                    ctx.move_to(p.x, p.y);
                }
            }
            ctx.close_path();
            ctx.stroke();

            // Optionally fill the polygon.
            if fill_opacity > 0. {
                ctx.set_fill_color(self.base.change_color_alpha(channel.color, fill_opacity));
                ctx.fill();
            }

            // Draw the markers.
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
                ctx.set_line_width(scale * marker_options.line_width);
                ctx.set_stroke_color(stroke_color);
                for p in channel.entities.iter() {
                    if marker_options.enabled {
                        p.draw(ctx, percent, p.index as i64 == focused_entity_index);
                    } else if p.index as i64 == focused_entity_index {
                        // Only draw marker on hover.
                        p.draw(ctx, percent, true);
                    }
                }
            }
        }

        return false;
    }

    // param should be Option
    fn update_channel(&self, _: usize) {
        let entity_count = self.base.data_table.frames.len();
        let mut channels = self.base.channels.borrow_mut();
        let props = self.props.borrow();

        let mut idx = 0;
        for channel in channels.iter_mut() {
            let color = self.base.get_color(idx);
            let highlight_color = self.base.get_highlight_color(color);
            channel.color = color;
            channel.highlight = highlight_color;

            let visible =
                channel.state == Visibility::Showing || channel.state == Visibility::Shown;

            for jdx in 0..entity_count {
                let mut entity = channel.entities.get_mut(jdx).unwrap();
                entity.index = jdx;
                entity.center = props.center;
                entity.radius = if visible {
                    self.value2radius(entity.value)
                } else {
                    0.0
                };
                entity.angle = self.get_angle(jdx);
                entity.color = color;
                entity.highlight_color = highlight_color;
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
    ) -> PolarPoint<D> {
        let props = self.props.borrow();
        let angle = self.get_angle(entity_index);
        let point_radius = self.base.options.channel.markers.size as f64;

        PolarPoint {
            index: entity_index,
            value,
            old_value: None,
            color,
            highlight_color,
            center: props.center,
            old_radius: 0.,
            old_angle: angle,
            old_point_radius: 0.,
            radius: self.value2radius(value),
            angle,
            point_radius,
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
    ) -> Vec<PolarPoint<D>> {
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
        let focused_entity_index = self.base.props.borrow().focused_entity_index;

        let bounding_box = &props.bounding_boxes[focused_entity_index as usize];
        let offset = self.base.options.channel.markers.size as f64 * 2. + 5.;
        let origin = bounding_box.origin;
        let mut x = origin.x + bounding_box.width() + offset;
        let y = origin.y + ((bounding_box.height() - tooltip_height) / 2.).trunc();

        let width = self.base.props.borrow().width;
        if x + tooltip_width > width {
            x = origin.x - tooltip_width - offset;
        }

        Point::new(x, y)
    }
}
