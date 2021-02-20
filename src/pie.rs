#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use crate::{CanvasContext, Chart, DataCollectionChangeRecord, Drawable, Entity, Point};

// let _pieChartDefaultOptions = {
//   // num - If between 0 and 1, displays a donut chart. The hole will have a
//   // radius equal to this value times the radius of the chart.
//   "pieHole": 0,

//   // Map - An object that controls the series.
//   "series": {
//     /// bool - Whether to draw the slices counterclockwise.
//     "counterclockwise": false,

//     // Map - An object that controls the series labels.
//     "labels": {
//       // bool - Whether to show the labels.
//       "enabled": false,

//       // (num) -> String - A function used to format the labels.
//       "formatter": null,

//       "style": {
//         "color": "white",
//         "fontFamily": _fontFamily,
//         "fontSize": 13,
//         "fontStyle": "normal"
//       },
//     },

//     // num - The start angle in degrees. Default is -90, which is 12 o"clock.
//     "startAngle": -90,
//   },
// };

const CLOCKWISE: i64 = 1;
const COUNTERCLOCKWISE: i64 = -1;
const HIGHLIGHT_OUTER_RADIUS_FACTOR: f64 = 1.05;

/// A pie in a pie chart.
pub struct Pie {
    old_start_angle: f64,
    old_end_angle: f64,
    start_angle: f64,
    end_angle: f64,

    center: Point,
    inner_radius: f64,
    outer_radius: f64,

    // [_Series] field.
    name: String,
}

impl Pie {
    // bool get isEmpty => startAngle == endAngle;

    fn contains_point(p: Point) -> bool {
        // p -= center;
        // let mag = p.magnitude;
        // if (mag > outerRadius || mag < innerRadius) {
        //   return false;
        // }

        // let angle = atan2(p.y, p.x);
        // let chartStartAngle = (chart as dynamic)._startAngle;

        // // Make sure [angle] is in range [chartStartAngle]..[chartStartAngle] + 2pi.
        // angle = (angle - chartStartAngle) % _2pi + chartStartAngle;

        // // If counterclockwise, make sure [angle] is in range
        // // [start] - 2*pi..[start].
        // if (startAngle > endAngle) angle -= _2pi;

        // if (startAngle <= endAngle) {
        //   // Clockwise.
        //   return isInRange(angle, startAngle, endAngle);
        // } else {
        //   // Counterclockwise.
        //   return isInRange(angle, endAngle, startAngle);
        // }
        unimplemented!()
    }

    fn save() {
        // oldStartAngle = startAngle;
        // oldEndAngle = endAngle;
        // super.save();
        unimplemented!()
    }
}

impl<C> Drawable<C> for Pie
where
    C: CanvasContext,
{
    fn draw(ctx: C, percent: f64, highlight: bool) {
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
        //   let options = chart._options["series"]["labels"];
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

pub struct PieChart {
    center: Point,
    outer_radius: f64,
    inner_radius: f64,

    /// The start angle in radians.
    start_angle: f64,

    /// 1 means clockwise and -1 means counterclockwise.
    direction: i64,
}

impl PieChart {
    pub fn new() {
        // : super(container)
        // _defaultOptions = mergeMaps(globalOptions, _pieChartDefaultOptions);
    }

    fn data_rows_changed(record: DataCollectionChangeRecord) {
        // _updateSeriesVisible(record.index, record.removedCount, record.addedCount);
        // super._dataRowsChanged(record);
        // _updateLegendContent();
        unimplemented!()
    }

    fn get_entity_group_index(x: f64, y: f64) -> i64 {
        // let p = Point(x, y);
        // let entities = _seriesList.first.entities;
        // for (let i = entities.length - 1; i >= 0; i--) {
        //   let pie = entities[i] as _Pie;
        //   if (pie.containsPoint(p)) return i;
        // }
        // return -1;
        unimplemented!()
    }

    // Vec<String> _getLegendLabels() => _dataTable.getColumnValues<String>(0);

    fn get_tooltip_position() -> Point {
        // let pie = _seriesList.first.entities[_focusedEntityIndex] as _Pie;
        // let angle = .5 * (pie.startAngle + pie.endAngle);
        // let radius = .5 * (_innerRadius + _outerRadius);
        // let poi64 = polarToCartesian(_center, radius, angle);
        // let x = point.x - .5 * _tooltip.offsetWidth;
        // let y = point.y - _tooltip.offsetHeight;
        // return Point(x, y);
        unimplemented!()
    }

    // fn create_entity(
    //     seriesIndex: usize,
    //     entityIndex: usize,
    //     value: String,
    //     color: String,
    //     highlightColor: String,
    // ) -> Entity {
    //     // // Override the colors.
    //     // color = _getColor(entityIndex);
    //     // highlightColor = _changeColorAlpha(color, .5);
    //     // let name = _dataTable.rows[entityIndex][0];
    //     // let startAngle = _startAngle;
    //     // if (entityIndex > 0 && _seriesList != null) {
    //     //   let prevPie = _seriesList[0].entities[entityIndex - 1] as _Pie;
    //     //   startAngle = prevPie.endAngle;
    //     // }
    //     // return _Pie()
    //     //   ..index = entityIndex
    //     //   ..value = value
    //     //   ..formattedValue = value != null ? _entityValueFormatter(value) : null
    //     //   ..name = name
    //     //   ..color = color
    //     //   ..highlightColor = highlightColor
    //     //   ..oldStartAngle = startAngle
    //     //   ..oldEndAngle = startAngle
    //     //   ..center = _center
    //     //   ..innerRadius = _innerRadius
    //     //   ..outerRadius = _outerRadius
    //     //   ..startAngle = startAngle
    //     //   ..endAngle = startAngle; // To be updated in [_updateSeries].
    //     unimplemented!()
    // }

    fn series_visibility_changed(index: usize) {
        // _updateSeries();
    }

    fn update_tooltip_content() {
        // let pie = _seriesList[0].entities[_focusedEntityIndex] as _Pie;
        // _tooltip.style
        //   ..borderColor = pie.color
        //   ..padding = "4px 12px";
        // let label = _tooltipLabelFormatter(pie.name);
        // let value = _tooltipValueFormatter(pie.value);
        // _tooltip.innerHtml = "$label: <strong>$value</strong>";
        unimplemented!()
    }
}

impl Chart for PieChart {
    fn calculate_drawing_sizes() {
        // super._calculateDrawingSizes();
        // let rect = _seriesAndAxesBox;
        // let halfW = rect.width >> 1;
        // let halfH = rect.height >> 1;
        // _center = Point(rect.left + halfW, rect.top + halfH);
        // _outerRadius = min(halfW, halfH) / _highlightOuterRadiusFactor;
        // let pieHole = _options["pieHole"];
        // if (pieHole > 1) pieHole = 0;
        // if (pieHole < 0) pieHole = 0;
        // _innerRadius = pieHole * _outerRadius;

        // let opt = _options["series"];
        // _entityValueFormatter =
        //     opt["labels"]["formatter"] ?? _defaultValueFormatter;
        // _direction = opt["counterclockwise"] ? _counterclockwise : _clockwise;
        // _startAngle = deg2rad(opt["startAngle"]);
        unimplemented!()
    }

    fn draw_series(percent: f64) -> bool {
        // _seriesContext
        //   ..lineWidth = 2
        //   ..strokeStyle = "#fff"
        //   ..textAlign = "center"
        //   ..textBaseline = "middle";
        // let pies = _seriesList.first.entities;
        // let labelOptions = _options["series"]["labels"];
        // _seriesContext.font = _getFont(labelOptions["style"]);
        // for (_Pie pie in pies) {
        //   if (pie.isEmpty && percent == 1.0) continue;
        //   let highlight =
        //       pie.index == _focusedSeriesIndex || pie.index == _focusedEntityIndex;
        //   pie.draw(_seriesContext, percent, highlight);
        // }

        // return false;
        unimplemented!()
    }

    fn update_series(index: usize) {
        // // Example data table:
        // //   Browser  Share
        // //   Chrome   .35
        // //   IE       .30
        // //   Firefox  .20
        // //   Other    .15

        // let sum = 0.0;
        // let startAngle = _startAngle;
        // let pieCount = _dataTable.rows.length;
        // let entities = _seriesList[0].entities;

        // // Sum the values of all visible pies.
        // for (let i = 0; i < pieCount; i++) {
        //   if (_seriesStates[i].index >= _VisibilityState.showing.index) {
        //     sum += entities[i].value;
        //   }
        // }

        // for (let i = 0; i < pieCount; i++) {
        //   _Pie pie = entities[i];
        //   let color = _getColor(i);
        //   pie.index = i;
        //   pie.name = _dataTable.rows[i][0];
        //   pie.color = color;
        //   pie.highlightColor = _getHighlightColor(color);
        //   pie.center = _center;
        //   pie.innerRadius = _innerRadius;
        //   pie.outerRadius = _outerRadius;

        //   if (_seriesStates[i].index >= _VisibilityState.showing.index) {
        //     pie.startAngle = startAngle;
        //     pie.endAngle = startAngle + _direction * pie.value * _2pi / sum;
        //     startAngle = pie.endAngle;
        //   } else {
        //     pie.startAngle = startAngle;
        //     pie.endAngle = startAngle;
        //   }
        // }
        unimplemented!()
    }
}
