#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};
use ux_dataflow::*;
use ux_primitives::{canvas::CanvasContext, geom::Point};

use crate::*;

#[derive(Default, Clone)]
pub struct GaugeEntity {
    // Chart chart,
    color: String,
    highlight_color: String,
    formatted_value: String,
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

    background_color: String,
    base: PieEntity,
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
        // let chartStartAngle = (chart as dynamic).startAngle;

        // Make sure [angle] is in range [chartStartAngle]..[chartStartAngle] + TAU.
        // angle = (angle - chartStartAngle) % TAU + chartStartAngle;

        // If counterclockwise, make sure [angle] is in range
        // [start] - 2*pi..[start].
        // if startAngle > endAngle {
        //     angle -= angle - TAU;
        // }

        // if (startAngle <= endAngle) {
        //   // Clockwise.
        //   return is_in_range(angle, startAngle, endAngle);
        // } else {
        //   // Counterclockwise.
        //   return is_in_range(angle, endAngle, startAngle);
        // }
        unimplemented!()
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
    fn draw(&self, ctx: C, percent: f64, highlight: bool) {
        // let tmpColor = color;
        // let tmpEndAngle = endAngle;

        // // Draw the background.

        // endAngle = startAngle + TAU;
        // color = backgroundColor;
        // self.base.draw(ctx, 1.0, false);

        // // Draw the foreground.

        // color = tmpColor;
        // endAngle = tmpEndAngle;
        // self.base.draw(ctx, percent, highlight);

        // // Draw the percent.

        // let fs1 = .75 * inner_radius;
        // let font1 = "${fs1}px $_fontFamily";
        // let text1 = lerp(oldValue, value, percent).round().to_string();
        // ctx.font = font1;
        // let w1 = ctx.measureText(text1).width;

        // let fs2 = .6 * fs1;
        // let font2 = "${fs2}px $_fontFamily";
        // let text2 = "%";
        // ctx.font = font2;
        // let w2 = ctx.measureText(text2).width;

        // let y = center.y + .3 * fs1;
        // ctx.set_font(font1);
        // ctx.fill_text(text1, center.x - .5 * w2, y);
        // ctx.set_font(font2);
        // ctx.fill_text(text2, center.x + .5 * w1, y);
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

    fn get_gauge_center(&self, index: i64) -> Point<D> {
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

        // let gaugeCount = data_table.rows.length;
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

        // self.axes_context.clearRect(0, 0, self.width, self.height);
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
            // for (let i = series_states.length - 1; i >= 0; i--) {
            //     if (series_states[i] == Visibility::showing) {
            //         series_states[i] = Visibility::shown;
            //     } else if (series_states[i] == Visibility::hiding) {
            //         series_states[i] = Visibility::hidden;
            //     }
            // }
        }

        let props = self.base.props.borrow();

        let ease = props.easing_function.unwrap();
        self.draw_series(ease(percent));
        // context.drawImageScaled(axes_context.canvas, 0, 0, width, height);
        // context.drawImageScaled(series_context.canvas, 0, 0, width, height);
        self.base.draw_title(ctx);

        if percent < 1.0 {
            // animation_frame_id = window.requestAnimationFrame(draw_frame);
        } else if time.is_some() {
            self.base.animation_end();
        }
    }

    fn draw_series(&self, percent: f64) -> bool {
        // let style = self.base.options.gauge_labels.style;
        // let labelsEnabled = self.base.options.gauge_labels.enabled;
        // ctx.set_stroke_style_color("white");
        // ctx.set_text_align("center");
        // for (Gauge gauge in series_list[0].entities) {
        //   let highlight = gauge.index == focused_entity_index;
        //   gauge.draw(series_context, percent, highlight);

        //   if (!labelsEnabled) continue;

        //   let x = gauge.center.x;
        //   let y = gauge.center.y +
        //       gauge.outer_radius +
        //       style["fontSize"] +
        //       AXIS_LABEL_MARGIN;
        //   ctx.set_fill_style_color(style.color);
        //   ctx.set_font(utils::get_font(style));
        //   ctx.set_text_align("center");
        //   ctx.fill_text(gauge.name, x, y);
        // }
        // return false;
        unimplemented!()
    }

    fn update_series(&self, index: usize) {
        // let n = data_table.rows.length;
        // for (let i = 0; i < n; i++) {
        //   let gauge = series_list[0].entities[i] as Gauge;
        //   let color = get_color(i);
        //   let highlight_color = change_color_alpha(color, .5);
        //   gauge
        //     ..index = i
        //     ..name = data_table.rows[i][0]
        //     ..color = color
        //     ..highlight_color = highlight_color
        //     ..center = getGaugeCenter(i)
        //     ..inner_radius = gaugeInnerRadius
        //     ..outer_radius = gaugeOuterRadius
        //     ..endAngle = startAngle + valueToAngle(gauge.value);
        // }
    }

    fn create_entity(
        &self,
        series_index: usize,
        entity_index: usize,
        value: String,
        color: String,
        highlight_color: String,
    ) -> GaugeEntity {
        // Override the colors.
        // let color = self.get_color(entity_index);
        // let highlight_color = self.change_color_alpha(color, .5);

        // let name = data_table.rows[entityIndex][0];
        // Gauge()
        //   ..index = entityIndex
        //   ..value = value
        //   ..name = name
        //   ..color = color
        //   ..backgroundColor = self.base.options.gauge_background_color
        //   ..highlight_color = highlight_color
        //   ..oldValue = 0
        //   ..oldStartAngle = startAngle
        //   ..oldEndAngle = startAngle
        //   ..center = getGaugeCenter(entityIndex)
        //   ..inner_radius = gaugeInnerRadius
        //   ..outer_radius = gaugeOuterRadius
        //   ..startAngle = startAngle
        //   ..endAngle = startAngle + valueToAngle(value);
        unimplemented!()
    }

    fn get_tooltip_position(&self) -> Point<f64> {
        // let gauge = series_list[0].entities[focused_entity_index] as Gauge;
        // let x = gauge.center.x - (tooltip.offset_width / 2).trunc();
        // let y = gauge.center.y -
        //     HIGHLIGHT_OUTER_RADIUS_FACTOR * gauge.outer_radius -
        //     tooltip.offset_height -
        //     5;
        // return Point(x, y);
        unimplemented!()
    }
}
