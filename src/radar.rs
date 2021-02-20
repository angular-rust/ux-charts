#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;

use crate::{CanvasContext, Chart, Drawable, Entity, Point, Rectangle};

// let _radarChartDefaultOptions = {
//   // Map - An object that controls the series.
//   "series": {
//     // num - The opacity of the area between a series and the x-axis.
//     "fillOpacity": .25,

//     // num - The line width of the series.
//     "lineWidth": 2,

//     // Map - An object that controls the series labels.
//     "labels": {
//       // bool - Whether to show the labels.
//       "enabled": false,
//       "style": {
//         "color": "#212121",
//         "fontFamily": _fontFamily,
//         "fontSize": 13,
//         "fontStyle": "normal"
//       }
//     },

//     // Map - An object that controls the markers.
//     "markers": {
//       // bool - Whether markers are enabled.
//       "enabled": true,

//       // String - The fill color. If `null`, the stroke color of the series
//       // will be used.
//       "fillColor": null,

//       // num - The line width of the markers.
//       "lineWidth": 1,

//       // String - The stroke color. If `null`, the stroke color of the series
//       // will be used.
//       "strokeColor": "white",

//       // num - Size of the markers. To disable markers, set this to zero.
//       "size": 4
//     }
//   },

//   // Map - An object that controls the x-axis.
//   "xAxis": {
//     // String - The color of the horizontal grid lines.
//     "gridLineColor": "#c0c0c0",

//     // num - The width of the horizontal grid lines.
//     "gridLineWidth": 1,

//     // Map - An object that controls the axis labels.
//     "labels": {
//       "style": {
//         // String - The labels" color.
//         "color": "#212121",

//         // String - The labels" font family.
//         "fontFamily": _fontFamily,

//         // String - The labels" font size.
//         "fontSize": 13,

//         // String - The labels" font style.
//         "fontStyle": "normal"
//       }
//     },
//   },

//   // Map - An object that controls the y-axis.
//   "yAxis": {
//     // String - The color of the vertical grid lines.
//     "gridLineColor": "#c0c0c0",

//     // num - The width of the vertical grid lines.
//     "gridLineWidth": 1,

//     // num - The interval of the tick marks in axis unit. If `null`, this value
//     // is automatically calculated.
//     "interval": null,

//     // Map - An object that controls the axis labels.
//     "labels": {
//       // (num value) -> String - A function that formats the labels.
//       "formatter": null,

//       // Map - An object that controls the styling of the axis labels.
//       "style": {
//         // String - The labels" color.
//         "color": "#212121",

//         // String - The labels" font family.
//         "fontFamily": _fontFamily,

//         // String - The labels" font size.
//         "fontSize": 13,

//         // String - The labels" font style.
//         "fontStyle": "normal"
//       }
//     },

//     // num - The minimum interval. If `null`, this value is automatically
//     // calculated.
//     "minInterval": null,
//   }
// };

pub struct PolarPoint {
    old_radius: f64,
    old_angle: f64,
    old_point_radius: f64,

    radius: f64,
    angle: f64,
    point_radius: f64,

    center: Point,
}

impl<C> Drawable<C> for PolarPoint
where
    C: CanvasContext,
{
    fn draw(ctx: C, percent: f64, highlight: bool) {
        // let r = lerp(oldRadius, radius, percent);
        // let a = lerp(oldAngle, angle, percent);
        // let pr = lerp(oldPointRadius, pointRadius, percent);
        // let p = polarToCartesian(center, r, a);
        // if (highlight) {
        //   ctx.fillStyle = highlightColor;
        //   ctx.beginPath();
        //   ctx.arc(p.x, p.y, 2 * pr, 0, _2pi);
        //   ctx.fill();
        // }
        // ctx.fillStyle = color;
        // ctx.beginPath();
        // ctx.arc(p.x, p.y, pr, 0, _2pi);
        // ctx.fill();
        // ctx.stroke();
        unimplemented!()
    }
}

impl Entity for PolarPoint {
    fn save() {
        // oldRadius = radius;
        // oldAngle = angle;
        // oldPointRadius = pointRadius;
        // super.save();
        unimplemented!()
    }
}

pub struct RadarChart {
    center: Point,
    radius: f64,
    angle_interval: f64,
    x_labels: Vec<String>,
    y_labels: Vec<String>,
    y_max_malue: f64,
    y_label_hop: f64,
    // yLabelFormatter: ValueFormatter,
    /// Each element is the bounding box of each entity group.
    /// A `null` element means the group has no visible entities.
    bounding_boxes: Vec<Rectangle>,
}

impl RadarChart {
    fn new() {
        // : super(container)
        // _defaultOptions = mergeMaps(globalOptions, _radarChartDefaultOptions);
    }

    // num _getAngle(i64 entityIndex) => entityIndex * _angleInterval - _pi_2;

    // num _valueToRadius(num value) =>
    //     (value != null) ? value * _radius / _yMaxValue : 0.0;

    fn calculate_bounding_boxes() {
        // if (!_options["tooltip"]["enabled"]) return;

        // let seriesCount = _seriesList.length;
        // let entityCount = _seriesList.first.entities.length;
        // _boundingBoxes = Vec<Rectangle>(entityCount);
        // for (let i = 0; i < entityCount; i++) {
        //   let minX = double.maxFinite;
        //   let minY = double.maxFinite;
        //   let maxX = -double.maxFinite;
        //   let maxY = -double.maxFinite;
        //   let count = 0;
        //   for (let j = 0; j < seriesCount; j++) {
        //     if (_seriesStates[j] == _VisibilityState.hidden) continue;
        //     if (_seriesStates[j] == _VisibilityState.hiding) continue;

        //     let pp = _seriesList[j].entities[i] as _PolarPoint;
        //     if (pp.value == null) continue;

        //     let cp = polarToCartesian(pp.center, pp.radius, pp.angle);
        //     minX = min(minX, cp.x);
        //     minY = min(minY, cp.y);
        //     maxX = max(maxX, cp.x);
        //     maxY = max(maxY, cp.y);
        //     count++;
        //   }
        //   _boundingBoxes[i] =
        //       count > 0 ? Rectangle(minX, minY, maxX - minX, maxY - minY) : null;
        // }
        unimplemented!()
    }

    // fn drawText(ctx: C, text: String, radius: f64, angle: f64, fontSize: f64) {
    //     // let w = ctx.measureText(text).width;
    //     // let x = _center.x + cos(angle) * (radius + .5 * w);
    //     // let y = _center.y + sin(angle) * (radius + .5 * fontSize);
    //     // ctx.fillText(text, x, y);
    // }

    fn get_entity_group_index(x: f64, y: f64) -> i64 {
        // let p = Point(x - _center.x, y - _center.y);
        // if (p.magnitude >= _radius) return -1;
        // let angle = atan2(p.y, p.x);
        // let points = _seriesList.first.entities.cast<_PolarPoint>();
        // for (let i = points.length - 1; i >= 0; i--) {
        //   if (_boundingBoxes[i] == null) continue;

        //   let delta = angle - points[i].angle;
        //   if (delta.abs() < .5 * _angleInterval) return i;
        //   if ((delta + _2pi).abs() < .5 * _angleInterval) return i;
        // }
        // return -1;
        unimplemented!()
    }

    fn get_tooltip_position() -> Point {
        // let box = _boundingBoxes[_focusedEntityIndex];
        // let offset = _options["series"]["markers"]["size"] * 2 + 5;
        // let x = box.right + offset;
        // let y = box.top + (box.height - _tooltip.offsetHeight) ~/ 2;
        // if (x + _tooltip.offsetWidth > _width)
        //   x = box.left - _tooltip.offsetWidth - offset;
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
    //     // let angle = _getAngle(entityIndex);
    //     // return _PolarPoint()
    //     //   ..index = entityIndex
    //     //   ..value = value
    //     //   ..color = color
    //     //   ..highlightColor = highlightColor
    //     //   ..center = _center
    //     //   ..oldRadius = 0
    //     //   ..oldAngle = angle
    //     //   ..oldPointRadius = 0
    //     //   ..radius = _valueToRadius(value)
    //     //   ..angle = angle
    //     //   ..pointRadius = _options["series"]["markers"]["size"];
    // }

    fn series_visibility_changed(index: usize) {
        // let visible = _seriesStates[index].index >= _VisibilityState.showing.index;
        // let markerSize = _options["series"]["markers"]["size"];
        // for (_PolarPoint p in _seriesList[index].entities) {
        //   if (visible) {
        //     p.radius = _valueToRadius(p.value);
        //     p.pointRadius = markerSize;
        //   } else {
        //     p.radius = 0.0;
        //     p.pointRadius = 0;
        //   }
        // }

        // _calculateBoundingBoxes();
    }

    fn update(options: HashMap<String, String>) {
        // super.update(options);
        // _calculateBoundingBoxes();
        unimplemented!()
    }
}

impl Chart for RadarChart {
    fn calculate_drawing_sizes() {
        // super._calculateDrawingSizes();

        // _xLabels = _dataTable.getColumnValues<String>(0);
        // _angleInterval = _2pi / _xLabels.length;

        // let rect = _seriesAndAxesBox;
        // let xLabelFontSize = _options["xAxis"]["labels"]["style"]["fontSize"];

        // // [_radius]*factor equals the height of the largest polygon.
        // let factor = 1 + sin((_xLabels.length >> 1) * _angleInterval - _pi_2);
        // _radius = min(rect.width, rect.height) / factor -
        //     factor * (xLabelFontSize + _axisLabelMargin);
        // _center =
        //     Point(rect.left + rect.width / 2, rect.top + rect.height / factor);

        // // The minimum value on the y-axis is always zero.
        // let yInterval = _options["yAxis"]["interval"];
        // if (yInterval == null) {
        //   let yMinInterval = _options["yAxis"]["minInterval"];
        //   _yMaxValue = findMaxValue(_dataTable);
        //   yInterval = calculateInterval(_yMaxValue, 3, yMinInterval);
        //   _yMaxValue = (_yMaxValue / yInterval).ceilToDouble() * yInterval;
        // }

        // _yLabelFormatter = _options["yAxis"]["labels"]["formatter"];
        // if (_yLabelFormatter == null) {
        //   let decimalPlaces = getDecimalPlaces(yInterval);
        //   let numberFormat = NumberFormat.decimalPattern()
        //     ..maximumFractionDigits = decimalPlaces
        //     ..minimumFractionDigits = decimalPlaces;
        //   _yLabelFormatter = numberFormat.format;
        // }
        // _entityValueFormatter = _yLabelFormatter;

        // _yLabels = <String>[];
        // let value = 0.0;
        // while (value <= _yMaxValue) {
        //   _yLabels.add(_yLabelFormatter(value));
        //   value += yInterval;
        // }

        // _yLabelHop = _radius / (_yLabels.length - 1);

        // // Tooltip.

        // _tooltipValueFormatter =
        //     _options["tooltip"]["valueFormatter"] ?? _yLabelFormatter;
        unimplemented!()
    }

    fn draw_axes_and_grid() {
        // let xLabelCount = _xLabels.length;
        // let yLabelCount = _yLabels.length;

        // // x-axis grid lines (i.e. concentric equilateral polygons).

        // let lineWidth = _options["xAxis"]["gridLineWidth"];
        // if (lineWidth > 0) {
        //   _axesContext
        //     ..lineWidth = lineWidth
        //     ..strokeStyle = _options["xAxis"]["gridLineColor"]
        //     ..beginPath();
        //   let radius = _radius;
        //   for (let i = yLabelCount - 1; i >= 1; i--) {
        //     let angle = -_pi_2 + _angleInterval;
        //     _axesContext.moveTo(_center.x, _center.y - radius);
        //     for (let j = 0; j < xLabelCount; j++) {
        //       let poi64 = polarToCartesian(_center, radius, angle);
        //       _axesContext.lineTo(point.x, point.y);
        //       angle += _angleInterval;
        //     }
        //     radius -= _yLabelHop;
        //   }
        //   _axesContext.stroke();
        // }

        // // y-axis grid lines (i.e. radii from the center to the x-axis labels).

        // lineWidth = _options["yAxis"]["gridLineWidth"];
        // if (lineWidth > 0) {
        //   _axesContext
        //     ..lineWidth = lineWidth
        //     ..strokeStyle = _options["yAxis"]["gridLineColor"]
        //     ..beginPath();
        //   let angle = -_pi_2;
        //   for (let i = 0; i < xLabelCount; i++) {
        //     let poi64 = polarToCartesian(_center, _radius, angle);
        //     _axesContext
        //       ..moveTo(_center.x, _center.y)
        //       ..lineTo(point.x, point.y);
        //     angle += _angleInterval;
        //   }
        //   _axesContext.stroke();
        // }

        // // y-axis labels - don"t draw the first (at center) and the last ones.

        // let style = _options["yAxis"]["labels"]["style"];
        // let x = _center.x - _axisLabelMargin;
        // let y = _center.y - _yLabelHop;
        // _axesContext
        //   ..fillStyle = style["color"]
        //   ..font = _getFont(style)
        //   ..textAlign = "right"
        //   ..textBaseline = "middle";
        // for (let i = 1; i <= yLabelCount - 2; i++) {
        //   _axesContext.fillText(_yLabels[i], x, y);
        //   y -= _yLabelHop;
        // }

        // // x-axis labels.

        // style = _options["xAxis"]["labels"]["style"];
        // _axesContext
        //   ..fillStyle = style["color"]
        //   ..font = _getFont(style)
        //   ..textAlign = "center"
        //   ..textBaseline = "middle";
        // let fontSize = style["fontSize"];
        // let angle = -_pi_2;
        // let radius = _radius + _axisLabelMargin;
        // for (let i = 0; i < xLabelCount; i++) {
        //   _drawText(_axesContext, _xLabels[i], radius, angle, fontSize);
        //   angle += _angleInterval;
        // }
        unimplemented!()
    }

    fn draw_series(percent: f64) -> bool {
        // let fillOpacity = _options["series"]["fillOpacity"];
        // let seriesLineWidth = _options["series"]["lineWidth"];
        // let markerOptions = _options["series"]["markers"];
        // let markerSize = markerOptions["size"];
        // let pointCount = _xLabels.length;

        // for (let i = 0; i < _seriesList.length; i++) {
        //   if (_seriesStates[i] == _VisibilityState.hidden) continue;

        //   let series = _seriesList[i];
        //   let scale = (i != _focusedSeriesIndex) ? 1 : 2;

        //   // Draw the polygon.

        //   _seriesContext
        //     ..lineWidth = scale * seriesLineWidth
        //     ..strokeStyle = series.color
        //     ..beginPath();
        //   for (let j = 0; j < pointCount; j++) {
        //     let poi64 = series.entities[j] as _PolarPoint;
        //     // TODO: Optimize.
        //     let radius = lerp(point.oldRadius, point.radius, percent);
        //     let angle = lerp(point.oldAngle, point.angle, percent);
        //     let p = polarToCartesian(_center, radius, angle);
        //     if (j > 0) {
        //       _seriesContext.lineTo(p.x, p.y);
        //     } else {
        //       _seriesContext.moveTo(p.x, p.y);
        //     }
        //   }
        //   _seriesContext.closePath();
        //   _seriesContext.stroke();

        //   // Optionally fill the polygon.

        //   if (fillOpacity > 0) {
        //     _seriesContext.fillStyle = _changeColorAlpha(series.color, fillOpacity);
        //     _seriesContext.fill();
        //   }

        //   // Draw the markers.

        //   if (markerSize > 0) {
        //     let fillColor = markerOptions["fillColor"] ?? series.color;
        //     let strokeColor = markerOptions["strokeColor"] ?? series.color;
        //     _seriesContext
        //       ..fillStyle = fillColor
        //       ..lineWidth = scale * markerOptions["lineWidth"]
        //       ..strokeStyle = strokeColor;
        //     for (let p in series.entities) {
        //       if (markerOptions["enabled"]) {
        //         p.draw(_seriesContext, percent, p.index == _focusedEntityIndex);
        //       } else if (p.index == _focusedEntityIndex) {
        //         // Only draw marker on hover.
        //         p.draw(_seriesContext, percent, true);
        //       }
        //     }
        //   }
        // }

        // return false;
        unimplemented!()
    }

    // param should be Option
    fn update_series(index: usize) {
        // let entityCount = _dataTable.rows.length;
        // for (let i = 0; i < _seriesList.length; i++) {
        //   let series = _seriesList[i];
        //   let color = _getColor(i);
        //   let highlightColor = _getHighlightColor(color);
        //   let visible = _seriesStates[i].index >= _VisibilityState.showing.index;
        //   series.color = color;
        //   series.highlightColor = highlightColor;
        //   for (let j = 0; j < entityCount; j++) {
        //     let p = series.entities[j] as _PolarPoint;
        //     p.index = j;
        //     p.center = _center;
        //     p.radius = visible ? _valueToRadius(p.value) : 0.0;
        //     p.angle = _getAngle(j);
        //     p.color = color;
        //     p.highlightColor = highlightColor;
        //   }
        // }
    }
}
