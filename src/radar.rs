#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};
use ux_dataflow::*;
use ux_primitives::{
    canvas::CanvasContext,
    color::Color,
    geom::{Point, Rect, Size},
    text::{BaseLine, TextAlign, TextStyle, TextWeight},
};

use crate::*;

#[derive(Default, Clone)]
pub struct PolarPoint {
    color: Color,
    highlight_color: Color,
    // formatted_value: String,
    index: usize,
    old_value: f64,
    value: f64,

    old_radius: f64,
    old_angle: f64,
    old_point_radius: f64,

    radius: f64,
    angle: f64,
    point_radius: f64,

    center: Point<f64>,
}

impl<C> Drawable<C> for PolarPoint
where
    C: CanvasContext,
{
    fn draw(&self, ctx: &C, percent: f64, highlight: bool) {
        let r = lerp(self.old_radius, self.radius, percent);
        let a = lerp(self.old_angle, self.angle, percent);
        let pr = lerp(self.old_point_radius, self.point_radius, percent);
        let p = polar2cartesian(&self.center, r, a);
        if highlight {
            ctx.set_fill_style_color(self.highlight_color);
            ctx.begin_path();
            ctx.arc(p.x, p.y, 2. * pr, 0., TAU, false);
            ctx.fill();
        }
        ctx.set_fill_style_color(self.color);
        ctx.begin_path();
        ctx.arc(p.x, p.y, pr, 0., TAU, false);
        ctx.fill();
        ctx.stroke();
    }
}

impl Entity for PolarPoint {
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
    // yLabelFormatter: ValueFormatter,
    /// Each element is the bounding box of each entity group.
    /// A `null` element means the group has no visible entities.
    bounding_boxes: Vec<Rect<f64>>,
}

pub struct RadarChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
{
    props: RefCell<RadarChartProperties>,
    base: BaseChart<'a, C, PolarPoint, M, D, RadarChartOptions<'a>>,
}

impl<'a, C, M, D> RadarChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
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

    pub fn value2radius(&self, value: f64) -> f64 {
        if value != 0.0 {
            let props = self.props.borrow();
            return value * props.radius / props.y_max_value;
        }
        0.0
    }

    fn calculate_bounding_boxes(&self) {
        if !self.base.options.tooltip.enabled {
            return;
        }

        let series_list = self.base.series_list.borrow();
        let series_count = series_list.len();
        let series = series_list.get(0).unwrap();
        let entity_count = series.entities.len();

        let series_states = &self.base.props.borrow().series_states;
        let series_list = self.base.series_list.borrow();
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
            for jdx in 0..series_count {
                let series_state = series_states.get(jdx).unwrap();
                if *series_state == Visibility::Hidden || *series_state == Visibility::Hiding {
                    continue;
                }

                let series = series_list.get(jdx).unwrap();
                let pp = series.entities.get(idx).unwrap();

                if pp.value == 0. {
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
        let series_list = self.base.series_list.borrow();
        let series = series_list.first().unwrap();
        let points = &series.entities;

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

    fn series_visibility_changed(&self, index: usize) {
        // let visible = series_states[index].index >= Visibility::showing.index;
        // let marker_size = self.base.options.series.markers.size;
        // for (PolarPoint p in series_list[index].entities) {
        //   if (visible) {
        //     p.radius = value2radius(p.value);
        //     p.pointRadius = marker_size;
        //   } else {
        //     p.radius = 0.0;
        //     p.pointRadius = 0;
        //   }
        // }

        // calculate_bounding_boxes();
    }
}

impl<'a, C, M, D> Chart<'a, C, M, D, PolarPoint> for RadarChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
{
    fn calculate_drawing_sizes(&self) {
        self.base.calculate_drawing_sizes();

        // xlabels = data_table.getColumnValues<String>(0);
        // angle_interval = TAU / xlabels.len();

        let rect = &self.base.props.borrow().series_and_axes_box;
        // let xLabelfont_size = self.base.options.x_axis.labels.style.font_size;

        // // [_radius]*factor equals the height of the largest polygon.
        // let factor = 1 + sin((xlabels.len() >> 1) * angle_interval - PI_2);
        // radius = min(rect.width, rect.height) / factor -
        //     factor * (xLabelfont_size + AXIS_LABEL_MARGIN);
        // center =
        //     Point(rect.left + rect.width / 2, rect.top + rect.height / factor);

        // // The minimum value on the y-axis is always zero.
        // let yInterval = self.base.options.y_axis.interval;
        // if (yInterval == null) {
        //   let yMinInterval = self.base.options.y_axis.min_interval;
        //   y_max_value = find_max_value(self.base.data_table);
        //   yInterval = utils::calculate_interval(y_max_value, 3, yMinInterval);
        //   y_max_value = (y_max_value / yInterval).ceilToDouble() * yInterval;
        // }

        // ylabel_formatter = self.base.options.y_axis.labels.formatter;
        // if (ylabel_formatter == null) {
        //   let decimalPlaces = utils::get_decimal_places(yInterval);
        //   let numberFormat = NumberFormat.decimalPattern()
        //     ..maximumFractionDigits = decimalPlaces
        //     ..minimumFractionDigits = decimalPlaces;
        //   ylabel_formatter = numberFormat.format;
        // }
        // entity_value_formatter = ylabel_formatter;

        // ylabels = <String>[];
        // let value = 0.0;
        // while (value <= y_max_value) {
        //   ylabels.add(ylabel_formatter(value));
        //   value += yInterval;
        // }

        // ylabel_hop = radius / (ylabels.len() - 1);

        // // Tooltip.

        // tooltip_value_formatter =
        //     self.base.options.tooltip.value_formatter ?? ylabel_formatter;
        unimplemented!()
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
        self.calculate_bounding_boxes();
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
            ctx.set_stroke_style_color(self.base.options.x_axis.grid_line_color);
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
            ctx.set_stroke_style_color(self.base.options.y_axis.grid_line_color);
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
        ctx.set_fill_style_color(style.color);

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
        ctx.set_fill_style_color(style.color);

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
        // let fill_opacity = self.base.options.series.fill_opacity;
        // let series_line_width = self.base.options.series.line_width;
        // let marker_options = self.base.options.series.markers;
        // let marker_size = marker_options["size"];
        // let point_count = xlabels.len();

        // for (let i = 0; i < series_list.len(); i++) {
        //   if (series_states[i] == Visibility::hidden) continue;

        //   let series = series_list[i];
        //   let scale = (i != focused_series_index) ? 1 : 2;

        //   // Draw the polygon.

        //   ctx
        //     ..line_width = scale * series_line_width
        //     ..strokeStyle = series.color
        //     ..begin_path();
        //   for (let j = 0; j < point_count; j++) {
        //     let point = series.entities[j] as PolarPoint;
        //     // TODO: Optimize.
        //     let radius = lerp(point.oldRadius, point.radius, percent);
        //     let angle = lerp(point.oldAngle, point.angle, percent);
        //     let p = utils::polar2cartesian(center, radius, angle);
        //     if (j > 0) {
        //       ctx.line_to(p.x, p.y);
        //     } else {
        //       ctx.move_to(p.x, p.y);
        //     }
        //   }
        //   ctx.closePath();
        //   ctx.stroke();

        //   // Optionally fill the polygon.

        //   if (fill_opacity > 0) {
        //     ctx.fillStyle = change_color_alpha(series.color, fill_opacity);
        //     ctx.fill();
        //   }

        //   // Draw the markers.

        //   if (marker_size > 0) {
        //     let fillColor = marker_options["fillColor"] ?? series.color;
        //     let strokeColor = marker_options["strokeColor"] ?? series.color;
        //     ctx
        //       ..fillStyle = fillColor
        //       ..line_width = scale * marker_options["line_width"]
        //       ..strokeStyle = strokeColor;
        //     for (let p in series.entities) {
        //       if (marker_options["enabled"]) {
        //         p.draw(ctx, percent, p.index == focused_entity_index);
        //       } else if (p.index == focused_entity_index) {
        //         // Only draw marker on hover.
        //         p.draw(ctx, percent, true);
        //       }
        //     }
        //   }
        // }

        // return false;
        unimplemented!()
    }

    // param should be Option
    fn update_series(&self, index: usize) {
        let entity_count = self.base.data_table.frames.len();
        let mut series_list = self.base.series_list.borrow_mut();
        let props = self.props.borrow();
        let series_states = &self.base.props.borrow().series_states;

        for idx in 0..series_list.len() {
            let mut series = series_list.get_mut(idx).unwrap();
            
            let color = self.base.get_color(idx);
            let highlight_color = self.base.get_highlight_color(color);
            series.color = color;
            series.highlight_color = highlight_color;

            let series_state = series_states[idx];
            let visible = series_state == Visibility::Showing || series_state == Visibility::Shown;
            for jdx in 0..entity_count {
                let mut entity = series.entities.get_mut(jdx).unwrap();
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
        }
    }

    fn create_entity(
        &self,
        series_index: usize,
        entity_index: usize,
        value: f64,
        color: Color,
        highlight_color: Color,
    ) -> PolarPoint {
        let props = self.props.borrow();
        let angle = self.get_angle(entity_index);
        let point_radius = self.base.options.series.markers.size as f64;

        PolarPoint {
            index: entity_index,
            value,
            old_value: 0.,
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

    fn get_tooltip_position(&self, tooltip_width: f64, tooltip_height: f64) -> Point<f64> {
        let props = self.props.borrow();
        let focused_entity_index = self.base.props.borrow().focused_entity_index;

        let bounding_box = &props.bounding_boxes[focused_entity_index as usize];
        let offset = self.base.options.series.markers.size as f64 * 2. + 5.;
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
