#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use animate::easing::{get_easing, Easing};
use dataflow::*;
use primitives::{palette, CanvasContext, Color, Point, TextAlign, TextStyle, TextWeight};
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::*;

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
        let mut a1 = utils::lerp(self.old_start_angle, self.start_angle, percent);
        let mut a2 = utils::lerp(self.old_end_angle, self.end_angle, percent);
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
{
    fn draw(&self, ctx: &C, percent: f64, highlight: bool) {
        let tmp_color = &self.color;
        let tmp_end_angle = self.end_angle;

        // Draw the background.
        // FIXME:
        // self.end_angle = self.start_angle + TAU;
        // self.color = self.background_color;
        self.draw_entity(ctx, 1.0, false);

        // Draw the foreground.
        // FIXME:
        // self.color = *tmp_color;
        // self.end_angle = tmp_end_angle;
        self.draw_entity(ctx, percent, highlight);

        // Draw the percent.
        let fs1 = 0.75 * self.inner_radius;
        let family = DEFAULT_FONT_FAMILY;
        // FIXME: empty entity
        // let text1 = utils::lerp(self.old_value.unwrap(), self.value.unwrap(), percent)
        //     .round()
        //     .to_string();
        // ctx.set_font(family, TextStyle::Normal, TextWeight::Normal, fs1);
        // let w1 = ctx.measure_text(text1.as_str()).width;

        // let fs2 = 0.6 * fs1;
        // let text2 = "%";
        // ctx.set_font(family, TextStyle::Normal, TextWeight::Normal, fs2);
        // let w2 = ctx.measure_text(text2).width;

        // let y = self.center.y + 0.3 * fs1;
        // ctx.set_font(family, TextStyle::Normal, TextWeight::Normal, fs1);
        // ctx.fill_text(text1.as_str(), self.center.x - 0.5 * w2, y);
        // ctx.set_font(family, TextStyle::Normal, TextWeight::Normal, fs2);
        // ctx.fill_text(text2, self.center.x + 0.5 * w1, y);
    }
}

#[derive(Default, Clone)]
struct GaugeChartProperties {
    gauge_hop: f64,
    gauge_inner_radius: f64,
    gauge_outer_radius: f64,
    gauge_center_y: f64,
    start_angle: f64, // = -f64::FRAC_PI_2;
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
    D: fmt::Display + Copy,
{
    pub fn new(options: GaugeChartOptions<'a>) -> Self {
        Self {
            props: Default::default(),
            base: BaseChart::new(options),
        }
    }

    fn get_gauge_center(&self, index: usize) -> Point<f64> {
        // Point((index + 0.5) * gauge_hop, gauge_center_y)
        unimplemented!()
    }

    fn value_to_angle(&self, value: D) -> f64 {
        // value * TAU / 100
        unimplemented!()
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

    /// Called when [data_table] has been changed.
    fn data_table_changed(&self) {
        info!("data_table_changed");
        // self.calculate_drawing_sizes(ctx);
        self.create_channels(0, self.base.data_table.meta.len());
    }
}

impl<'a, C, M, D> Chart<'a, C, M, D, GaugeEntity<D>> for GaugeChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display + Copy,
{
    fn calculate_drawing_sizes(&self, ctx: &C) {
        self.base.calculate_drawing_sizes(ctx);

        let mut props = self.props.borrow_mut();

        let gauge_count = self.base.data_table.frames.len();
        let mut label_total_height = 0.;

        if let Some(style) = &self.base.options.labels {
            label_total_height = AXIS_LABEL_MARGIN as f64 + style.font_size.unwrap_or(12.);
        }

        let channel_and_axes_box = &self.base.props.borrow().channel_and_axes_box;
        props.gauge_center_y = channel_and_axes_box.origin.y + 0.5 * channel_and_axes_box.size.height;
        props.gauge_hop = channel_and_axes_box.size.width / gauge_count as f64;

        let avail_w = 0.618 * props.gauge_hop; // Golden ratio.
        let avail_h = channel_and_axes_box.size.height - 2. * label_total_height as f64;
        props.gauge_outer_radius = 0.5 * avail_w.min(avail_h) / HIGHLIGHT_OUTER_RADIUS_FACTOR;
        props.gauge_inner_radius = 0.5 * props.gauge_outer_radius;
    }

    fn set_stream(&mut self, stream: DataStream<'a, M, D>) {
        self.base.data_table = stream;
    }

    fn draw(&self, ctx: &C) {
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
        self.update_channel(0);

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
        self.base.draw_axes_and_grid(ctx);
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
        ctx.set_stroke_color(palette::WHITE);
        ctx.set_text_align(TextAlign::Center);

        let channels = self.base.channels.borrow();
        let channel = channels.first().unwrap();

        let labels = &self.base.options.labels;

        let focused_entity_index = self.base.props.borrow().focused_entity_index as usize;

        for entity in channel.entities.iter() {
            // let highlight = entity.index == focused_entity_index;
            // entity.draw(ctx, percent, highlight);
            // if let Some(style) = labels {
            //     let x = entity.center.x;
            //     let y = entity.center.y
            //         + entity.outer_radius
            //         + style.font_size.unwrap_or(12.)
            //         + AXIS_LABEL_MARGIN as f64;
            //     ctx.set_fill_color(style.color);

            //     ctx.set_font(
            //         &style.font_family.unwrap_or(DEFAULT_FONT_FAMILY),
            //         style.font_style.unwrap_or(TextStyle::Normal),
            //         TextWeight::Normal,
            //         style.font_size.unwrap_or(12.),
            //     );
            //     ctx.set_text_align(TextAlign::Center);
            //     ctx.fill_text(&entity.name, x, y);
            // }
        }
        return false;
    }

    fn update_channel(&self, _: usize) {
        let len = self.base.data_table.frames.len();
        let props = self.props.borrow();
        let mut channels = self.base.channels.borrow_mut();
        let channel = channels.first_mut().unwrap();
        for idx in 0..len {
            // let color = self.base.get_color(idx);
            // let highlight_color = self.base.change_color_alpha(color, 0.5);
            // let entity = channel.entities.get_mut(idx).unwrap();
            // entity.index = idx;
            // // TODO: deal with name
            // //   gauge.name = self.base.data_table.frames[idx][0];
            // entity.color = color;
            // entity.highlight_color = highlight_color;
            // entity.center = self.get_gauge_center(idx);
            // entity.inner_radius = props.gauge_inner_radius;
            // entity.outer_radius = props.gauge_outer_radius;
            // entity.end_angle = props.start_angle + self.value_to_angle(entity.value);
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

        let stream = &self.base.data_table;
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
            background_color: options.background_color,
            highlight_color,
            old_value: None,
            old_start_angle: props.start_angle,
            old_end_angle: props.start_angle,
            center,
            inner_radius: props.gauge_inner_radius,
            outer_radius: props.gauge_outer_radius,
            start_angle: props.start_angle,
            end_angle: props.start_angle + self.value_to_angle(value.unwrap()),
        }
    }

    fn create_channels(&self, start: usize, end: usize) {
        println!("create_channels");
        let result = Vec::new();
        // let entity_count = self.data_table.frames.len();
        // while (start < end) {
        //   let name = self.base.data_table.columns[start + 1].name;
        //   let color = get_color(start);
        //   let highlight_color = get_highlight_color(color);
        //   let entities =
        //       create_entities(start, 0, entity_count, color, highlight_color);
        //   result.add(Series(name, color, highlight_color, entities));
        //   start++;
        // }
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
        info!("create_entities");
        let result = Vec::new();
        // while (start < end) {
        //   let value = self.base.data_table.rows[start][channel_index + 1];
        //   let e = create_entity(channel_index, start, value, color, highlight_color);
        //   e.chart = this;
        //   result.add(e);
        //   start++;
        // }
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
