#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::{collections::HashMap, fmt, cell::RefCell, rc::Rc};
use ux_primitives::{
    canvas::CanvasContext,
    geom::{Point, Rect}
};
use ux_dataflow::*;

use crate::*;

#[derive(Default, Clone)]
pub struct PolarPoint {
    // Chart chart,
    color: String,
    highlight_color: String,
    formatted_value: String,
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
    fn draw(&self, ctx: C, percent: f64, highlight: bool) {
        let r = lerp(self.old_radius, self.radius, percent);
        let a = lerp(self.old_angle, self.angle, percent);
        let pr = lerp(self.old_point_radius, self.point_radius, percent);
        let p = polar2cartesian(&self.center, r, a);
        if highlight {
            // ctx.set_fill_style_color(value)
            //   ctx.fillStyle = highlight_color;
            //   ctx.begin_path();
            //   ctx.arc(p.x, p.y, 2 * pr, 0, TAU);
            //   ctx.fill();
        }
        // ctx.fillStyle = color;
        // ctx.begin_path();
        // ctx.arc(p.x, p.y, pr, 0, TAU);
        // ctx.fill();
        // ctx.stroke();
        unimplemented!()
    }
}

impl Entity for PolarPoint {
    fn free(&mut self) {
        // chart = null;
    }

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
        // if !self.base.options.tooltip.enabled return;

        // let seriesCount = series_list.length;
        // let entityCount = series_list.first.entities.length;
        // bounding_boxes = Vec<Rectangle>(entityCount);
        // for (let i = 0; i < entityCount; i++) {
        //   let minX = f64::MAX;
        //   let minY = f64::MAX;
        //   let maxX = -f64::MAX;
        //   let maxY = -f64::MAX;
        //   let count = 0;
        //   for (let j = 0; j < seriesCount; j++) {
        //     if (series_states[j] == Visibility::hidden) continue;
        //     if (series_states[j] == Visibility::hiding) continue;

        //     let pp = series_list[j].entities[i] as PolarPoint;
        //     if (pp.value == null) continue;

        //     let cp = polarToCartesian(pp.center, pp.radius, pp.angle);
        //     minX = min(minX, cp.x);
        //     minY = min(minY, cp.y);
        //     maxX = max(maxX, cp.x);
        //     maxY = max(maxY, cp.y);
        //     count++;
        //   }
        //   bounding_boxes[i] =
        //       count > 0 ? Rectangle(minX, minY, maxX - minX, maxY - minY) : null;
        // }
        unimplemented!()
    }

    // fn drawText(ctx: C, text: String, radius: f64, angle: f64, fontSize: f64) {
    //     // let w = ctx.measureText(text).width;
    //     // let x = center.x + cos(angle) * (radius + .5 * w);
    //     // let y = center.y + sin(angle) * (radius + .5 * fontSize);
    //     // ctx.fill_text(text, x, y);
    // }

    fn get_entity_group_index(&self, x: f64, y: f64) -> i64 {
        // let p = Point(x - center.x, y - center.y);
        // if (p.magnitude >= radius) return -1;
        // let angle = atan2(p.y, p.x);
        // let points = series_list.first.entities.cast<PolarPoint>();
        // for (let i = points.length - 1; i >= 0; i--) {
        //   if (bounding_boxes[i] == null) continue;

        //   let delta = angle - points[i].angle;
        //   if (delta.abs() < .5 * angle_interval) return i;
        //   if ((delta + TAU).abs() < .5 * angle_interval) return i;
        // }
        // return -1;
        unimplemented!()
    }

    fn series_visibility_changed(&self, index: usize) {
        // let visible = series_states[index].index >= Visibility::showing.index;
        // let markerSize = self.base.options.series.markers.size;
        // for (PolarPoint p in series_list[index].entities) {
        //   if (visible) {
        //     p.radius = value2radius(p.value);
        //     p.pointRadius = markerSize;
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
        // angle_interval = TAU / xlabels.length;
        
        let rect = &self.base.props.borrow().series_and_axes_box;
        // let xLabelFontSize = self.base.options.x_axis.labels.style.font_size;

        // // [_radius]*factor equals the height of the largest polygon.
        // let factor = 1 + sin((xlabels.length >> 1) * angle_interval - PI_2);
        // radius = min(rect.width, rect.height) / factor -
        //     factor * (xLabelFontSize + AXIS_LABEL_MARGIN);
        // center =
        //     Point(rect.left + rect.width / 2, rect.top + rect.height / factor);

        // // The minimum value on the y-axis is always zero.
        // let yInterval = self.base.options.y_axis.interval;
        // if (yInterval == null) {
        //   let yMinInterval = self.base.options.y_axis.min_interval;
        //   y_max_value = find_max_value(data_table);
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

        // ylabel_hop = radius / (ylabels.length - 1);

        // // Tooltip.

        // tooltip_value_formatter =
        //     self.base.options.tooltip.value_formatter ?? ylabel_formatter;
        unimplemented!()
    }

    fn set_stream(&self, stream: DataStream<'a, M, D>) {
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
                
        // self.axes_context.clearRect(0, 0, self.width, self.height);
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

        // let line_width = self.base.options.x_axis.grid_line_width;
        // if (lineWidth > 0) {
        //   axes_context
        //     ..lineWidth = lineWidth
        //     ..strokeStyle = self.base.options.x_axis.grid_line_color
        //     ..begin_path();
        //   let radius = radius;
        //   for (let i = yLabelCount - 1; i >= 1; i--) {
        //     let angle = -PI_2 + angle_interval;
        //     axes_context.moveTo(center.x, center.y - radius);
        //     for (let j = 0; j < xLabelCount; j++) {
        //       let point = polarToCartesian(center, radius, angle);
        //       axes_context.lineTo(point.x, point.y);
        //       angle += angle_interval;
        //     }
        //     radius -= ylabel_hop;
        //   }
        //   axes_context.stroke();
        // }

        // // y-axis grid lines (i.e. radii from the center to the x-axis labels).

        // lineWidth = self.base.options.y_axis.grid_line_width;
        // if (lineWidth > 0) {
        //   axes_context
        //     ..lineWidth = lineWidth
        //     ..strokeStyle = self.base.options.y_axis.grid_line_color
        //     ..begin_path();
        //   let angle = -PI_2;
        //   for (let i = 0; i < xLabelCount; i++) {
        //     let point = polarToCartesian(center, radius, angle);
        //     axes_context
        //       ..moveTo(center.x, center.y)
        //       ..lineTo(point.x, point.y);
        //     angle += angle_interval;
        //   }
        //   axes_context.stroke();
        // }

        // // y-axis labels - don"t draw the first (at center) and the last ones.

        // let style = self.base.options.y_axis.labels.style;
        // let x = center.x - AXIS_LABEL_MARGIN;
        // let y = center.y - ylabel_hop;
        // axes_context
        //   ..fillStyle = style["color"]
        //   ..font = get_font(style)
        //   ..textAlign = "right"
        //   ..textBaseline = "middle";
        // for (let i = 1; i <= yLabelCount - 2; i++) {
        //   axes_context.fill_text(ylabels[i], x, y);
        //   y -= ylabel_hop;
        // }

        // // x-axis labels.

        // style = self.base.options.x_axis.labels.style;
        // axes_context
        //   ..fillStyle = style["color"]
        //   ..font = get_font(style)
        //   ..textAlign = "center"
        //   ..textBaseline = "middle";
        // let fontSize = style["fontSize"];
        // let angle = -PI_2;
        // let radius = radius + AXIS_LABEL_MARGIN;
        // for (let i = 0; i < xLabelCount; i++) {
        //   drawText(axes_context, xlabels[i], radius, angle, fontSize);
        //   angle += angle_interval;
        // }
        unimplemented!()
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
        // let fillOpacity = self.base.options.series.fill_opacity;
        // let seriesLineWidth = self.base.options.series.line_width;
        // let markerOptions = self.base.options.series.markers;
        // let markerSize = markerOptions["size"];
        // let pointCount = xlabels.length;

        // for (let i = 0; i < series_list.length; i++) {
        //   if (series_states[i] == Visibility::hidden) continue;

        //   let series = series_list[i];
        //   let scale = (i != focused_series_index) ? 1 : 2;

        //   // Draw the polygon.

        //   series_context
        //     ..lineWidth = scale * seriesLineWidth
        //     ..strokeStyle = series.color
        //     ..begin_path();
        //   for (let j = 0; j < pointCount; j++) {
        //     let point = series.entities[j] as PolarPoint;
        //     // TODO: Optimize.
        //     let radius = lerp(point.oldRadius, point.radius, percent);
        //     let angle = lerp(point.oldAngle, point.angle, percent);
        //     let p = polarToCartesian(center, radius, angle);
        //     if (j > 0) {
        //       series_context.lineTo(p.x, p.y);
        //     } else {
        //       series_context.moveTo(p.x, p.y);
        //     }
        //   }
        //   series_context.closePath();
        //   series_context.stroke();

        //   // Optionally fill the polygon.

        //   if (fillOpacity > 0) {
        //     series_context.fillStyle = change_color_alpha(series.color, fillOpacity);
        //     series_context.fill();
        //   }

        //   // Draw the markers.

        //   if (markerSize > 0) {
        //     let fillColor = markerOptions["fillColor"] ?? series.color;
        //     let strokeColor = markerOptions["strokeColor"] ?? series.color;
        //     series_context
        //       ..fillStyle = fillColor
        //       ..lineWidth = scale * markerOptions["lineWidth"]
        //       ..strokeStyle = strokeColor;
        //     for (let p in series.entities) {
        //       if (markerOptions["enabled"]) {
        //         p.draw(series_context, percent, p.index == focused_entity_index);
        //       } else if (p.index == focused_entity_index) {
        //         // Only draw marker on hover.
        //         p.draw(series_context, percent, true);
        //       }
        //     }
        //   }
        // }

        // return false;
        unimplemented!()
    }

    // param should be Option
    fn update_series(&self, index: usize) {
        // let entityCount = data_table.rows.length;
        // for (let i = 0; i < series_list.length; i++) {
        //   let series = series_list[i];
        //   let color = get_color(i);
        //   let highlight_color = get_highlight_color(color);
        //   let visible = series_states[i].index >= Visibility::showing.index;
        //   series.color = color;
        //   series.highlight_color = highlight_color;
        //   for (let j = 0; j < entityCount; j++) {
        //     let p = series.entities[j] as PolarPoint;
        //     p.index = j;
        //     p.center = center;
        //     p.radius = visible ? value2radius(p.value) : 0.0;
        //     p.angle = get_angle(j);
        //     p.color = color;
        //     p.highlight_color = highlight_color;
        //   }
        // }
    }

    fn create_entity(
        &self,
        series_index: usize,
        entity_index: usize,
        value: String,
        color: String,
        highlight_color: String,
    ) -> PolarPoint {
        // let angle = self.get_angle(entity_index);
        // PolarPoint()
        //   ..index = entityIndex
        //   ..value = value
        //   ..color = color
        //   ..highlight_color = highlight_color
        //   ..center = center
        //   ..oldRadius = 0
        //   ..oldAngle = angle
        //   ..oldPointRadius = 0
        //   ..radius = value2radius(value)
        //   ..angle = angle
        //   ..pointRadius = self.base.options.series.markers.size;
        unimplemented!()
    }

    fn get_tooltip_position(&self) -> Point<f64> {
        // FIXME: as usize
        let props = self.props.borrow();
        let focused_entity_index = self.base.props.borrow().focused_entity_index;

        let bbox = &props.bounding_boxes[focused_entity_index as usize];
        // let offset = self.base.options.series.markers.size * 2 + 5;
        // let x = box.right + offset;
        // let y = box.top + ((box.height - tooltip.offset_height) / 2).trunc();
        // if (x + tooltip.offset_width > width)
        //   x = box.left - tooltip.offset_width - offset;
        // return Point(x, y);
        unimplemented!()
    }
}
