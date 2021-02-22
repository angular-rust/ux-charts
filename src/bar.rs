#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use crate::{CanvasContext, Chart, Drawable, Entity, TwoAxisChart};

pub struct Bar {
    old_left: f64,
    old_width: f64,
    old_height: f64,
    bottom: f64,
    left: f64,
    width: f64,
    height: f64,
}

impl Bar {
    // num get right => left + width;
}

impl<C> Drawable<C> for Bar
where
    C: CanvasContext,
{
    fn draw(ctx: C, percent: f64, highlight: bool) {
        // let x = lerp(oldLeft, left, percent);
        // let h = lerp(oldHeight, height, percent);
        // let w = lerp(oldWidth, width, percent);
        // ctx.fillStyle = color;
        // ctx.fillRect(x, bottom - h, w, h);
        // if (highlight) {
        //   ctx.fillStyle = "rgba(255, 255, 255, .25)";
        //   ctx.fillRect(x, bottom - h, w, h);
        // }
        unimplemented!()
    }
}

impl Entity for Bar {
    fn free() {
        // chart = null;
    }

    fn save() {
        // oldLeft = left;
        // oldWidth = width;
        // oldHeight = height;
        // super.save();
        unimplemented!()
    }
}

// extends TwoAxisChart
pub struct BarChart {
    bar_width: f64,
    bar_spacing: f64,
    bar_group_width: f64,
}

impl BarChart {
    pub fn new() -> Self {
        // : super(container)
        // _defaultOptions = mergeMaps(globalOptions, _barChartDefaultOptions);
        unimplemented!()
    }

    fn get_bar_left(series_index: usize, bar_index: usize) -> f64 {
        // xLabelX(barIndex) -
        //     0.5 * _barGroupWidth +
        //     countVisibleSeries(seriesIndex) * (barWidth + barSpacing)
        unimplemented!()
    }

    fn update_bar_width() {
        // let count = countVisibleSeries();
        // if count > 0 {
        //   barWidth = (barGroupWidth + barSpacing) / count - barSpacing;
        // } else {
        //   barWidth = 0.0;
        // }
        unimplemented!()
    }

    fn value_to_bar_height(value: f64) -> f64 {
        // if value != null {
        //   return xAxisTop - valueToY(value);
        // }
        // return 0;
        unimplemented!()
    }

    fn calculate_average_y_values(index: usize) {
        // if (!_options["tooltip"]["enabled"]) return;

        // let entityCount = _seriesList.first.entities.length;
        // let start = index ?? 0;
        // let end = index == null ? entityCount : index + 1;

        // averageYValues ??= <num>[];
        // averageYValues.length = entityCount;

        // for (let i = start; i < end; i++) {
        //   let sum = 0.0;
        //   let count = 0;
        //   for (let j = seriesList.length - 1; j >= 0; j--) {
        //     let state = seriesStates[j];
        //     if (state == VisibilityState.hidden) continue;
        //     if (state == VisibilityState.hiding) continue;

        //     let bar = seriesList[j].entities[i] as Bar;
        //     if (bar.value != null) {
        //       sum += bar.height;
        //       count++;
        //     }
        //   }
        //   averageYValues[i] = (count > 0) ? xAxisTop - sum / count : null;
        // }
    }

    // fn create_entity(
    //     seriesIndex: usize,
    //     entityIndex: usize,
    //     value: String,
    //     color: String,
    //     highlightColor: String,
    // ) -> Entity {
    //     // let left = _getBarLeft(seriesIndex, entityIndex);
    //     // let oldLeft = left;
    //     // let height = _valueToBarHeight(value);

    //     // // Animate width.
    //     // num oldHeight = height;
    //     // num oldWidth = 0;

    //     // if (_seriesList == null) {
    //     //   // Data table changed. Animate height.
    //     //   oldHeight = 0;
    //     //   oldWidth = _barWidth;
    //     // }

    //     // return _Bar()
    //     //   ..index = entityIndex
    //     //   ..value = value
    //     //   ..formattedValue = value != null ? _entityValueFormatter(value) : null
    //     //   ..color = color
    //     //   ..highlightColor = highlightColor
    //     //   ..bottom = _xAxisTop
    //     //   ..oldLeft = oldLeft
    //     //   ..left = left
    //     //   ..oldHeight = oldHeight
    //     //   ..height = height
    //     //   ..oldWidth = oldWidth
    //     //   ..width = _barWidth;
    //     unimplemented!()
    // }

    fn series_visibility_changed(index: usize) {
        // _updateBarWidth();
        // _updateSeries();
        // _calculateAverageYValues();
    }
}

impl Chart for BarChart {
    fn calculate_drawing_sizes() {
        // super._calculateDrawingSizes();
        // barGroupWidth = 0.618 * _xLabelHop; // Golden ratio.
        // tooltipOffset = 0.5 * xLabelHop + 4;
        // updateBarWidth();
        unimplemented!()
    }

    fn draw_series(percent: f64) -> bool {
        // for (let i = 0, n = _seriesList.length; i < n; i++) {
        //   if (_seriesStates[i] == _VisibilityState.hidden) continue;

        //   let series = _seriesList[i];

        //   // Draw the bars.
        //   for (_Bar bar in series.entities) {
        //     if (bar.value == null) continue;
        //     bar.draw(_seriesContext, percent, false);
        //   }

        //   let opt = _options["xAxis"]["crosshair"];
        //   if (_focusedEntityIndex >= 0 && opt["enabled"]) {
        //     _seriesContext
        //       ..fillStyle = opt["color"]
        //       ..fillRect(_yAxisLeft + _xLabelHop * _focusedEntityIndex,
        //           _xAxisTop - _yAxisLength, _xLabelHop, _yAxisLength);
        //   }

        //   // Draw the labels.
        //   if (percent == 1.0) {
        //     opt = _options["series"]["labels"];
        //     if (!opt["enabled"]) continue;
        //     _seriesContext
        //       ..fillStyle = opt["style"]["color"]
        //       ..font = _getFont(opt["style"])
        //       ..textAlign = "center"
        //       ..textBaseline = "alphabetic";
        //     for (_Bar bar in series.entities) {
        //       if (bar.value == null) continue;
        //       let x = bar.left + .5 * bar.width;
        //       let y = _xAxisTop - bar.height - 5;
        //       _seriesContext.fillText(bar.formattedValue, x, y);
        //     }
        //   }
        // }

        return false;
    }

    fn update_series(index: usize) {
        // let entityCount = _dataTable.rows.length;
        // for (let i = 0; i < _seriesList.length; i++) {
        //   let series = _seriesList[i];
        //   let left = _getBarLeft(i, 0);
        //   let barWidth = 0.0;
        //   if (_seriesStates[i].index >= _VisibilityState.showing.index) {
        //     barWidth = _barWidth;
        //   }
        //   let color = _getColor(i);
        //   let highlightColor = _getHighlightColor(color);
        //   series.color = color;
        //   series.highlightColor = highlightColor;
        //   for (let j = 0; j < entityCount; j++) {
        //     let bar = series.entities[j] as _Bar;
        //     bar.index = j;
        //     bar.color = color;
        //     bar.highlightColor = highlightColor;
        //     bar.left = left;
        //     bar.bottom = _xAxisTop;
        //     bar.height = _valueToBarHeight(bar.value);
        //     bar.width = barWidth;
        //     left += _xLabelHop;
        //   }
        // }
        unimplemented!()
    }
}
