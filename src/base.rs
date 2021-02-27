#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;
use ux_primitives::canvas::CanvasContext;

use crate::{DataCellChangeRecord, DataCollectionChangeRecord, DataTable, EasingFunction};

/// The 2*pi constant.
// const 2pi: f64 = 2 * pi;

/// The pi/2 constant.
// const pi_2: f64 = f64::FRAC_PI_2;

const FONT_FAMILY: &str = r#""Segoe UI", "Open Sans", Verdana, Arial"#;

/// The padding of the chart itself.
const CHART_PADDING: usize = 12;

/// The margin between the legend and the chart-axes box in pixels.
const LEGEND_MARGIN: usize = 12;

const CHART_TITLE_MARGIN: usize = 12;

/// The padding around the chart title and axis titles.
const TITLE_PADDING: usize = 6;

/// The top-and/or-bottom margin of x-axis labels and the right-and/or-left
/// margin of y-axis labels.
///
/// x-axis labels always have top margin. If the x-axis title is N/A, x-axis
/// labels also have bottom margin.
///
/// y-axis labels always have right margin. If the y-axis title is N/A, y-axis
/// labels also have left margin.
const AXIS_LABEL_MARGIN: usize = 12;

type LabelFormatter = fn(label: String) -> String;

type ValueFormatter = fn(value: f64) -> String;

fn default_label_formatter(label: String) -> String {
    label
}

fn default_value_formatter(value: f64) -> String {
    "$value".into()
}

pub enum VisibilityState {
    HIDDEN,
    HIDDING,
    SHOWING,
    SHOWN,
}

pub struct Point;
pub struct Rectangle;
pub struct MouseEvent;

/// A chart entity such as a point, a bar, a pie...
pub trait Entity {
    // Chart chart;
    // String color;
    // String highlightColor;
    // String formattedValue;
    // num index;
    // num oldValue;
    // num value;

    fn free() {
        // chart = null;
    }

    fn save() {
        // oldValue = value;
    }
}

pub trait Drawable<C>
where
    C: CanvasContext,
{
    fn draw(ctx: C, percent: f64, highlight: bool);
}

pub struct Series<E>
where
    E: Entity,
{
    name: String,
    color: String,
    highlight_color: String,
    entities: Vec<E>,
}

impl<E> Series<E>
where
    E: Entity,
{
    fn new(name: String, color: String, highlight_color: String, entities: Vec<E>) {
        unimplemented!()
    }

    // end is optional
    fn free_entities(start: i64, end: i64) {
        // end ??= entities.length;
        // while (start < end) {
        //   entities[start].free();
        //   start++;
        // }
        unimplemented!()
    }
}

/// Base class for all charts.
pub struct BaseChart<'a, C, E>
where
    C: CanvasContext,
    E: Entity,
{
    /// ID of the current animation frame.
    animation_frame_id: i64,

    /// The starting time of an animation cycle.
    animation_start_time: f64,

    // dataTableSubscriptionTracker: StreamSubscriptionTracker, // = StreamSubscriptionTracker();
    /// The data table.
    /// Row 0 contains column names.
    /// Column 0 contains x-axis/pie labels.
    /// Column 1..n - 1 contain series data.
    data_table: DataTable<'a>,

    easing_function: EasingFunction,

    /// The default drawing options initialized in the constructor.
    default_options: HashMap<String, String>,

    /// The drawing options.
    options: HashMap<String, String>,

    /// The chart"s width.
    height: i64,

    /// The chart"s height.
    width: i64,

    /// Index of the highlighted poi64 group/bar group/pie/...
    focused_entity_index: i64, // = -1;

    focused_series_index: i64, // = -1;

    entity_value_formatter: ValueFormatter,

    // /// The legend element.
    // legend: Element,

    // /// The subscription tracker for legend items" events.
    // legendItemSubscriptionTracker: StreamSubscriptionTracker, // = StreamSubscriptionTracker();

    // mouseMoveSub: StreamSubscription,

    // /// The tooltip element. To position the tooltip, change its transform CSS.
    // tooltip: Element,
    /// The function used to format series names to display in the tooltip.
    tooltip_label_formatter: LabelFormatter,

    /// The function used to format series data to display in the tooltip.
    tooltip_value_formatter: ValueFormatter,

    // /// Bounding box of the series and axes.
    // seriesAndAxesBox: Rectangle<int>,

    // /// Bounding box of the chart title.
    // titleBox: Rectangle<int>,
    /// The main rendering context.
    context: C,

    /// The rendering context for the axes.
    axes_context: C,

    /// The rendering context for the series.
    series_context: C,

    eries_list: Vec<Series<E>>,

    /// A list used to keep track of the visibility of the series.
    series_states: Vec<VisibilityState>,
    // The color cache used by changeColorAlpha. (should be doc)
    //  static colorCache = <String, String>{};
}

pub trait Chart {
    /// Calculates various drawing sizes.
    ///
    /// Overriding methods must call this method first to have [_seriesAndAxesBox]
    /// calculated.
    ///
    /// To be overridden.
    fn calculate_drawing_sizes();

    /// Updates the series at index [index]. If [index] is `null`, updates all
    /// series.
    ///
    /// To be overridden.
    // index is opt
    fn update_series(index: usize) {}

    /// Draws the axes and the grid.
    ///
    /// To be overridden.
    fn draw_axes_and_grid() {}

    /// Draws the series given the current animation percent [percent].
    ///
    /// If this method returns `false`, the animation is continued until [percent]
    /// reaches 1.0.
    ///
    /// If this method returns `true`, the animation is stopped immediately.
    /// This is useful as there are cases where no animation is expected.
    /// In those cases, the overriding method will return `true` to stop the
    /// animation.
    ///
    /// To be overridden.
    fn draw_series(percent: f64) -> bool {
        true
    }
}

impl<'a, C, E> BaseChart<'a, C, E>
where
    C: CanvasContext,
    E: Entity,
{
    /// Creates a new color by combining the R, G, B components of [color] with
    /// [alpha].
    fn change_color_alpha(color: String, alpha: f64) {
        // let key = "$color$alpha";
        // let result = _colorCache[key];
        // if (result == null) {
        //   // Convert [color] to HEX/RGBA format using [_context].
        //   _context.fillStyle = color;
        //   color = _context.fillStyle;

        //   if (color[0] == "#") {
        //     result = hexToRgba(color, alpha);
        //   } else {
        //     let list = color.split(",");
        //     list[list.length - 1] = "$alpha)";
        //     result = list.join(",");
        //   }
        //   _colorCache[key] = result;
        // }
        // return result;
        unimplemented!()
    }

    /// Counts the number of visible series up to (but not including) the [end]th
    /// series.
    // end is opt
    fn count_visible_series(end: i64) -> i64 {
        // end ??= _seriesStates.length;
        // return _seriesStates
        //     .take(end)
        //     .where((e) => e.index >= _VisibilityState.showing.index)
        //     .length;
        unimplemented!()
    }

    fn get_color(index: i64) -> String {
        // let colors = _options["colors"] as List;
        // return colors[index % colors.length];
        unimplemented!()
    }

    fn get_highlight_color(color: String) -> String {
        // changeColorAlpha(color, .5);
        unimplemented!()
    }

    /// Returns a CSS font string given a map that contains at least three keys:
    /// `fontStyle`, `fontSize`, and `fontFamily`.
    fn get_gont(style: HashMap<String, String>) -> String {
        // "${style["fontStyle"]} ${style["fontSize"]}px ${style["fontFamily"]}"
        unimplemented!()
    }

    /// Called when the animation ends.
    fn animation_end() {
        // _animationStartTime = null;

        // for (let series in _seriesList) {
        //   for (let entity in series.entities) {
        //     entity.save();
        //   }
        // }

        // let callback = _options["animation"]["onEnd"];
        // if (callback != null) callback();
    }

    /// Calculates various drawing sizes.
    ///
    /// Overriding methods must call this method first to have [_seriesAndAxesBox]
    /// calculated.
    ///
    /// To be overridden.
    fn calculate_drawing_sizes() {
        // let title = _options["title"];
        // let titleX = 0;
        // let titleY = 0;
        // let titleW = 0;
        // let titleH = 0;
        // if (title["position"] != "none" && title["text"] != null) {
        //   titleH = title["style"]["fontSize"] + 2 * _titlePadding;
        // }
        // _seriesAndAxesBox = MutableRectangle(_chartPadding, _chartPadding,
        //     _width - 2 * _chartPadding, _height - 2 * _chartPadding);

        // // Consider the title.

        // if (titleH > 0) {
        //   switch (title["position"]) {
        //     case "above":
        //       titleY = _chartPadding;
        //       _seriesAndAxesBox.top += titleH + _chartTitleMargin;
        //       _seriesAndAxesBox.height -= titleH + _chartTitleMargin;
        //       break;
        //     case "middle":
        //       titleY = (_height - titleH) ~/ 2;
        //       break;
        //     case "below":
        //       titleY = _height - titleH - _chartPadding;
        //       _seriesAndAxesBox.height -= titleH + _chartTitleMargin;
        //       break;
        //   }
        //   _context.font = _getFont(title["style"]);
        //   titleW =
        //       _context.measureText(title["text"]).width.round() + 2 * _titlePadding;
        //   titleX = (_width - titleW - 2 * _titlePadding) ~/ 2;
        // }
        // _titleBox = Rectangle(titleX, titleY, titleW, titleH);

        // // Consider the legend.

        // if (_legend != null) {
        //   let lwm = _legend.offsetWidth + _legendMargin;
        //   let lhm = _legend.offsetHeight + _legendMargin;
        //   switch (_options["legend"]["position"]) {
        //     case "right":
        //       _seriesAndAxesBox.width -= lwm;
        //       break;
        //     case "bottom":
        //       _seriesAndAxesBox.height -= lhm;
        //       break;
        //     case "left":
        //       _seriesAndAxesBox.left += lwm;
        //       _seriesAndAxesBox.width -= lwm;
        //       break;
        //     case "top":
        //       _seriesAndAxesBox.top += lhm;
        //       _seriesAndAxesBox.height -= lhm;
        //       break;
        //   }
        // }
    }

    fn create_entities(
        series_index: i64,
        start: i64,
        end: i64,
        color: String,
        highlight_color: String,
    ) -> Vec<E> {
        // let result = <_Entity>[];
        // while (start < end) {
        //   let value = _dataTable.rows[start][seriesIndex + 1];
        //   let e = _create_entity(seriesIndex, start, value, color, highlightColor);
        //   e.chart = this;
        //   result.add(e);
        //   start++;
        // }
        // return result;
        unimplemented!()
    }

    fn create_entity(
        series_index: i64,
        entity_index: i64,
        value: String,
        color: String,
        highlight_color: String,
    ) -> E {
        // null;
        unimplemented!()
    }

    fn create_series_list(start: i64, end: i64) -> Vec<Series<E>> {
        // let result = <_Series>[];
        // let entityCount = _dataTable.rows.length;
        // while (start < end) {
        //   let name = _dataTable.columns[start + 1].name;
        //   let color = _getColor(start);
        //   let highlightColor = _getHighlightColor(color);
        //   let entities =
        //       _createEntities(start, 0, entityCount, color, highlightColor);
        //   result.add(_Series(name, color, highlightColor, entities));
        //   start++;
        // }
        // return result;
        unimplemented!()
    }

    // /// Event handler for [DataTable.onCellChanged].
    // ///
    // /// NOTE: This method only handles the case when [record.columnIndex] >= 1;
    // fn dataCellChanged(record: DataCellChangeRecord) {
    //     // if (record.columnIndex >= 1) {
    //     //   let f = _entityValueFormatter != null && record.newValue != null
    //     //       ? _entityValueFormatter(record.newValue)
    //     //       : null;
    //     //   _seriesList[record.columnIndex - 1].entities[record.rowIndex]
    //     //     ..value = record.newValue
    //     //     ..formattedValue = f;
    //     // }
    // }

    /// Event handler for [DataTable.onRowsChanged].
    fn data_rows_changed(record: DataCollectionChangeRecord) {
        // _calculateDrawingSizes();
        // let entityCount = _dataTable.rows.length;
        // let removedEnd = record.index + record.removedCount;
        // let addedEnd = record.index + record.addedCount;
        // for (let i = 0; i < _seriesList.length; i++) {
        //   let series = _seriesList[i];

        //   // Remove old entities.
        //   if (record.removedCount > 0) {
        //     series.freeEntities(record.index, removedEnd);
        //     series.entities.removeRange(record.index, removedEnd);
        //   }

        //   // Insert new entities.
        //   if (record.addedCount > 0) {
        //     let newEntities = _createEntities(
        //         i, record.index, addedEnd, series.color, series.highlightColor);
        //     series.entities.insertAll(record.index, newEntities);

        //     // Update entity indexes.
        //     for (let j = addedEnd; j < entityCount; j++) {
        //       series.entities[j].index = j;
        //     }
        //   }
        // }
    }

    /// Event handler for [DataTable.onColumnsChanged].
    fn data_columns_changed(record: DataCollectionChangeRecord) {
        // _calculateDrawingSizes();
        // let start = record.index - 1;
        // _updateSeriesVisible(start, record.removedCount, record.addedCount);
        // if (record.removedCount > 0) {
        //   let end = start + record.removedCount;
        //   for (let i = start; i < end; i++) {
        //     _seriesList[i].freeEntities(0);
        //   }
        //   _seriesList.removeRange(start, end);
        // }
        // if (record.addedCount > 0) {
        //   let list = _createSeriesList(start, start + record.addedCount);
        //   _seriesList.insertAll(start, list);
        // }
        // _updateLegendContent();
    }

    /// Called when [_dataTable] has been changed.
    fn data_fable_changed() {
        // calculateDrawingSizes();
        // _seriesList = _createSeriesList(0, _dataTable.columns.length - 1);
    }

    fn update_series_visible(index: i64, removed_count: i64, added_count: i64) {
        // if (removedCount > 0) {
        //   _seriesStates.removeRange(index, index + removedCount);
        // }
        // if (addedCount > 0) {
        //   let list = List.filled(addedCount, _VisibilityState.showing);
        //   _seriesStates.insertAll(index, list);
        // }
        unimplemented!()
    }

    /// Draws the current animation frame.
    ///
    /// If [time] is `null`, draws the last frame (i.e. no animation).
    fn draw_frame(time: f64) {
        // let percent = 1.0;
        // let duration = _options["animation"]["duration"];
        // _animationStartTime ??= time;
        // if (duration > 0 && time != null) {
        //   percent = (time - _animationStartTime) / duration;
        // }

        // if (percent >= 1.0) {
        //   percent = 1.0;

        //   // Update the visibility states of all series before the last frame.
        //   for (let i = _seriesStates.length - 1; i >= 0; i--) {
        //     if (_seriesStates[i] == _VisibilityState.showing) {
        //       _seriesStates[i] = _VisibilityState.shown;
        //     } else if (_seriesStates[i] == _VisibilityState.hiding) {
        //       _seriesStates[i] = _VisibilityState.hidden;
        //     }
        //   }
        // }

        // _context.fillStyle = _options["backgroundColor"];
        // _context.fillRect(0, 0, _width, _height);
        // _seriesContext.clearRect(0, 0, _width, _height);
        // _drawSeries(_easingFunction(percent));
        // _context.drawImageScaled(_axesContext.canvas, 0, 0, _width, _height);
        // _context.drawImageScaled(_seriesContext.canvas, 0, 0, _width, _height);
        // _drawTitle();

        // if (percent < 1.0) {
        //   _animationFrameId = window.requestAnimationFrame(_drawFrame);
        // } else if (time != null) {
        //   _animationEnd();
        // }
    }

    /// Draws the chart title using the main rendering context.
    fn draw_title() {
        // let title = _options["title"];
        // if (title["text"] == null) return;

        // let x = (_titleBox.left + _titleBox.right) ~/ 2;
        // let y = _titleBox.bottom - _titlePadding;
        // _context
        //   ..font = _getFont(title["style"])
        //   ..fillStyle = title["style"]["color"]
        //   ..textAlign = "center"
        //   ..fillText(title["text"], x, y);
    }

    fn initialize_legend() {
        // let n = _getLegendLabels().length;
        // _seriesStates = Vec<_VisibilityState>.filled(n, _VisibilityState.showing,
        //     growable: true);

        // if (_legend != null) {
        //   _legend.remove();
        //   _legend = null;
        // }

        // if (_options["legend"]["position"] == "none") return;

        // _legend = _createTooltipOrLegend(_options["legend"]["style"]);
        // _legend.style.lineHeight = "180%";
        // _updateLegendContent();
        // container.append(_legend);
    }

    /// This must be called after [_calculateDrawingSizes] as we need to know
    /// where the title is in order to position the legend correctly.
    fn position_legend() {
        // if (_legend == null) return;

        // let s = _legend.style;
        // switch (_options["legend"]["position"]) {
        //   case "right":
        //     s.right = "${_chartPadding}px";
        //     s.top = "50%";
        //     s.transform = "translateY(-50%)";
        //     break;
        //   case "bottom":
        //     let bottom = _chartPadding;
        //     if (_options["title"]["position"] == "below" && _titleBox.height > 0) {
        //       bottom += _titleBox.height;
        //     }
        //     s.bottom = "${bottom}px";
        //     s.left = "50%";
        //     s.transform = "translateX(-50%)";
        //     break;
        //   case "left":
        //     s.left = "${_chartPadding}px";
        //     s.top = "50%";
        //     s.transform = "translateY(-50%)";
        //     break;
        //   case "top":
        //     let top = _chartPadding;
        //     if (_options["title"]["position"] == "above" && _titleBox.height > 0) {
        //       top += _titleBox.height;
        //     }
        //     s.top = "${top}px";
        //     s.left = "50%";
        //     s.transform = "translateX(-50%)";
        //     break;
        // }
    }

    fn update_legend_content() {
        // let labels = _getLegendLabels();
        // let formatter =
        //     _options["legend"]["labelFormatter"] ?? _defaultLabelFormatter;
        // _legendItemSubscriptionTracker.clear();
        // _legend.innerHtml = "";
        // for (let i = 0; i < labels.length; i++) {
        //   let label = labels[i];
        //   let formattedLabel = formatter(label);
        //   let e = _createTooltipOrLegendItem(_getColor(i), formattedLabel);
        //   if (label != formattedLabel) {
        //     e.title = label;
        //   }
        //   e.style.cursor = "pointer";
        //   e.style.userSelect = "none";
        //   _legendItemSubscriptionTracker
        //     ..add(e.onClick.listen(_legendItemClick))
        //     ..add(e.onMouseOver.listen(_legendItemMouseOver))
        //     ..add(e.onMouseOut.listen(_legendItemMouseOut));

        //   let state = _seriesStates[i];
        //   if (state == _VisibilityState.hidden ||
        //       state == _VisibilityState.hiding) {
        //     e.style.opacity = ".4";
        //   }

        //   // Display the items in one row if the legend"s position is "top" or
        //   // "bottom".
        //   let pos = _options["legend"]["position"];
        //   if (pos == "top" || pos == "bottom") {
        //     e.style.display = "inline-block";
        //   }
        //   _legend.append(e);
        // }
    }

    fn get_legend_labels() -> Vec<String> {
        // _dataTable.columns.skip(1).map((e) => e.name).toList();
        unimplemented!()
    }

    fn legend_item_click(e: MouseEvent) {
        // if (!isInteractive) return;

        // let item = e.currentTarget as Element;
        // let index = item.parent.children.indexOf(item);

        // if (_seriesStates[index] == _VisibilityState.shown) {
        //   _seriesStates[index] = _VisibilityState.hiding;
        //   item.style.opacity = ".4";
        // } else {
        //   _seriesStates[index] = _VisibilityState.showing;
        //   item.style.opacity = "";
        // }

        // _seriesVisibilityChanged(index);
        // _startAnimation();
    }

    fn legend_item_mouse_over(e: MouseEvent) {
        // if (!isInteractive) return;
        // let item = e.currentTarget as Element;
        // _focusedSeriesIndex = item.parent.children.indexOf(item);
        // _drawFrame(null);
    }

    fn legend_item_mouse_out(e: MouseEvent) {
        // if (!isInteractive) return;
        // _focusedSeriesIndex = -1;
        // _drawFrame(null);
    }

    /// Called when the visibility of a series is changed.
    ///
    /// [index] is the index of the affected series.
    ///
    /// To be overridden.
    fn series_visibility_changed(index: i64) {}

    /// Returns the index of the poi64 group/bar group/pie/... near the position
    /// specified by [x] and [y].
    ///
    /// To be overridden.
    fn get_entity_group_index(x: f64, num: f64) -> i64 {
        -1
    }

    /// Handles `mousemove` or `touchstart` events to highlight appropriate
    /// points/bars/pies/... as well as update the tooltip.
    fn mouse_move(e: MouseEvent) {
        // if (!isInteractive || e.buttons != 0) return;

        // let rect = _context.canvas.getBoundingClientRect();
        // let x = e.client.x - rect.left;
        // let y = e.client.y - rect.top;
        // let index = _getEntityGroupIndex(x, y);

        // if (index != _focusedEntityIndex) {
        //   _focusedEntityIndex = index;
        //   _drawFrame(null);
        //   if (index >= 0) {
        //     _updateTooltipContent();
        //     _tooltip.hidden = false;
        //     let p = _getTooltipPosition();
        //     _tooltip.style.transform = "translate(${p.x}px, ${p.y}px)";
        //   } else {
        //     _tooltip.hidden = true;
        //   }
        // }
    }

    fn initialize_tooltip() {
        // if (_tooltip != null) {
        //   _tooltip.remove();
        //   _tooltip = null;
        // }

        // let opt = _options["tooltip"];
        // if (!opt["enabled"]) return;

        // _tooltipLabelFormatter = opt["labelFormatter"] ?? _defaultLabelFormatter;
        // _tooltipValueFormatter = opt["valueFormatter"] ?? _defaultValueFormatter;
        // _tooltip = _createTooltipOrLegend(opt["style"])
        //   ..hidden = true
        //   ..style.left = "0"
        //   ..style.top = "0"
        //   ..style.boxShadow = "4px 4px 4px rgba(0,0,0,.25)"
        //   ..style.transition = "transform .4s cubic-bezier(.4,1,.4,1)";
        // container.append(_tooltip);

        // _mouseMoveSub?.cancel();
        // _mouseMoveSub = container.onMouseMove.listen(_mouseMove);
    }

    /// Returns the position of the tooltip based on [_focusedEntityIndex].
    /// To be overridden.
    fn get_tooltip_position() -> Point {
        // null
        unimplemented!()
    }

    fn update_tooltip_content() {
        // let columnCount = _dataTable.columns.length;
        // let row = _dataTable.rows[_focusedEntityIndex];
        // _tooltip.innerHtml = "";

        // // Tooltip title.
        // _tooltip.append(DivElement()
        //   ..text = row[0]
        //   ..style.padding = "4px 12px"
        //   ..style.fontWeight = "bold");

        // // Tooltip items.
        // for (let i = 1; i < columnCount; i++) {
        //   let state = _seriesStates[i - 1];
        //   if (state == _VisibilityState.hidden) continue;
        //   if (state == _VisibilityState.hiding) continue;

        //   let series = _seriesList[i - 1];
        //   let value = row[i];
        //   if (value == null) continue;

        //   value = _tooltipValueFormatter(value);
        //   let label = _tooltipLabelFormatter(series.name);

        //   let e = _createTooltipOrLegendItem(
        //       series.color, "$label: <strong>$value</strong>");
        //   _tooltip.append(e);
        // }
    }

    // /// Creates an absolute positioned div with styles specified by [style].
    // fn create_tooltip_or_legend(style: HashMap<String, String>) -> Element {
    //     // return DivElement()
    //     //   ..style.backgroundColor = style["backgroundColor"]
    //     //   ..style.borderColor = style["borderColor"]
    //     //   ..style.borderStyle = "solid"
    //     //   ..style.borderWidth = "${style["borderWidth"]}px"
    //     //   ..style.color = style["color"]
    //     //   ..style.fontFamily = style["fontFamily"]
    //     //   ..style.fontSize = "${style["fontSize"]}px"
    //     //   ..style.fontStyle = style["fontStyle"]
    //     //   ..style.position = "absolute";
    // }

    // fn create_tooltip_or_legend_item(color: String, text: String) -> Element {
    //     // let e = DivElement()
    //     //   ..innerHtml = "<span></span> $text"
    //     //   ..style.padding = "4px 12px";
    //     // e.children.first.style
    //     //   ..backgroundColor = color
    //     //   ..display = "inline-block"
    //     //   ..width = "12px"
    //     //   ..height = "12px";
    //     // return e;
    // }

    fn start_animation() {
        // _animationFrameId = window.requestAnimationFrame(_drawFrame);
    }

    fn stop_animation() {
        // _animationStartTime = null;
        // if (_animationFrameId != 0) {
        //   window.cancelAnimationFrame(_animationFrameId);
        //   _animationFrameId = 0;
        // }
    }

    /// Creates a chart given a container.
    ///
    /// If the CSS position of [container] is "static", it will be changed to
    /// "relative".
    fn new(context: C) {
        // if (container.getComputedStyle().position == "static") {
        //   container.style.position = "relative";
        // }
        // _context = CanvasElement().getContext("2d");
        // _axesContext = CanvasElement().getContext("2d");
        // _seriesContext = CanvasElement().getContext("2d");

        // container.append(_context.canvas);
    }

    // @Deprecated("Use [isAnimating] instead")
    // bool get animating => isAnimating;

    // /// Whether the chart is animating.
    // bool get isAnimating => _animationStartTime != null;

    // /// Whether the chart is interactive.
    // ///
    // /// This property returns `false` if the chart is animating or there are no
    // /// series to draw.
    // bool get isInteractive => !isAnimating && _seriesList != null;

    // /// The element that contains this chart.
    // let Element container;

    // /// The data table that stores chart data.
    // DataTable get dataTable => _dataTable;

    /// Disposes of resources used by this chart. The chart will become unusable
    /// until [draw] is called again.
    ///
    /// Be sure to call this method when the chart is no longer used to afn any
    /// memory leaks.
    ///
    /// @mustCallSuper
    fn dispose() {
        // // This causes [canHandleInteraction] to be `false`.
        // _seriesList = null;
        // _mouseMoveSub?.cancel();
        // _mouseMoveSub = null;
        // _dataTableSubscriptionTracker.clear();
        // _legendItemSubscriptionTracker.clear();
    }

    /// Draws the chart given a data table [dataTable] and an optional set of
    /// options [options].
    // options is opt
    fn draw(data_table: DataTable, options: HashMap<String, String>) {
        // dispose();
        // _dataTable = dataTable;
        // _dataTableSubscriptionTracker
        //   ..add(dataTable.onCellChange.listen(_dataCellChanged))
        //   ..add(dataTable.onColumnsChange.listen(_dataColumnsChanged))
        //   ..add(dataTable.onRowsChange.listen(_dataRowsChanged));
        // _options = mergeMaps(_defaultOptions, options);
        // _easingFunction = getEasingFunction(_options["animation"]["easing"]);
        // _initializeLegend();
        // _initializeTooltip();
        // resize(true);
    }

    /// Resizes the chart to fit the new size of the container.
    // [bool forceRedraw = false]
    fn resize(force_redraw: bool) {
        // let w = container.clientWidth;
        // let h = container.clientHeight;

        // if (w == 0 || h == 0) return;

        // if (w != _width || h != _height) {
        //   _width = w;
        //   _height = h;
        //   forceRedraw = true;

        //   let dpr = window.devicePixelRatio;
        //   let scaledW = (w * dpr).round();
        //   let scaledH = (h * dpr).round();

        //   fn setCanvasSize(ctx: C) {
        //     // Scale the drawing canvas by [dpr] to ensure sharp rendering on
        //     // high pixel density displays.
        //     ctx.canvas
        //       ..style.width = "${w}px"
        //       ..style.height = "${h}px"
        //       ..width = scaledW
        //       ..height = scaledH;
        //     ctx.set_transform(dpr, 0, 0, dpr, 0, 0);
        //   }

        //   setCanvasSize(_context);
        //   setCanvasSize(_axesContext);
        //   setCanvasSize(_seriesContext);
        // }

        // if (forceRedraw) {
        //   _stopAnimation();
        //   _dataTableChanged();
        //   _positionLegend();
        //   update();
        // }
    }

    /// Updates the chart.
    ///
    ///  This method should be called after [dataTable] has been modified.
    // TODO: handle updates while animation is happening.
    // options is opt
    fn update(options: HashMap<String, String>) {
        // if (_width == 0 || _height == 0) return;

        // if (options != null) {
        //   _options = mergeMaps(_options, options);
        // }

        // // This call is redundant for row and column changes but necessary for
        // // cell changes.
        // _calculateDrawingSizes();
        // _updateSeries();
        // _axesContext.clearRect(0, 0, _width, _height);
        // _drawAxesAndGrid();
        // _startAnimation();
    }
}

// extends Chart
pub struct TwoAxisChart {
    x_axis_top: f64,
    y_axis_left: f64,
    x_axis_length: f64,
    y_axis_length: f64,
    x_label_max_width: f64,
    y_label_max_width: f64,
    x_label_rotation: f64, // 0..90
    x_label_step: i64,
    x_label_hop: f64, // Distance between two consecutive x-axis labels.
    y_label_hop: f64, // Distance between two consecutive x-axis labels.
    //  xTitleBox: Rectangle,
    //  yTitleBox: Rectangle,
    x_title_center: Point,
    y_title_center: Point,
    x_labels: Vec<String>,
    y_labels: Vec<String>,
    y_interval: f64,
    y_max_value: f64,
    y_min_value: f64,
    y_range: f64,

    /// The horizontal offset of the tooltip with respect to the vertical line
    /// passing through an x-axis label.
    tooltip_offset: f64,

    y_label_formatter: ValueFormatter,
    average_y_values: Vec<f64>,

    x_label_offset_factor: f64, // = .5;
}

/// Base class for charts having two axes.
impl TwoAxisChart {
    /// Returns the x coordinate of the x-axis label at [index].
    fn x_label_x(index: i64) -> f64 {
        // _yAxisLeft + _xLabelHop * (index + _xLabelOffsetFactor)
        unimplemented!()
    }

    /// Returns the y-coordinate corresponding to the data poi64 [value] and
    /// the animation percent [percent].
    fn value_to_y(value: f64) -> f64 {
        // value != null
        //   ? _xAxisTop - (value - _yMinValue) / _yRange * _yAxisLength
        //   : _xAxisTop;
        unimplemented!()
    }

    /// Calculates average y values for the visible series to help position the
    /// tooltip.
    ///
    /// If [index] is given, calculates the average y value for the entity group
    /// at [index] only.
    ///
    /// To be overridden.
    // index is opt
    fn calculate_average_y_values(index: usize) {}

    // TODO: Separate y-axis stuff into a separate method.
    fn calculate_drawing_sizes() {
        //     super._calculateDrawingSizes();

        //     // y-axis min-max.

        //     _yMaxValue = _options["yAxis"]["maxValue"] ?? double.negativeInfinity;
        //     _yMaxValue = max(_yMaxValue, findMaxValue(_dataTable));
        //     if (_yMaxValue == double.negativeInfinity) _yMaxValue = 0.0;

        //     _yMinValue = _options["yAxis"]["minValue"] ?? double.infinity;
        //     _yMinValue = min(_yMinValue, findMinValue(_dataTable));
        //     if (_yMinValue == double.infinity) _yMinValue = 0.0;

        //     _yInterval = _options["yAxis"]["interval"];
        //     let minInterval = _options["yAxis"]["minInterval"];

        //     if (_yInterval == null) {
        //       if (_yMinValue == _yMaxValue) {
        //         if (_yMinValue == 0.0) {
        //           _yMaxValue = 1.0;
        //           _yInterval = 1.0;
        //         } else if (_yMinValue == 1.0) {
        //           _yMinValue = 0.0;
        //           _yInterval = 1.0;
        //         } else {
        //           _yInterval = _yMinValue * .25;
        //           _yMinValue -= _yInterval;
        //           _yMaxValue += _yInterval;
        //         }
        //         if (minInterval != null) {
        //           _yInterval = max(_yInterval, minInterval);
        //         }
        //       } else {
        //         _yInterval = calculateInterval(_yMaxValue - _yMinValue, 5, minInterval);
        //       }
        //     }

        //     _yMinValue = (_yMinValue / _yInterval).floorToDouble() * _yInterval;
        //     _yMaxValue = (_yMaxValue / _yInterval).ceilToDouble() * _yInterval;
        //     _yRange = _yMaxValue - _yMinValue;

        //     // y-axis labels.

        //     _yLabels = <String>[];
        //     _yLabelFormatter = _options["yAxis"]["labels"]["formatter"];
        //     if (_yLabelFormatter == null) {
        //       let maxDecimalPlaces =
        //           max(getDecimalPlaces(_yInterval), getDecimalPlaces(_yMinValue));
        //       let numberFormat = NumberFormat.decimalPattern()
        //         ..maximumFractionDigits = maxDecimalPlaces
        //         ..minimumFractionDigits = maxDecimalPlaces;
        //       _yLabelFormatter = numberFormat.format;
        //     }
        //     let value = _yMinValue;
        //     while (value <= _yMaxValue) {
        //       _yLabels.add(_yLabelFormatter(value));
        //       value += _yInterval;
        //     }
        //     _yLabelMaxWidth = calculateMaxTextWidth(
        //             _context, _getFont(_options["yAxis"]["labels"]["style"]), _yLabels)
        //         .round();

        //     _entityValueFormatter = _yLabelFormatter;

        //     // Tooltip.

        //     _tooltipValueFormatter =
        //         _options["tooltip"]["valueFormatter"] ?? _yLabelFormatter;

        //     // x-axis title.

        //     let xTitleLeft = 0;
        //     let xTitleTop = 0;
        //     let xTitleWidth = 0;
        //     let xTitleHeight = 0;
        //     let xTitle = _options["xAxis"]["title"];
        //     if (xTitle["text"] != null) {
        //       _context.font = _getFont(xTitle["style"]);
        //       xTitleWidth = _context.measureText(xTitle["text"]).width.round() +
        //           2 * _titlePadding;
        //       xTitleHeight = xTitle["style"]["fontSize"] + 2 * _titlePadding;
        //       xTitleTop = _seriesAndAxesBox.bottom - xTitleHeight;
        //     }

        //     // y-axis title.

        //     let yTitleLeft = 0;
        //     let yTitleTop = 0;
        //     let yTitleWidth = 0;
        //     let yTitleHeight = 0;
        //     let yTitle = _options["yAxis"]["title"];
        //     if (yTitle["text"] != null) {
        //       _context.font = _getFont(yTitle["style"]);
        //       yTitleHeight = _context.measureText(yTitle["text"]).width.round() +
        //           2 * _titlePadding;
        //       yTitleWidth = yTitle["style"]["fontSize"] + 2 * _titlePadding;
        //       yTitleLeft = _seriesAndAxesBox.left;
        //     }

        //     // Axes" size and position.

        //     _yAxisLeft = _seriesAndAxesBox.left + _yLabelMaxWidth + _axisLabelMargin;
        //     if (yTitleWidth > 0) {
        //       _yAxisLeft += yTitleWidth + _chartTitleMargin;
        //     } else {
        //       _yAxisLeft += _axisLabelMargin;
        //     }

        //     _xAxisLength = _seriesAndAxesBox.right - _yAxisLeft;

        //     _xAxisTop = _seriesAndAxesBox.bottom;
        //     if (xTitleHeight > 0) {
        //       _xAxisTop -= xTitleHeight + _chartTitleMargin;
        //     } else {
        //       _xAxisTop -= _axisLabelMargin;
        //     }
        //     _xAxisTop -= _axisLabelMargin;

        //     // x-axis labels and x-axis"s position.

        //     let rowCount = _dataTable.rows.length;
        //     _xLabels = <String>[];
        //     for (let i = 0; i < rowCount; i++) {
        //       _xLabels.add(_dataTable.rows[i][0].to_string());
        //     }
        //     _xLabelMaxWidth = calculateMaxTextWidth(
        //         _context, _getFont(_options["xAxis"]["labels"]["style"]), _xLabels);
        //     if (_xLabelOffsetFactor > 0 && rowCount > 1) {
        //       _xLabelHop = _xAxisLength / rowCount;
        //     } else if (rowCount > 1) {
        //       _xLabelHop = _xAxisLength / (rowCount - 1);
        //     } else {
        //       _xLabelHop = _xAxisLength;
        //     }
        //     _xLabelRotation = 0;

        //     let fontSize = _options["xAxis"]["labels"]["style"]["fontSize"];
        //     let maxRotation = _options["xAxis"]["labels"]["maxRotation"];
        //     let minRotation = _options["xAxis"]["labels"]["minRotation"];
        //     const angles = [0, -45, 45, -90, 90];

        //     outer:
        //     for (let step = 1; step <= rowCount; step++) {
        //       let scaledLabelHop = step * _xLabelHop;
        //       let minSpacing = max(.1 * scaledLabelHop, 10);
        //       for (let angle in angles) {
        //         if (angle > maxRotation) continue;
        //         if (angle < minRotation) continue;

        //         let absAngleRad = deg2rad(angle).abs();
        //         let labelSpacing = angle == 0
        //             ? scaledLabelHop - _xLabelMaxWidth
        //             : scaledLabelHop * sin(absAngleRad) - fontSize;
        //         if (labelSpacing < minSpacing) continue;

        //         _xLabelRotation = angle;
        //         _xLabelStep = step;
        //         _xAxisTop -=
        //             _xLabelMaxWidth * sin(absAngleRad) + fontSize * cos(absAngleRad);
        //         break outer;
        //       }
        //     }

        //     // Wrap up.

        //     _yAxisLength = _xAxisTop -
        //         _seriesAndAxesBox.top -
        //         _options["yAxis"]["labels"]["style"]["fontSize"] ~/ 2;
        //     _yLabelHop = _yAxisLength / (_yLabels.length - 1);

        //     xTitleLeft = _yAxisLeft + (_xAxisLength - xTitleWidth) ~/ 2;
        //     yTitleTop = _seriesAndAxesBox.top + (_yAxisLength - yTitleHeight) ~/ 2;

        //     if (xTitleHeight > 0) {
        // //      _xTitleBox =
        // //          Rectangle(xTitleLeft, xTitleTop, xTitleWidth, xTitleHeight);
        //       _xTitleCenter =
        //           Point(xTitleLeft + xTitleWidth ~/ 2, xTitleTop + xTitleHeight ~/ 2);
        //     } else {
        // //      _xTitleBox = null;
        //       _xTitleCenter = null;
        //     }

        //     if (yTitleHeight > 0) {
        // //      _yTitleBox =
        // //          Rectangle(yTitleLeft, yTitleTop, yTitleWidth, yTitleHeight);
        //       _yTitleCenter =
        //           Point(yTitleLeft + yTitleWidth ~/ 2, yTitleTop + yTitleHeight ~/ 2);
        //     } else {
        // //      _yTitleBox = null;
        //       _yTitleCenter = null;
        //     }
        unimplemented!()
    }

    // fn data_cell_changed(record: DataCellChangeRecord) {
    //     // if record.columnIndex == 0 {
    //     //   _xLabels[record.rowIndex] = record.newValue;
    //     // } else {
    //     //   super._dataCellChanged(record);
    //     // }
    // }

    fn draw_axes_and_grid() {
        // // x-axis title.

        // if (_xTitleCenter != null) {
        //   let opt = _options["xAxis"]["title"];
        //   _axesContext
        //     ..fillStyle = opt["style"]["color"]
        //     ..font = _getFont(opt["style"])
        //     ..textAlign = "center"
        //     ..textBaseline = "middle"
        //     ..fillText(opt["text"], _xTitleCenter.x, _xTitleCenter.y);
        // }

        // // y-axis title.

        // if (_yTitleCenter != null) {
        //   let opt = _options["yAxis"]["title"];
        //   _axesContext
        //     ..save()
        //     ..fillStyle = opt["style"]["color"]
        //     ..font = _getFont(opt["style"])
        //     ..translate(_yTitleCenter.x, _yTitleCenter.y)
        //     ..rotate(-_pi_2)
        //     ..textAlign = "center"
        //     ..textBaseline = "middle"
        //     ..fillText(opt["text"], 0, 0)
        //     ..restore();
        // }

        // // x-axis labels.

        // let opt = _options["xAxis"]["labels"];
        // _axesContext.fillStyle = opt["style"]["color"];
        // _axesContext.font = _getFont(opt["style"]);
        // let x = _xLabelX(0);
        // let y = _xAxisTop + _axisLabelMargin + opt["style"]["fontSize"];
        // let scaledLabelHop = _xLabelStep * _xLabelHop;

        // if (_xLabelRotation == 0) {
        //   _axesContext.textAlign = "center";
        //   _axesContext.textBaseline = "alphabetic";
        //   for (let i = 0; i < _xLabels.length; i += _xLabelStep) {
        //     _axesContext.fillText(_xLabels[i], x, y);
        //     x += scaledLabelHop;
        //   }
        // } else {
        //   _axesContext.textAlign = _xLabelRotation < 0 ? "right" : "left";
        //   _axesContext.textBaseline = "middle";
        //   if (_xLabelRotation == 90) {
        //     x += _xLabelRotation.sign * (opt["style"]["fontSize"] ~/ 8);
        //   }
        //   let angle = deg2rad(_xLabelRotation);
        //   for (let i = 0; i < _xLabels.length; i += _xLabelStep) {
        //     _axesContext
        //       ..save()
        //       ..translate(x, y)
        //       ..rotate(angle)
        //       ..fillText(_xLabels[i], 0, 0)
        //       ..restore();
        //     x += scaledLabelHop;
        //   }
        // }

        // // y-axis labels.

        // _axesContext
        //   ..fillStyle = _options["yAxis"]["labels"]["style"]["color"]
        //   ..font = _getFont(_options["yAxis"]["labels"]["style"])
        //   ..textAlign = "right"
        //   ..textBaseline = "middle";
        // x = _yAxisLeft - _axisLabelMargin;
        // y = _xAxisTop - (_options["yAxis"]["labels"]["style"]["fontSize"] ~/ 8);
        // for (let label in _yLabels) {
        //   _axesContext.fillText(label, x, y);
        //   y -= _yLabelHop;
        // }

        // // x grid lines - draw bottom up.

        // if (_options["xAxis"]["gridLineWidth"] > 0) {
        //   _axesContext
        //     ..lineWidth = _options["xAxis"]["gridLineWidth"]
        //     ..strokeStyle = _options["xAxis"]["gridLineColor"]
        //     ..beginPath();
        //   y = _xAxisTop - _yLabelHop;
        //   for (let i = _yLabels.length - 1; i >= 1; i--) {
        //     _axesContext.moveTo(_yAxisLeft, y);
        //     _axesContext.lineTo(_yAxisLeft + _xAxisLength, y);
        //     y -= _yLabelHop;
        //   }
        //   _axesContext.stroke();
        // }

        // // y grid lines or x-axis ticks - draw from left to right.

        // let lineWidth = _options["yAxis"]["gridLineWidth"];
        // x = _yAxisLeft;
        // if (_xLabelStep > 1) {
        //   x = _xLabelX(0);
        // }
        // if (lineWidth > 0) {
        //   y = _xAxisTop - _yAxisLength;
        // } else {
        //   lineWidth = 1;
        //   y = _xAxisTop + _axisLabelMargin;
        // }
        // _axesContext
        //   ..lineWidth = lineWidth
        //   ..strokeStyle = _options["yAxis"]["gridLineColor"]
        //   ..beginPath();
        // for (let i = 0; i < _xLabels.length; i += _xLabelStep) {
        //   _axesContext.moveTo(x, y);
        //   _axesContext.lineTo(x, _xAxisTop);
        //   x += scaledLabelHop;
        // }
        // _axesContext.stroke();

        // // x-axis itself.

        // if (_options["xAxis"]["lineWidth"] > 0) {
        //   _axesContext
        //     ..lineWidth = _options["xAxis"]["lineWidth"]
        //     ..strokeStyle = _options["xAxis"]["lineColor"]
        //     ..beginPath()
        //     ..moveTo(_yAxisLeft, _xAxisTop)
        //     ..lineTo(_yAxisLeft + _xAxisLength, _xAxisTop)
        //     ..stroke();
        // }

        // // y-axis itself.

        // if (_options["yAxis"]["lineWidth"] > 0) {
        //   _axesContext
        //     ..lineWidth = _options["yAxis"]["lineWidth"]
        //     ..strokeStyle = _options["yAxis"]["lineColor"]
        //     ..beginPath()
        //     ..moveTo(_yAxisLeft, _xAxisTop - _yAxisLength)
        //     ..lineTo(_yAxisLeft, _xAxisTop)
        //     ..stroke();
        // }
    }

    fn get_entity_group_index(x: f64, num: f64) -> i64 {
        // let dx = x - _yAxisLeft;
        // // If (x, y) is inside the rectangle defined by the two axes.
        // if (y > _xAxisTop - _yAxisLength &&
        //     y < _xAxisTop &&
        //     dx > 0 &&
        //     dx < _xAxisLength) {
        //   let index = (dx / _xLabelHop - _xLabelOffsetFactor).round();
        //   // If there is at least one visible poi64 in the current poi64 group...
        //   if (_averageYValues[index] != null) return index;
        // }
        // return -1;
        unimplemented!()
    }

    fn get_tooltip_position() -> Point {
        // let x = _xLabelX(_focusedEntityIndex) + _tooltipOffset;
        // let y = max(_xAxisTop - _yAxisLength,
        //     _averageYValues[_focusedEntityIndex] - _tooltip.offsetHeight ~/ 2);
        // if (x + _tooltip.offsetWidth > _width) {
        //   x -= _tooltip.offsetWidth + 2 * _tooltipOffset;
        //   x = max(x, _yAxisLeft);
        // }
        // return Point(x, y);
        unimplemented!()
    }

    // fn new(container: Element) {
    //     // super(container);
    // }

    // options is opt
    fn update(options: HashMap<String, String>) {
        // super.update(options);
        // _calculateAverageYValues();
    }
}
