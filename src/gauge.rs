#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use animate::{
    easing::{get_easing, Easing},
    interpolate::lerp,
};
use dataflow::*;
use primitives::{palette, CanvasContext, Color, Point, TextAlign, TextStyle, TextWeight};
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::*;

const START_ANGLE: f64 = -std::f64::consts::FRAC_PI_2;

#[derive(Default, Clone)]
pub struct GaugeEntity<D> {
    // Chart chart,
    color: Color,
    highlight_color: Color,
    // formatted_value: String,
    index: usize,
    old_value: Option<D>,
    value: Option<D>,

    old_start_angle: f64,
    old_end_angle: f64,
    start_angle: f64,
    end_angle: f64,

    center: Point<f64>,
    inner_radius: f64,
    outer_radius: f64,

    // [Series] field.
    name: String,

    background_color: Color,
}

impl<D> GaugeEntity<D> {
    pub fn is_empty(&self) -> bool {
        self.start_angle == self.end_angle
    }

    pub fn contains_point(&self, p: Point<f64>) -> bool {
        // let p = p - center;
        let mag = p.distance_to(Point::default()); //p.magnitude()
        if mag > self.outer_radius || mag < self.inner_radius {
            return false;
        }

        let angle = f64::atan2(p.y, p.x);
        // let chartStartAngle = (chart as dynamic).start_angle;

        // Make sure [angle] is in range [chartStartAngle]..[chartStartAngle] + TAU.
        // angle = (angle - chartStartAngle) % TAU + chartStartAngle;

        // If counterclockwise, make sure [angle] is in range
        // [start] - 2*pi..[start].
        // if start_angle > end_angle {
        //     angle -= angle - TAU;
        // }

        // if (start_angle <= end_angle) {
        //   // Clockwise.
        //   return is_in_range(angle, start_angle, end_angle);
        // } else {
        //   // Counterclockwise.
        //   return is_in_range(angle, end_angle, start_angle);
        // }
        unimplemented!()
    }

    fn draw_entity<C: CanvasContext>(&self, ctx: &C, percent: f64, highlight: bool) {
        // Draw the background.
        {
            let mut a1 = lerp(self.old_start_angle, self.start_angle, percent);
            let mut a2 = lerp(self.old_end_angle, self.start_angle + TAU, percent);
            if a1 > a2 {
                let tmp = a1;
                a1 = a2;
                a2 = tmp;
            }
            let center = &self.center;
            ctx.set_fill_color(self.background_color);
            ctx.begin_path();
            ctx.arc(center.x, center.y, self.outer_radius, a1, a2, false);
            ctx.arc(center.x, center.y, self.inner_radius, a2, a1, true);
            ctx.fill();
        }

        let mut a1 = lerp(self.old_start_angle, self.start_angle, percent);
        let mut a2 = lerp(self.old_end_angle, self.end_angle, percent);
        if a1 > a2 {
            let tmp = a1;
            a1 = a2;
            a2 = tmp;
        }
        let center = &self.center;

        if highlight {
            let highlight_outer_radius = HIGHLIGHT_OUTER_RADIUS_FACTOR * self.outer_radius;
            ctx.set_fill_color(self.highlight_color);
            ctx.begin_path();
            ctx.arc(center.x, center.y, highlight_outer_radius, a1, a2, false);
            ctx.arc(center.x, center.y, self.inner_radius, a2, a1, true);
            ctx.fill();
        }

        ctx.set_fill_color(self.color);
        ctx.begin_path();
        ctx.arc(center.x, center.y, self.outer_radius, a1, a2, false);
        ctx.arc(center.x, center.y, self.inner_radius, a2, a1, true);
        ctx.fill();
        ctx.stroke();
    }
}

impl<D> Entity for GaugeEntity<D> {
    fn free(&mut self) {
        // chart = null;
    }

    fn save(&self) {
        // self.old_start_angle = self.start_angle;
        // self.old_end_angle = self.end_angle;
        // self.old_value = self.value;
    }
}

impl<C, D> Drawable<C> for GaugeEntity<D>
where
    C: CanvasContext,
    D: fmt::Display + Copy + Into<f64> + Ord + Default,
{
    fn draw(&self, ctx: &C, percent: f64, highlight: bool) {
        let tmp_color = &self.color;
        let tmp_end_angle = self.end_angle;

        self.draw_entity(ctx, percent, highlight);

        // Draw the percent.
        let fs1 = 0.75 * self.inner_radius;
        let family = DEFAULT_FONT_FAMILY;

        let old_value = match self.old_value {
            Some(value) => value.into(),
            None => 0.0,
        };

        let value = match self.value {
            Some(value) => value.into(),
            None => 0.0,
        };

        let fs2 = 0.6 * fs1;
        let y = self.center.y + 0.3 * fs1;

        let text1 = lerp(old_value, value, percent).round().to_string();
        ctx.set_font(family, TextStyle::Normal, TextWeight::Normal, fs1);
        let w1 = ctx.measure_text(text1.as_str()).width;
        ctx.fill_text(text1.as_str(), self.center.x - 0.5 * w1, y);

        let text2 = "%";
        ctx.set_font(family, TextStyle::Normal, TextWeight::Normal, fs2);
        let w2 = ctx.measure_text(text2).width;
        ctx.fill_text(text2, self.center.x + 0.5 * w1 + 5., y);
    }
}

#[derive(Default, Clone)]
struct GaugeChartProperties {
    gauge_hop: f64,
    gauge_inner_radius: f64,
    gauge_outer_radius: f64,
    gauge_center_y: f64,
    // start_angle: f64,
}

pub struct GaugeChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display + Copy,
{
    props: RefCell<GaugeChartProperties>,
    base: BaseChart<'a, C, GaugeEntity<D>, M, D, GaugeChartOptions<'a>>,
}

impl<'a, C, M, D> GaugeChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display + Copy + Into<f64> + Ord + Default,
{
    pub fn new(options: GaugeChartOptions<'a>) -> Self {
        Self {
            props: Default::default(),
            base: BaseChart::new(options),
        }
    }

    fn get_gauge_center(&self, index: usize) -> Point<f64> {
        let props = self.props.borrow();
        Point::new((index as f64 + 0.5) * props.gauge_hop, props.gauge_center_y)
    }

    fn value_to_angle(&self, value: Option<D>) -> f64 {
        match value {
            Some(value) => value.into() * TAU / 100.,
            None => 0.0,
        }
    }

    fn update_tooltip_content(&self) {
        // let gauge = channels[0].entities[focused_entity_index] as Gauge;
        // tooltip.style
        //   ..borderColor = gauge.color
        //   ..padding = "4px 12px";
        // let label = tooltip_label_formatter(gauge.name);
        // let value = tooltip_value_formatter(gauge.value);
        // tooltip.innerHtml = "$label: <strong>$value%</strong>";
    }

    fn get_entity_group_index(&self, x: f64, y: f64) -> i64 {
        // let p = Point(x, y);
        // for (Gauge g in channels[0].entities) {
        //   if (g.containsPoint(p)) return g.index;
        // }
        // return -1;
        unimplemented!()
    }

    // /// Called when [data_table] has been changed.
    // fn data_changed(&self) {
    //     info!("data_changed");
    //     // self.calculate_drawing_sizes(ctx);
    //     self.create_channels(0, self.base.data.meta.len());
    // }
}

impl<'a, C, M, D> Chart<'a, C, M, D, GaugeEntity<D>> for GaugeChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display + Copy + Into<f64> + Ord + Default,
{
    fn calculate_drawing_sizes(&self, ctx: &C) {
        self.base.calculate_drawing_sizes(ctx);

        let mut props = self.props.borrow_mut();

        let gauge_count = self.base.data.frames.len();
        let mut label_total_height = 0.;

        if let Some(style) = &self.base.options.labels {
            label_total_height = AXIS_LABEL_MARGIN as f64 + style.fontsize.unwrap_or(12.);
        }

        let area = &self.base.props.borrow().area;
        props.gauge_center_y = area.origin.y + 0.5 * area.size.height;
        props.gauge_hop = area.size.width / gauge_count as f64;

        let avail_w = 0.618 * props.gauge_hop; // Golden ratio.
        let avail_h = area.size.height - 2. * label_total_height as f64;
        props.gauge_outer_radius = 0.5 * avail_w.min(avail_h) / HIGHLIGHT_OUTER_RADIUS_FACTOR;
        props.gauge_inner_radius = 0.5 * props.gauge_outer_radius;
    }

    fn set_stream(&mut self, stream: DataStream<'a, M, D>) {
        self.base.data = stream;
        self.create_channels(0, self.base.data.meta.len());
    }

    fn draw(&self, ctx: &C) {
        self.base.dispose();
        // data_tableSubscriptionTracker
        //   ..add(dataTable.onCellChange.listen(data_cell_changed))
        //   ..add(dataTable.onColumnsChange.listen(dataColumnsChanged))
        //   ..add(dataTable.onRowsChange.listen(data_rows_changed));
        // self.easing = get_easing(self.options.animation().easing);
        // self.base.initialize_legend();
        // self.base.initialize_tooltip();
        // self.base.position_legend();

        // This call is redundant for row and column changes but necessary for
        // cell changes.
        self.calculate_drawing_sizes(ctx);
        self.update_channel(0);

        self.draw_frame(ctx, None);
    }

    fn resize(&self, w: f64, h: f64) {
        self.base.resize(w, h);
    }

    /// Draws the axes and the grid.
    ///
    fn draw_axes_and_grid(&self, ctx: &C) {
        self.base.draw_axes_and_grid(ctx);
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
        ctx.set_stroke_color(palette::WHITE);
        // ctx.set_text_align(TextAlign::Center);

        let channels = self.base.channels.borrow();
        let labels = &self.base.options.labels;
        // let mut focused_entity_index = self.base.props.borrow().focused_entity_index;
        let focused_entity_index = -1;
        match channels.first() {
            Some(channel) => {
                if channel.state == Visibility::Showing || channel.state == Visibility::Shown {
                    for entity in channel.entities.iter() {
                        let highlight = entity.index as i64 == focused_entity_index;
                        entity.draw(ctx, percent, highlight);

                        if let Some(style) = labels {
                            let x = entity.center.x;
                            let y = entity.center.y
                                + entity.outer_radius
                                + style.fontsize.unwrap_or(12.)
                                + AXIS_LABEL_MARGIN as f64;
                            ctx.set_fill_color(style.color);

                            ctx.set_font(
                                &style.fontfamily.unwrap_or(DEFAULT_FONT_FAMILY),
                                style.fontstyle.unwrap_or(TextStyle::Normal),
                                TextWeight::Normal,
                                style.fontsize.unwrap_or(12.),
                            );
                            // ctx.set_text_align(TextAlign::Center);
                            let w = ctx.measure_text(&entity.name).width;
                            ctx.fill_text(&entity.name, x - 0.5 * w, y);
                        }
                    }
                }
            }
            None => {}
        }
        return false;
    }

    fn update_channel(&self, _: usize) {
        let len = self.base.data.frames.len();
        let props = self.props.borrow();
        let mut channels = self.base.channels.borrow_mut();
        let mut idx = 0;

        match channels.first_mut() {
            Some(channel) => {
                for entity in channel.entities.iter_mut() {
                    match entity.value {
                        Some(value) => {
                            let color = self.base.get_color(idx);
                            let highlight_color = self.base.change_color_alpha(color, 0.5);

                            entity.index = idx;
                            entity.color = color;
                            entity.highlight_color = highlight_color;

                            // here focus
                            entity.center = self.get_gauge_center(idx);
                            entity.inner_radius = props.gauge_inner_radius;
                            entity.outer_radius = props.gauge_outer_radius;
                            entity.end_angle = START_ANGLE + self.value_to_angle(entity.value);
                        }
                        None => {}
                    }
                    idx += 1;
                }
            }
            None => {}
        }
    }

    fn create_entity(
        &self,
        channel_index: usize,
        entity_index: usize,
        value: Option<D>,
        color: Color,
        highlight_color: Color,
    ) -> GaugeEntity<D> {
        // Override the colors.
        let color = self.base.get_color(entity_index);
        let highlight_color = self.base.change_color_alpha(color, 0.5);

        let stream = &self.base.data;
        let frame = stream.frames.get(entity_index).unwrap();
        let name = format!("{}", frame.metric);

        let props = self.props.borrow();
        let options = &self.base.options;
        let center = self.get_gauge_center(entity_index);

        GaugeEntity {
            index: entity_index,
            value,
            name,
            color,
            background_color: options.gauge_background,
            highlight_color,
            old_value: None,
            old_start_angle: START_ANGLE,
            old_end_angle: START_ANGLE,
            center,
            inner_radius: props.gauge_inner_radius,
            outer_radius: props.gauge_outer_radius,
            start_angle: START_ANGLE,
            end_angle: START_ANGLE + self.value_to_angle(value),
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
    ) -> Vec<GaugeEntity<D>> {
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

            result.push(entity);
            start += 1;
        }
        result
    }

    fn get_tooltip_position(&self, tooltip_width: f64, tooltip_height: f64) -> Point<f64> {
        // let channels = self.base.channels.borrow();
        // let channel = channels.first().unwrap();

        // let focused_entity_index = self.base.props.borrow().focused_entity_index as usize;

        // let gauge = channel.entities.get(focused_entity_index).unwrap();
        // let x = gauge.center.x - (tooltip_width / 2.).trunc();
        // let y = gauge.center.y
        //     - HIGHLIGHT_OUTER_RADIUS_FACTOR * gauge.outer_radius
        //     - tooltip_height
        //     - 5.;

        // Point::new(x, y)
        unimplemented!()
    }
}
