#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use crate::{CanvasContext, Chart, Drawable, Entity, TwoAxisChart};

// let _lineChartDefaultOptions = {
//   // Map - An object that controls the series.
//   "series": {
//     // num - The curve tension. The typical value is from 0.3 to 0.5.
//     // To draw straight lines, set this to zero.
//     "curveTension": .4,

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

//       // num - Size of the markers.
//       "size": 4
//     }
//   },

//   // Map - An object that controls the x-axis.
//   "xAxis": {
//     // String - The color of the horizontal grid lines.
//     "gridLineColor": "#c0c0c0",

//     // num - The width of the horizontal grid lines.
//     "gridLineWidth": 1,

//     // String - The color of the axis itself.
//     "lineColor": "#c0c0c0",

//     // num - The width of the axis itself.
//     "lineWidth": 1,

//     // Map - An object that controls the axis labels.
//     "labels": {
//       // num - The maximum rotation angle in degrees. Must be <= 90.
//       "maxRotation": 0,

//       // num - The minimum rotation angle in degrees. Must be >= -90.
//       "minRotation": -90,

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

//     // String - The position of the axis relative to the chart area.
//     // Supported values: "bottom".
//     "position": "bottom",

//     // Map - An object that controls the axis title.
//     "title": {
//       // Map - An object that controls the styling of the axis title.
//       "style": {
//         // String - The title"s color.
//         "color": "#212121",

//         // String - The title"s font family.
//         "fontFamily": _fontFamily,

//         // String - The title"s font size.
//         "fontSize": 15,

//         // String - The title"s font style.
//         "fontStyle": "normal"
//       },

//       // The title text. A `null` value means the title is hidden.
//       "text": null
//     }
//   },

//   // Map - An object that controls the y-axis.
//   "yAxis": {
//     // String - The color of the vertical grid lines.
//     "gridLineColor": "#c0c0c0",

//     // num - The width of the vertical grid lines.
//     "gridLineWidth": 0,

//     // String - The color of the axis itself.
//     "lineColor": "#c0c0c0",

//     // num - The width of the axis itself.
//     "lineWidth": 0,

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

//     // num - The desired maximum value on the axis. If set, the calculated value
//     // is guaranteed to be >= this value.
//     "maxValue": null,

//     // num - The minimum interval. If `null`, this value is automatically
//     // calculated.
//     "minInterval": null,

//     // num - The desired minimum value on the axis. If set, the calculated value
//     // is guaranteed to be <= this value.
//     "minValue": null,

//     // String - The position of the axis relative to the chart area.
//     // Supported values: "left".
//     "position": "left",

//     // Map - An object that controls the axis title.
//     "title": {
//       // Map - An object that controls the styling of the axis title.
//       "style": {
//         // String - The title"s color.
//         "color": "#212121",

//         // String - The title"s font family.
//         "fontFamily": _fontFamily,

//         // String - The title"s font size.
//         "fontSize": 15,

//         // String - The title"s font style.
//         "fontStyle": "normal"
//       },

//       // The title text. A `null` value means the title is hidden.
//       "text": null
//     }
//   }
// };

struct Point {
    old_x: f64,
    old_y: f64,
    // oldCp1: Point,
    // oldCp2: Point,
    old_point_radius: f64,

    // /// The first control point.
    // cp1: Point,

    // /// The second control point.
    // cp2: Point,

    x: f64,

    y: f64,

    point_radius: f64,
}

/// A poi64 in a line chart.
impl<C> Drawable<C> for Point
where
    C: CanvasContext,
{
    fn draw(ctx: C, percent: f64, highlight: bool) {
        // let cx = lerp(oldX, x, percent);
        // let cy = lerp(oldY, y, percent);
        // let pr = lerp(oldPointRadius, pointRadius, percent);
        // if (highlight) {
        //   ctx.fillStyle = highlightColor;
        //   ctx.beginPath();
        //   ctx.arc(cx, cy, 2 * pr, 0, _2pi);
        //   ctx.fill();
        // }
        // ctx.beginPath();
        // ctx.arc(cx, cy, pr, 0, _2pi);
        // ctx.fill();
        // ctx.stroke();
        unimplemented!()
    }
}

impl Entity for Point {
    fn save() {
        // oldX = x;
        // oldY = y;
        // oldCp1 = cp1;
        // oldCp2 = cp2;
        // oldPointRadius = pointRadius;
        // super.save();
        unimplemented!()
    }

    // Point get asPoint => Point(x, y);
}

// extends TwoAxisChart
pub struct LineChart {}

impl LineChart {
    pub fn new() {
        // : super(container)
        // _defaultOptions = mergeMaps(globalOptions, _lineChartDefaultOptions);
    }

    fn calculate_average_y_values(index: usize) {
        // if (!_options["tooltip"]["enabled"]) return;

        // let entityCount = _dataTable.rows.length;
        // let start = index ?? 0;
        // let end = index == null ? entityCount : index + 1;

        // _averageYValues ??= <num>[];
        // _averageYValues.length = entityCount;

        // for (let i = start; i < end; i++) {
        //   let sum = 0.0;
        //   let count = 0;
        //   for (let j = _seriesList.length - 1; j >= 0; j--) {
        //     if (_seriesStates[j].index <= _VisibilityState.hiding.index) continue;
        //     let poi64 = _seriesList[j].entities[i] as _Point;
        //     if (point.value != null) {
        //       sum += point.y;
        //       count++;
        //     }
        //   }
        //   _averageYValues[i] = (count > 0) ? sum / count : null;
        // }
    }

    fn lerp_points(points: Vec<Point>, percent: f64) -> Vec<Point> {
        // return points.map((p) {
        //   let x = lerp(p.oldX, p.x, percent);
        //   let y = lerp(p.oldY, p.y, percent);
        //   let cp1 = (p.cp1 != null) ? lerp(p.oldCp1, p.cp1, percent) : null;
        //   let cp2 = (p.cp2 != null) ? lerp(p.oldCp2, p.cp2, percent) : null;
        //   return _Point()
        //     ..index = p.index
        //     ..value = p.value
        //     ..color = p.color
        //     ..highlightColor = p.highlightColor
        //     ..oldPointRadius = p.oldPointRadius
        //     ..oldX = p.oldX
        //     ..oldY = p.oldY
        //     ..pointRadius = p.pointRadius
        //     ..x = x
        //     ..y = y
        //     ..cp1 = cp1
        //     ..cp2 = cp2;
        // }).toList();
        unimplemented!()
    }

    // fn create_entity(
    //     seriesIndex: usize,
    //     entityIndex: usize,
    //     value: String,
    //     color: String,
    //     highlightColor: String,
    // ) -> Entity {
    //     // let x = _xLabelX(entityIndex);
    //     // let oldY = _xAxisTop;
    //     // // oldCp1 and oldCp2 are calculated in [_updateSeries].
    //     // return _Point()
    //     //   ..index = entityIndex
    //     //   ..value = value
    //     //   ..formattedValue = value != null ? _entityValueFormatter(value) : null
    //     //   ..color = color
    //     //   ..highlightColor = highlightColor
    //     //   ..oldX = x
    //     //   ..oldY = oldY
    //     //   ..oldPointRadius = 09
    //     //   ..x = x
    //     //   ..y = _valueToY(value)
    //     //   ..pointRadius = _options["series"]["markers"]["size"];
    //     unimplemented!()
    // }

    fn series_visibility_changed(index: usize) {
        // _updateSeries(index);
        // _calculateAverageYValues();
        unimplemented!()
    }
}

impl Chart for LineChart {
    // let num _xLabelOffsetFactor = 0;

    fn calculate_drawing_sizes() {
        // super._calculateDrawingSizes();
        // _tooltipOffset = _options["series"]["markers"]["size"] * 2 + 5;
    }

    fn draw_series(percent: f64) -> bool {
        //   fn curveTo(Point cp1, Point cp2, _Point p) {
        //     if cp2 == null && cp1 == null {
        //       _seriesContext.lineTo(p.x, p.y);
        //     } else if cp2 == null {
        //       _seriesContext.quadraticCurveTo(cp1.x, cp1.y, p.x, p.y);
        //     } else if cp1 == null {
        //       _seriesContext.quadraticCurveTo(cp2.x, cp2.y, p.x, p.y);
        //     } else {
        //       _seriesContext.bezierCurveTo(cp1.x, cp1.y, cp2.x, cp2.y, p.x, p.y);
        //     }
        //   }

        //   let seriesCount = _seriesList.length;
        //   let entityCount = _dataTable.rows.length;
        //   let fillOpacity = _options["series"]["fillOpacity"];
        //   let seriesLineWidth = _options["series"]["lineWidth"];
        //   let markerOptions = _options["series"]["markers"];
        //   let markerSize = markerOptions["size"];

        //   for (let i = 0; i < seriesCount; i++) {
        //     if (_seriesStates[i] == _VisibilityState.hidden) continue;

        //     let series = _seriesList[i];
        //     let points = _lerpPoints(series.entities.cast<_Point>(), percent);
        //     let scale = (i != _focusedSeriesIndex) ? 1 : 2;

        //     _seriesContext.lineJoin = "round";

        //     // Draw series with filling.

        //     if (fillOpacity > 0.0) {
        //       let color = _changeColorAlpha(series.color, fillOpacity);
        //       _seriesContext.fillStyle = color;
        //       _seriesContext.strokeStyle = color;
        //       let j = 0;
        //       while (true) {
        //         // Skip points with a null value.
        //         while (j < entityCount && points[j].value == null) j++;

        //         // Stop if we have reached the end of the series.
        //         if (j == entityCount) break;

        //         // Connect a series of contiguous points with a non-null value and
        //         // fill the area between them and the x-axis.
        //         let p = points[j];
        //         _seriesContext
        //           ..beginPath()
        //           ..moveTo(p.x, _xAxisTop)
        //           ..lineTo(p.x, p.y);
        //         let lastPoint = p;
        //         let count = 1;
        //         while (++j < entityCount && points[j].value != null) {
        //           p = points[j];
        //           curveTo(lastPoint.cp2, p.cp1, p);
        //           lastPoint = p;
        //           count++;
        //         }
        //         if (count >= 2) {
        //           _seriesContext
        //             ..lineTo(lastPoint.x, _xAxisTop)
        //             ..closePath()
        //             ..fill();
        //         }
        //       }
        //     }

        //     // Draw series without filling.

        //     if (seriesLineWidth > 0) {
        //       let lastPoint = _Point();
        //       _seriesContext
        //         ..lineWidth = scale * seriesLineWidth
        //         ..strokeStyle = series.color
        //         ..beginPath();
        //       for (let p in points) {
        //         if (p.value != null) {
        //           if (lastPoint.value != null) {
        //             curveTo(lastPoint.cp2, p.cp1, p);
        //           } else {
        //             _seriesContext.moveTo(p.x, p.y);
        //           }
        //         }
        //         lastPoint = p;
        //       }
        //       _seriesContext.stroke();
        //     }

        //     // Draw markers.

        //     if (markerSize > 0) {
        //       let fillColor = markerOptions["fillColor"] ?? series.color;
        //       let strokeColor = markerOptions["strokeColor"] ?? series.color;
        //       _seriesContext
        //         ..fillStyle = fillColor
        //         ..lineWidth = scale * markerOptions["lineWidth"]
        //         ..strokeStyle = strokeColor;
        //       for (let p in points) {
        //         if (p.value != null) {
        //           if (markerOptions["enabled"]) {
        //             p.draw(_seriesContext, 1.0, p.index == _focusedEntityIndex);
        //           } else if (p.index == _focusedEntityIndex) {
        //             // Only draw marker on hover.
        //             p.draw(_seriesContext, 1.0, true);
        //           }
        //         }
        //       }
        //     }
        //   }

        //   // Draw labels only on the last frame.

        //   let labelOptions = _options["series"]["labels"];
        //   if (percent == 1.0 && labelOptions["enabled"]) {
        //     _seriesContext
        //       ..fillStyle = labelOptions["style"]["color"]
        //       ..font = _getFont(labelOptions["style"])
        //       ..textAlign = "center"
        //       ..textBaseline = "alphabetic";
        //     for (let i = 0; i < seriesCount; i++) {
        //       if (_seriesStates[i] != _VisibilityState.shown) continue;

        //       let points = _seriesList[i].entities;
        //       for (_Point p in points) {
        //         if (p.value != null) {
        //           let y = p.y - markerSize - 5;
        //           _seriesContext.fillText(p.formattedValue, p.x, y);
        //         }
        //       }
        //     }
        //   }

        false
    }

    fn update_series(index: usize) {
        // let entityCount = _dataTable.rows.length;
        // let markerSize = _options["series"]["markers"]["size"];
        // let curveTension = _options["series"]["curveTension"];
        // let curve = curveTension > 0 && entityCount > 2;

        // let start = index ?? 0;
        // let end = (index == null) ? _seriesList.length : index + 1;
        // for (let i = start; i < end; i++) {
        //   let visible = _seriesStates[i].index >= _VisibilityState.showing.index;
        //   let series = _seriesList[i];
        //   let entities = series.entities;
        //   let color = _getColor(i);
        //   let highlightColor = _getHighlightColor(color);
        //   series.color = color;
        //   series.highlightColor = highlightColor;

        //   for (let j = 0; j < entityCount; j++) {
        //     let e = entities[j] as _Point;
        //     e.index = j;
        //     e.color = color;
        //     e.highlightColor = highlightColor;
        //     e.x = _xLabelX(j);
        //     e.y = visible ? _valueToY(e.value) : _xAxisTop;
        //     e.pointRadius = visible ? markerSize : 0;
        //   }

        //   if (!curve) continue;

        //   let e1;
        //   let e2 = entities[0] as _Point;
        //   let e3 = entities[1] as _Point;
        //   for (let j = 2; j < entityCount; j++) {
        //     e1 = e2;
        //     e2 = e3;
        //     e3 = entities[j];
        //     if (e1.value == null) continue;
        //     if (e2.value == null) continue;
        //     if (e3.value == null) continue;
        //     let list = calculateControlPoints(
        //         e1.asPoint, e2.asPoint, e3.asPoint, curveTension);
        //     e2.cp1 = list[0];
        //     e2.cp2 = list[1];
        //     e2.oldCp1 ??= Point(e2.cp1.x, _xAxisTop);
        //     e2.oldCp2 ??= Point(e2.cp2.x, _xAxisTop);
        //   }
        // }
        unimplemented!()
    }
}
