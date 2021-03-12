#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};
use ux_dataflow::*;
use ux_primitives::{
    canvas::CanvasContext,
    color::Color,
    geom::Point,
    text::{TextAlign, TextStyle, TextWeight},
};

use crate::*;

#[derive(Default, Clone)]
pub struct GaugeEntity {
    // Chart chart,
    color: Color,
    highlight_color: Color,
    // formatted_value: String,
    index: usize,
    old_value: f64,
    value: f64,

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

impl GaugeEntity {
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
            ctx.set_fill_style_color(self.highlight_color);
            ctx.begin_path();
            ctx.arc(center.x, center.y, highlight_outer_radius, a1, a2, false);
            ctx.arc(center.x, center.y, self.inner_radius, a2, a1, true);
            ctx.fill();
        }

        ctx.set_fill_style_color(self.color);
        ctx.begin_path();
        ctx.arc(center.x, center.y, self.outer_radius, a1, a2, false);
        ctx.arc(center.x, center.y, self.inner_radius, a2, a1, true);
        ctx.fill();
        ctx.stroke();
    }
}

impl Entity for GaugeEntity {
    fn free(&mut self) {
        // chart = null;
    }

    fn save(&self) {
        // self.old_start_angle = self.start_angle;
        // self.old_end_angle = self.end_angle;
        // self.old_value = self.value;
    }
}

impl<C> Drawable<C> for GaugeEntity
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
        let family = "Roboto";
        let text1 = lerp(self.old_value, self.value, percent)
            .round()
            .to_string();
        ctx.set_font(family, TextStyle::Normal, TextWeight::Normal, fs1);
        let w1 = ctx.measure_text(text1.as_str()).width;

        let fs2 = 0.6 * fs1;
        let text2 = "%";
        ctx.set_font(family, TextStyle::Normal, TextWeight::Normal, fs2);
        let w2 = ctx.measure_text(text2).width;

        let y = self.center.y + 0.3 * fs1;
        ctx.set_font(family, TextStyle::Normal, TextWeight::Normal, fs1);
        ctx.fill_text(text1.as_str(), self.center.x - 0.5 * w2, y);
        ctx.set_font(family, TextStyle::Normal, TextWeight::Normal, fs2);
        ctx.fill_text(text2, self.center.x + 0.5 * w1, y);
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
    D: fmt::Display,
{
    props: RefCell<GaugeChartProperties>,
    base: BaseChart<'a, C, GaugeEntity, M, D, GaugeChartOptions<'a>>,
}

impl<'a, C, M, D> GaugeChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
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

    fn value_to_angle(&self, value: f64) -> f64 {
        // value * TAU / 100
        unimplemented!()
    }

    fn update_tooltip_content(&self) {
        // let gauge = series_list[0].entities[focused_entity_index] as Gauge;
        // tooltip.style
        //   ..borderColor = gauge.color
        //   ..padding = "4px 12px";
        // let label = tooltip_label_formatter(gauge.name);
        // let value = tooltip_value_formatter(gauge.value);
        // tooltip.innerHtml = "$label: <strong>$value%</strong>";
    }

    fn get_entity_group_index(&self, x: f64, y: f64) -> i64 {
        // let p = Point(x, y);
        // for (Gauge g in series_list[0].entities) {
        //   if (g.containsPoint(p)) return g.index;
        // }
        // return -1;
        unimplemented!()
    }
}

impl<'a, C, M, D> Chart<'a, C, M, D, GaugeEntity> for GaugeChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
{
    fn calculate_drawing_sizes(&self) {
        self.base.calculate_drawing_sizes();

        // let gaugeCount = self.base.data_table.rows.len();
        // let labelTotalHeight = 0;
        // if (self.base.options.gauge_labels.enabled) {
        //   labelTotalHeight =
        //       AXIS_LABEL_MARGIN + self.base.options.gauge_labels.style.font_size;
        // }

        // gauge_center_y = series_and_axes_box.top + .5 * series_and_axes_box.height;
        // gauge_hop = self.base.series_and_axes_box.width / gaugeCount;

        // let availW = .618 * gauge_hop; // Golden ratio.
        // let availH = self.base.series_and_axes_box.height - 2 * labelTotalHeight;
        // gaugeOuterRadius = .5 * min(availW, availH) / HIGHLIGHT_OUTER_RADIUS_FACTOR;
        // gaugeInnerRadius = .5 * gaugeOuterRadius;
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
        // let style = self.base.options.gauge_labels.style;
        // let labelsEnabled = self.base.options.gauge_labels.enabled;
        // ctx.set_stroke_style_color("white");
        // ctx.set_text_align(TextAlign::Center);
        // for (Gauge gauge in series_list[0].entities) {
        //   let highlight = gauge.index == focused_entity_index;
        //   gauge.draw(ctx, percent, highlight);

        //   if (!labelsEnabled) continue;

        //   let x = gauge.center.x;
        //   let y = gauge.center.y +
        //       gauge.outer_radius +
        //       style["font_size"] +
        //       AXIS_LABEL_MARGIN;
        //   ctx.set_fill_style_color(style.color);
        //   ctx.set_font(utils::get_font(style));
        //   ctx.set_text_align(TextAlign::Center);
        //   ctx.fill_text(gauge.name, x, y);
        // }
        // return false;
        unimplemented!()
    }

    fn update_series(&self, index: usize) {
        let len = self.base.data_table.frames.len();
        let props = self.props.borrow();
        let mut series_list = self.base.series_list.borrow_mut();
        let series = series_list.first_mut().unwrap();
        for idx in 0..len {
            let color = self.base.get_color(idx);
            let highlight_color = self.base.change_color_alpha(color, 0.5);
            let gauge = series.entities.get_mut(idx).unwrap();
            gauge.index = idx;
            // TODO: deal with name
            //   gauge.name = self.base.data_table.frames[idx][0];
            gauge.color = color;
            gauge.highlight_color = highlight_color;
            gauge.center = self.get_gauge_center(idx);
            gauge.inner_radius = props.gauge_inner_radius;
            gauge.outer_radius = props.gauge_outer_radius;
            gauge.end_angle = props.start_angle + self.value_to_angle(gauge.value);
        }
    }

    fn create_entity(
        &self,
        series_index: usize,
        entity_index: usize,
        value: f64,
        color: Color,
        highlight_color: Color,
    ) -> GaugeEntity {
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
            old_value: 0.,
            old_start_angle: props.start_angle,
            old_end_angle: props.start_angle,
            center,
            inner_radius: props.gauge_inner_radius,
            outer_radius: props.gauge_outer_radius,
            start_angle: props.start_angle,
            end_angle: props.start_angle + self.value_to_angle(value),
        }
    }

    fn get_tooltip_position(&self, tooltip_width: f64, tooltip_height: f64) -> Point<f64> {
        let series_list = self.base.series_list.borrow();
        let series = series_list.first().unwrap();

        let focused_entity_index = self.base.props.borrow().focused_entity_index as usize;

        let gauge = series.entities.get(focused_entity_index).unwrap();
        let x = gauge.center.x - (tooltip_width / 2.).trunc();
        let y = gauge.center.y
            - HIGHLIGHT_OUTER_RADIUS_FACTOR * gauge.outer_radius
            - tooltip_height
            - 5.;

        Point::new(x, y)
    }
}
