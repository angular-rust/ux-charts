#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use crate::{CanvasContext, Chart, Drawable, Entity, Point};

// let _gaugeChartDefaultOptions = {
//   // String - The background color of the gauges.
//   "gaugeBackgroundColor": "#dbdbdb",

//   // Map - An object that controls the gauge labels.
//   "gaugeLabels": {
//     // bool - Whether to show the labels.
//     "enabled": true,

//     // Map - An object that controls the styling of the gauge labels.
//     "style": {
//       "color": "#212121",
//       "fontFamily": _fontFamily,
//       "fontSize": 13,
//       "fontStyle": "normal"
//     }
//   }
// };

// Gauge extends Pie
struct Gauge {
    background_color: String,
}

impl<C> Drawable<C> for Gauge
where
    C: CanvasContext,
{
    fn draw(ctx: C, percent: f64, highlight: bool) {
        // let tmpColor = color;
        // let tmpEndAngle = endAngle;

        // // Draw the background.

        // endAngle = startAngle + _2pi;
        // color = backgroundColor;
        // super.draw(ctx, 1.0, false);

        // // Draw the foreground.

        // color = tmpColor;
        // endAngle = tmpEndAngle;
        // super.draw(ctx, percent, highlight);

        // // Draw the percent.

        // let fs1 = .75 * innerRadius;
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
        // ctx
        //   ..font = font1
        //   ..fillText(text1, center.x - .5 * w2, y)
        //   ..font = font2
        //   ..fillText(text2, center.x + .5 * w1, y);
    }
}

struct GaugeChart {
    gauge_hop: f64,
    gauge_inner_radius: f64,
    gauge_outer_radius: f64,
    gauge_center_y: f64,
    start_angle: f64, // = -_pi_2;
}

impl GaugeChart {
    fn get_gauge_center(index: i64) -> Point {
        // Point((index + 0.5) * _gaugeHop, _gaugeCenterY)
        unimplemented!()
    }

    fn value_to_angle(value: f64) -> f64 {
        // value * _2pi / 100
        unimplemented!()
    }

    // fn create_entity(
    //     seriesIndex: i64,
    //     entityIndex: i64,
    //     value: i64,
    //     color: String,
    //     highlightColor: String,
    // ) -> Entity {
    //     // // Override the colors.
    //     // color = _getColor(entityIndex);
    //     // highlightColor = _changeColorAlpha(color, .5);

    //     // let name = _dataTable.rows[entityIndex][0];
    //     // return _Gauge()
    //     //   ..index = entityIndex
    //     //   ..value = value
    //     //   ..name = name
    //     //   ..color = color
    //     //   ..backgroundColor = _options["gaugeBackgroundColor"]
    //     //   ..highlightColor = highlightColor
    //     //   ..oldValue = 0
    //     //   ..oldStartAngle = _startAngle
    //     //   ..oldEndAngle = _startAngle
    //     //   ..center = _getGaugeCenter(entityIndex)
    //     //   ..innerRadius = _gaugeInnerRadius
    //     //   ..outerRadius = _gaugeOuterRadius
    //     //   ..startAngle = _startAngle
    //     //   ..endAngle = _startAngle + _valueToAngle(value);
    //     unimplemented!()
    // }

    fn update_tooltip_content() {
        // let gauge = _seriesList[0].entities[_focusedEntityIndex] as _Gauge;
        // _tooltip.style
        //   ..borderColor = gauge.color
        //   ..padding = "4px 12px";
        // let label = _tooltipLabelFormatter(gauge.name);
        // let value = _tooltipValueFormatter(gauge.value);
        // _tooltip.innerHtml = "$label: <strong>$value%</strong>";
    }

    fn get_entity_group_index(x: f64, y: f64) -> i64 {
        // let p = Point(x, y);
        // for (_Gauge g in _seriesList[0].entities) {
        //   if (g.containsPoint(p)) return g.index;
        // }
        // return -1;
        unimplemented!()
    }

    fn get_tooltip_position() -> Point {
        // let gauge = _seriesList[0].entities[_focusedEntityIndex] as _Gauge;
        // let x = gauge.center.x - _tooltip.offsetWidth ~/ 2;
        // let y = gauge.center.y -
        //     _highlightOuterRadiusFactor * gauge.outerRadius -
        //     _tooltip.offsetHeight -
        //     5;
        // return Point(x, y);
        unimplemented!()
    }

    fn new() {
        // super(container)
        // _defaultOptions = mergeMaps(globalOptions, _gaugeChartDefaultOptions);
        // _defaultOptions["legend"]["position"] = "none";
    }
}

impl Chart for GaugeChart {

    fn calculate_drawing_sizes() {
        // super._calculateDrawingSizes();

        // let gaugeCount = _dataTable.rows.length;
        // let labelTotalHeight = 0;
        // if (_options["gaugeLabels"]["enabled"]) {
        //   labelTotalHeight =
        //       _axisLabelMargin + _options["gaugeLabels"]["style"]["fontSize"];
        // }

        // _gaugeCenterY = _seriesAndAxesBox.top + .5 * _seriesAndAxesBox.height;
        // _gaugeHop = _seriesAndAxesBox.width / gaugeCount;

        // let availW = .618 * _gaugeHop; // Golden ratio.
        // let availH = _seriesAndAxesBox.height - 2 * labelTotalHeight;
        // _gaugeOuterRadius = .5 * min(availW, availH) / _highlightOuterRadiusFactor;
        // _gaugeInnerRadius = .5 * _gaugeOuterRadius;
    }

    fn draw_series(percent: f64) -> bool {
        // let style = _options["gaugeLabels"]["style"];
        // let labelsEnabled = _options["gaugeLabels"]["enabled"];
        // _seriesContext
        //   ..strokeStyle = "white"
        //   ..textAlign = "center";
        // for (_Gauge gauge in _seriesList[0].entities) {
        //   let highlight = gauge.index == _focusedEntityIndex;
        //   gauge.draw(_seriesContext, percent, highlight);

        //   if (!labelsEnabled) continue;

        //   let x = gauge.center.x;
        //   let y = gauge.center.y +
        //       gauge.outerRadius +
        //       style["fontSize"] +
        //       _axisLabelMargin;
        //   _seriesContext
        //     ..fillStyle = style["color"]
        //     ..font = _getFont(style)
        //     ..textAlign = "center"
        //     ..fillText(gauge.name, x, y);
        // }
        // return false;
        unimplemented!()
    }

    fn update_series(index: usize) {
        // let n = _dataTable.rows.length;
        // for (let i = 0; i < n; i++) {
        //   let gauge = _seriesList[0].entities[i] as _Gauge;
        //   let color = _getColor(i);
        //   let highlightColor = _changeColorAlpha(color, .5);
        //   gauge
        //     ..index = i
        //     ..name = _dataTable.rows[i][0]
        //     ..color = color
        //     ..highlightColor = highlightColor
        //     ..center = _getGaugeCenter(i)
        //     ..innerRadius = _gaugeInnerRadius
        //     ..outerRadius = _gaugeOuterRadius
        //     ..endAngle = _startAngle + _valueToAngle(gauge.value);
        // }
    }
}
