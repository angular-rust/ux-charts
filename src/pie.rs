#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::fmt;
use ux_primitives::{canvas::*, math::*};

use crate::*;

const CLOCKWISE: i64 = 1;
const COUNTERCLOCKWISE: i64 = -1;
const HIGHLIGHT_OUTER_RADIUS_FACTOR: f64 = 1.05;

/// A pie in a pie chart.
#[derive(Default, Clone)]
pub struct PieEntity {
    // Chart chart,
    // String color,
    // String highlightColor,
    // String formattedValue,
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
}

impl PieEntity {
    pub fn is_empty(&self) -> bool {
        self.start_angle == self.end_angle
    }

    fn contains_point(&self, p: Point<f64>) -> bool {
        // p -= center;
        // let mag = p.magnitude;
        // if (mag > outerRadius || mag < innerRadius) {
        //   return false;
        // }

        // let angle = atan2(p.y, p.x);
        // let chartStartAngle = (chart as dynamic)._startAngle;

        // // Make sure [angle] is in range [chartStartAngle]..[chartStartAngle] + TAU.
        // angle = (angle - chartStartAngle) % TAU + chartStartAngle;

        // // If counterclockwise, make sure [angle] is in range
        // // [start] - 2*pi..[start].
        // if (startAngle > endAngle) angle -= TAU;

        // if (startAngle <= endAngle) {
        //   // Clockwise.
        //   return isInRange(angle, startAngle, endAngle);
        // } else {
        //   // Counterclockwise.
        //   return isInRange(angle, endAngle, startAngle);
        // }
        unimplemented!()
    }
}

impl Entity for PieEntity {
    fn free(&mut self) {
        // chart = null;
    }

    fn save(&self) {
        // self.old_start_angle = self.start_angle;
        // self.old_end_angle = self.end_angle;
        // self.old_value = self.value;
    }
}

impl<C> Drawable<C> for PieEntity
where
    C: CanvasContext,
{
    fn draw(&self, ctx: C, percent: f64, highlight: bool) {
        // let a1 = lerp(oldStartAngle, startAngle, percent);
        // let a2 = lerp(oldEndAngle, endAngle, percent);
        // if (a1 > a2) {
        //   let tmp = a1;
        //   a1 = a2;
        //   a2 = tmp;
        // }
        // if (highlight) {
        //   let highlightOuterRadius = _highlightOuterRadiusFactor * outerRadius;
        //   ctx.fillStyle = highlightColor;
        //   ctx.beginPath();
        //   ctx.arc(center.x, center.y, highlightOuterRadius, a1, a2);
        //   ctx.arc(center.x, center.y, innerRadius, a2, a1, true);
        //   ctx.fill();
        // }
        // ctx.fillStyle = color;
        // ctx.beginPath();
        // ctx.arc(center.x, center.y, outerRadius, a1, a2);
        // ctx.arc(center.x, center.y, innerRadius, a2, a1, true);
        // ctx.fill();
        // ctx.stroke();

        // if (formattedValue != null && chart is PieChart && a2 - a1 > pi / 36) {
        //   let options = chart.options["series"]["labels"];
        //   if (options["enabled"]) {
        //     let r = .25 * innerRadius + .75 * outerRadius;
        //     let a = .5 * (a1 + a2);
        //     let p = polarToCartesian(center, r, a);
        //     ctx.fillStyle = options["style"]["color"];
        //     ctx.fillText(formattedValue, p.x, p.y);
        //   }
        // }
        unimplemented!()
    }
}

pub struct PieChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
{
    center: Point<f64>,
    outer_radius: f64,
    inner_radius: f64,

    /// The start angle in radians.
    start_angle: f64,

    /// 1 means clockwise and -1 means counterclockwise.
    direction: i64,

    base: BaseChart<'a, C, PieEntity, M, D, PieChartOptions<'a>>,
}

impl<'a, C, M, D> PieChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
{
    pub fn new(options: PieChartOptions<'a>) -> Self {
        Self {
            center: Default::default(),
            outer_radius: 0.0,
            inner_radius: 0.0,
            start_angle: 0.0,
            direction: 1,
            base: BaseChart::new(options),
        }
    }

    fn data_rows_changed(&self, record: DataCollectionChangeRecord) {
        // update_series_visible(record.index, record.removedCount, record.addedCount);
        // self.base._dataRowsChanged(record);
        // update_legend_content();
        unimplemented!()
    }

    fn get_entity_group_index(&self, x: f64, y: f64) -> i64 {
        // let p = Point(x, y);
        // let entities = series_list.first.entities;
        // for (let i = entities.length - 1; i >= 0; i--) {
        //   let pie = entities[i] as _Pie;
        //   if (pie.containsPoint(p)) return i;
        // }
        // return -1;
        unimplemented!()
    }

    pub fn get_legend_labels(&self) -> Vec<String> {
        //self.data_table.getColumnValues<String>(0)
        unimplemented!()
    }

    fn series_visibility_changed(&self, index: usize) {
        self.update_series(0);
    }

    fn update_tooltip_content(&self) {
        // let pie = series_list[0].entities[focused_entity_index] as _Pie;
        // _tooltip.style
        //   ..borderColor = pie.color
        //   ..padding = "4px 12px";
        // let label = tooltip_label_formatter(pie.name);
        // let value = tooltip_value_formatter(pie.value);
        // _tooltip.innerHtml = "$label: <strong>$value</strong>";
        unimplemented!()
    }
}

impl<'a, C, M, D> Chart<PieEntity> for PieChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
{
    fn calculate_drawing_sizes(&self) {
        self.base.calculate_drawing_sizes();
        // let rect = series_and_axes_box;
        // let halfW = rect.width >> 1;
        // let halfH = rect.height >> 1;
        // _center = Point(rect.left + halfW, rect.top + halfH);
        // _outerRadius = min(halfW, halfH) / _highlightOuterRadiusFactor;
        // let pieHole = options["pieHole"];
        // if (pieHole > 1) pieHole = 0;
        // if (pieHole < 0) pieHole = 0;
        // _innerRadius = pieHole * _outerRadius;

        // let opt = options["series"];
        // entity_value_formatter =
        //     opt["labels"]["formatter"] ?? _defaultValueFormatter;
        // _direction = opt["counterclockwise"] ? _counterclockwise : _clockwise;
        // _startAngle = deg2rad(opt["startAngle"]);
        unimplemented!()
    }

    fn draw_series(&self, percent: f64) -> bool {
        // series_context
        //   ..lineWidth = 2
        //   ..strokeStyle = "#fff"
        //   ..textAlign = "center"
        //   ..textBaseline = "middle";
        // let pies = series_list.first.entities;
        // let labelOptions = options["series"]["labels"];
        // series_context.font = get_font(labelOptions["style"]);
        // for (_Pie pie in pies) {
        //   if (pie.isEmpty && percent == 1.0) continue;
        //   let highlight =
        //       pie.index == focused_series_index || pie.index == focused_entity_index;
        //   pie.draw(series_context, percent, highlight);
        // }

        // return false;
        unimplemented!()
    }

    fn update_series(&self, index: usize) {
        // // Example data table:
        // //   Browser  Share
        // //   Chrome   .35
        // //   IE       .30
        // //   Firefox  .20
        // //   Other    .15

        // let sum = 0.0;
        // let startAngle = _startAngle;
        // let pieCount = data_table.rows.length;
        // let entities = series_list[0].entities;

        // // Sum the values of all visible pies.
        // for (let i = 0; i < pieCount; i++) {
        //   if (series_states[i].index >= Visibility::showing.index) {
        //     sum += entities[i].value;
        //   }
        // }

        // for (let i = 0; i < pieCount; i++) {
        //   _Pie pie = entities[i];
        //   let color = get_color(i);
        //   pie.index = i;
        //   pie.name = data_table.rows[i][0];
        //   pie.color = color;
        //   pie.highlightColor = get_highlight_color(color);
        //   pie.center = _center;
        //   pie.innerRadius = _innerRadius;
        //   pie.outerRadius = _outerRadius;

        //   if (series_states[i].index >= Visibility::showing.index) {
        //     pie.startAngle = startAngle;
        //     pie.endAngle = startAngle + _direction * pie.value * TAU / sum;
        //     startAngle = pie.endAngle;
        //   } else {
        //     pie.startAngle = startAngle;
        //     pie.endAngle = startAngle;
        //   }
        // }
        unimplemented!()
    }

    fn create_entity(
        &self,
        series_index: usize,
        entity_index: usize,
        value: String,
        color: String,
        highlight_color: String,
    ) -> PieEntity {
        // // Override the colors.
        // color = get_color(entityIndex);
        // highlightColor = change_color_alpha(color, .5);
        // let name = data_table.rows[entityIndex][0];
        // let startAngle = _startAngle;
        // if (entityIndex > 0 && series_list != null) {
        //   let prevPie = series_list[0].entities[entityIndex - 1] as _Pie;
        //   startAngle = prevPie.endAngle;
        // }
        // return _Pie()
        //   ..index = entityIndex
        //   ..value = value
        //   ..formattedValue = value != null ? entity_value_formatter(value) : null
        //   ..name = name
        //   ..color = color
        //   ..highlightColor = highlightColor
        //   ..oldStartAngle = startAngle
        //   ..oldEndAngle = startAngle
        //   ..center = _center
        //   ..innerRadius = _innerRadius
        //   ..outerRadius = _outerRadius
        //   ..startAngle = startAngle
        //   ..endAngle = startAngle; // To be updated in [update_series].
        unimplemented!()
    }

    fn get_tooltip_position(&self) -> Point<f64> {
        // let pie = series_list.first.entities[focused_entity_index] as _Pie;
        // let angle = .5 * (pie.startAngle + pie.endAngle);
        // let radius = .5 * (_innerRadius + _outerRadius);
        // let point = polarToCartesian(_center, radius, angle);
        // let x = point.x - .5 * _tooltip.offsetWidth;
        // let y = point.y - _tooltip.offsetHeight;
        // return Point(x, y);
        unimplemented!()
    }
}
