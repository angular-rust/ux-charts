use std::{borrow::Borrow, cell::RefCell, collections::HashMap, fmt, rc::Rc};
use ux_primitives::{canvas::*, math::*};

use super::{
    BaseOption, Chart, DataCollectionChangeRecord, DataStream, Easing, EasingFunction, Entity,
    LabelFormatter, MouseEvent, Series, ValueFormatter, Visibility,
};

/// Base class for all charts.
#[derive(Default, Clone)]
pub struct BaseChart<'a, C, E, M, D, O>
where
    C: CanvasContext,
    E: Entity,
    M: fmt::Display,
    D: fmt::Display,
    O: BaseOption<'a>,
{
    /// ID of the current animation frame.
    pub animation_frame_id: i64,

    /// The starting time of an animation cycle.
    pub animation_start_time: Option<f64>,

    /// The data table that stores chart data
    /// Row 0 contains column names.
    /// Column 0 contains x-axis/pie labels.
    /// Column 1..n - 1 contain series data.
    pub data_table: DataStream<'a, M, D>,

    // dataTableSubscriptionTracker: StreamSubscriptionTracker, // = StreamSubscriptionTracker();
    pub easing_function: Option<EasingFunction>,

    // /// The default drawing options initialized in the constructor.
    // default_options: O,

    /// The drawing options initialized in the constructor.
    pub options: O,

    /// The chart"s width.
    pub height: RefCell<f64>,

    /// The chart"s height.
    pub width: RefCell<f64>,

    /// Index of the highlighted point group/bar group/pie/...
    pub focused_entity_index: i64, // = -1;

    pub focused_series_index: i64, // = -1;

    pub entity_value_formatter: Option<ValueFormatter>,

    /// The legend element.
    legend: Option<bool>, //Element,

    // /// The subscription tracker for legend items" events.
    // legendItemSubscriptionTracker: StreamSubscriptionTracker, // = StreamSubscriptionTracker();

    // mouseMoveSub: StreamSubscription,

    /// The tooltip element. To position the tooltip, change its transform CSS.
    tooltip: Option<bool>, //Element,
    /// The function used to format series names to display in the tooltip.
    pub tooltip_label_formatter: Option<LabelFormatter>,

    /// The function used to format series data to display in the tooltip.
    pub tooltip_value_formatter: Option<ValueFormatter>,

    /// Bounding box of the series and axes.
    pub series_and_axes_box: Rectangle<i64>,

    /// Bounding box of the chart title.
    pub title_box: Rectangle<i64>,
    /// The main rendering context.
    pub context: Option<C>,

    /// The rendering context for the axes.
    pub axes_context: Option<C>,

    /// The rendering context for the series.
    pub series_context: Option<C>,

    pub series_list: Vec<Series<E>>,

    /// A list used to keep track of the visibility of the series.
    pub series_states: Vec<Visibility>,

    /// The color cache used by change_color_alpha. (should be doc)
    // should be global static cache
    pub color_cache: HashMap<String, String>,
}

impl<'a, C, E, M, D, O> BaseChart<'a, C, E, M, D, O>
where
    C: CanvasContext,
    E: Entity,
    M: fmt::Display,
    D: fmt::Display,
    O: BaseOption<'a>,
{
    /// Creates a chart given a container.
    ///
    /// If the CSS position of [container] is "static", it will be changed to
    /// "relative".
    pub fn new(options: O) -> Self {
        // if (container.getComputedStyle().position == "static") {
        //   container.style.position = "relative";
        // }
        // context = CanvasElement().getContext("2d");
        // axes_context = CanvasElement().getContext("2d");
        // series_context = CanvasElement().getContext("2d");

        // container.append(context.canvas);
        Self {
            animation_frame_id: 0,
            animation_start_time: None,
            data_table: Default::default(),
            easing_function: None,
            // default_options: O,
            options,
            height: RefCell::new(0.0),
            width: RefCell::new(0.0),
            focused_entity_index: 0, // = -1;
            focused_series_index: 0, // = -1;
            entity_value_formatter: None,
            legend: None,
            // legendItemSubscriptionTracker: StreamSubscriptionTracker, // = StreamSubscriptionTracker();
            // mouseMoveSub: StreamSubscription,
            tooltip: None,
            tooltip_label_formatter: None,
            tooltip_value_formatter: None,
            series_and_axes_box: Default::default(),
            title_box: Default::default(),
            context: None,
            axes_context: None,
            series_context: None,
            series_list: Vec::new(),
            series_states: Vec::new(),
            color_cache: HashMap::new(),
        }
    }

    /// Creates a new color by combining the R, G, B components of [color] with
    /// [alpha].
    pub fn change_color_alpha(&self, color: &str, alpha: f64) -> String {

        let o = self.options.animation();

        let key = format!("{}{}", color, alpha);
        let result = self.color_cache.get(&key);
        match result {
            Some(color) => color.clone(),
            None => {
                // // Convert [color] to HEX/RGBA format using [context].
                // context.fillStyle = color;
                // color = context.fillStyle;

                // if (color[0] == "#") {
                // result = hexToRgba(color, alpha);
                // } else {
                // let list = color.split(",");
                // list[list.length - 1] = "$alpha)";
                // result = list.join(",");
                // }
                // color_cache[key] = result;
                "".into()
            }
        }
    }

    /// Counts the number of visible series up to (but not including) the [end]th
    /// series.
    // end is opt
    pub fn count_visible_series(&self, end: Option<usize>) -> usize {
        let end = match end {
            Some(end) => end,
            None => self.series_states.len(),
        };

        // return series_states
        //     .take(end)
        //     .where((e) => e.index >= Visibility::showing.index)
        //     .length;
        unimplemented!()
    }

    pub fn get_color(&self, index: usize) -> String {
        let colors = self.options.colors();
        let color = colors.get(index % colors.len()).unwrap();
        color.clone().into()
    }

    pub fn get_highlight_color(&self, color: &str) -> String {
        self.change_color_alpha(color, 0.5)
    }

    /// Called when the animation ends.
    pub fn animation_end(&mut self) {
        self.animation_start_time = None;

        for series in &self.series_list {
            for entity in &series.entities {
                entity.save();
            }
        }

        let animation = self.options.animation();

        if let Some(callback) = animation.on_end {
            callback();
        }
    }

    /// Calculates various drawing sizes.
    ///
    /// Overriding methods must call this method first to have [series_and_axes_box]
    /// calculated.
    ///
    /// To be overridden.
    pub fn calculate_drawing_sizes(&self) {
        // let title = options["title"];
        let title_x = 0;
        let title_y = 0;
        let title_w = 0;
        let title_h = 0;
        // if (title["position"] != "none" && title["text"] != null) {
        //   titleH = title["style"]["fontSize"] + 2 * title_padding;
        // }
        // series_and_axes_box = MutableRectangle(chart_padding, chart_padding,
        //     _width - 2 * chart_padding, _height - 2 * chart_padding);

        // // Consider the title.

        // if (titleH > 0) {
        //   switch (title["position"]) {
        //     case "above":
        //       titleY = chart_padding;
        //       series_and_axes_box.top += titleH + chart_title_margin;
        //       series_and_axes_box.height -= titleH + chart_title_margin;
        //       break;
        //     case "middle":
        //       titleY = (_height - titleH) ~/ 2;
        //       break;
        //     case "below":
        //       titleY = _height - titleH - chart_padding;
        //       series_and_axes_box.height -= titleH + chart_title_margin;
        //       break;
        //   }
        //   context.font = get_font(title["style"]);
        //   titleW =
        //       context.measureText(title["text"]).width.round() + 2 * title_padding;
        //   titleX = (_width - titleW - 2 * title_padding) ~/ 2;
        // }
        // title_box = Rectangle(titleX, titleY, titleW, titleH);

        // // Consider the legend.

        // if (self.legend != null) {
        //   let lwm = self.legend.offsetWidth + legend_margin;
        //   let lhm = self.legend.offsetHeight + legend_margin;
        //   switch (options["legend"]["position"]) {
        //     case "right":
        //       series_and_axes_box.width -= lwm;
        //       break;
        //     case "bottom":
        //       series_and_axes_box.height -= lhm;
        //       break;
        //     case "left":
        //       series_and_axes_box.left += lwm;
        //       series_and_axes_box.width -= lwm;
        //       break;
        //     case "top":
        //       series_and_axes_box.top += lhm;
        //       series_and_axes_box.height -= lhm;
        //       break;
        //   }
        // }
    }

    // /// Event handler for [DataTable.onCellChanged].
    // ///
    // /// NOTE: This method only handles the case when [record.columnIndex] >= 1;
    // fn data_cell_changed(&self, record: DataCellChangeRecord) {
    //     // if (record.columnIndex >= 1) {
    //     //   let f = entity_value_formatter != null && record.newValue != null
    //     //       ? entity_value_formatter(record.newValue)
    //     //       : null;
    //     //   series_list[record.columnIndex - 1].entities[record.rowIndex]
    //     //     ..value = record.newValue
    //     //     ..formattedValue = f;
    //     // }
    // }

    /// Event handler for [DataTable.onRowsChanged].
    pub fn data_rows_changed(&self, record: DataCollectionChangeRecord) {
        self.calculate_drawing_sizes();
        // let entityCount = data_table.rows.length;
        // let removedEnd = record.index + record.removedCount;
        // let addedEnd = record.index + record.addedCount;
        // for (let i = 0; i < series_list.length; i++) {
        //   let series = series_list[i];

        //   // Remove old entities.
        //   if (record.removedCount > 0) {
        //     series.freeEntities(record.index, removedEnd);
        //     series.entities.remove_range(record.index, removedEnd);
        //   }

        //   // Insert new entities.
        //   if (record.addedCount > 0) {
        //     let newEntities = create_entities(
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
    pub fn data_columns_changed(&self, record: DataCollectionChangeRecord) {
        self.calculate_drawing_sizes();
        let start = record.index - 1;
        self.update_series_visible(start, record.removed_count, record.added_count);
        if record.removed_count > 0 {
            let end = start + record.removed_count;
            //   for (let i = start; i < end; i++) {
            //     self.series_list[i].freeEntities(0);
            //   }
            //   self.series_list.remove_range(start, end);
        }

        if record.added_count > 0 {
            //   let list = create_series_list(start, start + record.addedCount);
            //   self.series_list.insertAll(start, list);
        }
        self.update_legend_content();
    }

    /// Called when [data_table] has been changed.
    pub fn data_table_changed(&self) {
        self.calculate_drawing_sizes();
        // self.series_list = self.create_series_list(0, self.data_table.columns.length - 1);
        unimplemented!()
    }

    pub fn update_series_visible(&self, index: usize, removed_count: usize, added_count: usize) {
        if removed_count > 0 {
            // self.series_states.remove_range(index, index + removed_count);
            unimplemented!()
        }
        if added_count > 0 {
            // let list = List.filled(added_count, Visibility::showing);
            // self.series_states.insertAll(index, list);
            unimplemented!()
        }
    }

    /// Draws the current animation frame.
    ///
    /// If [time] is `null`, draws the last frame (i.e. no animation).
    pub fn draw_frame(&mut self, time: Option<f64>) {
        let percent = 1.0;
        // let duration = options["animation"]["duration"];
        if let None = self.animation_start_time {
            self.animation_start_time = time
        }

        // if (duration > 0 && time != null) {
        //   percent = (time - animation_start_time) / duration;
        // }

        // if (percent >= 1.0) {
        //   percent = 1.0;

        //   // Update the visibility states of all series before the last frame.
        //   for (let i = series_states.length - 1; i >= 0; i--) {
        //     if (series_states[i] == Visibility::showing) {
        //       series_states[i] = Visibility::shown;
        //     } else if (series_states[i] == Visibility::hiding) {
        //       series_states[i] = Visibility::hidden;
        //     }
        //   }
        // }

        // context.fillStyle = options["backgroundColor"];
        // context.fillRect(0, 0, _width, _height);
        // series_context.clearRect(0, 0, _width, _height);
        // _drawSeries(_easingFunction(percent));
        // context.drawImageScaled(axes_context.canvas, 0, 0, _width, _height);
        // context.drawImageScaled(series_context.canvas, 0, 0, _width, _height);
        // _drawTitle();

        // if (percent < 1.0) {
        //   animation_frame_id = window.requestAnimationFrame(draw_frame);
        // } else if (time != null) {
        //   _animationEnd();
        // }
    }

    /// Draws the chart title using the main rendering context.
    pub fn draw_title(&self) {
        // let title = options["title"];
        // if (title["text"] == null) return;

        // let x = (title_box.left + title_box.right) ~/ 2;
        // let y = title_box.bottom - title_padding;
        // context
        //   ..font = get_font(title["style"])
        //   ..fillStyle = title["style"]["color"]
        //   ..textAlign = "center"
        //   ..fillText(title["text"], x, y);
    }

    pub fn initialize_legend(&self) {
        // let n = get_legend_labels().length;
        // series_states = Vec<VISIBILITY>.filled(n, Visibility::showing,
        //     growable: true);

        // if (self.legend != null) {
        //   self.legend.remove();
        //   self.legend = null;
        // }

        // if (options["legend"]["position"] == "none") return;

        // self.legend = create_tooltip_or_legend(options["legend"]["style"]);
        // self.legend.style.lineHeight = "180%";
        // update_legend_content();
        // container.append(self.legend);
    }

    /// This must be called after [calculate_drawing_sizes] as we need to know
    /// where the title is in order to position the legend correctly.
    pub fn position_legend(&self) {
        // if (self.legend == null) return;

        // let s = self.legend.style;
        // switch (options["legend"]["position"]) {
        //   case "right":
        //     s.right = "${chart_padding}px";
        //     s.top = "50%";
        //     s.transform = "translateY(-50%)";
        //     break;
        //   case "bottom":
        //     let bottom = chart_padding;
        //     if (options["title"]["position"] == "below" && title_box.height > 0) {
        //       bottom += title_box.height;
        //     }
        //     s.bottom = "${bottom}px";
        //     s.left = "50%";
        //     s.transform = "translateX(-50%)";
        //     break;
        //   case "left":
        //     s.left = "${chart_padding}px";
        //     s.top = "50%";
        //     s.transform = "translateY(-50%)";
        //     break;
        //   case "top":
        //     let top = chart_padding;
        //     if (options["title"]["position"] == "above" && title_box.height > 0) {
        //       top += title_box.height;
        //     }
        //     s.top = "${top}px";
        //     s.left = "50%";
        //     s.transform = "translateX(-50%)";
        //     break;
        // }
    }

    pub fn update_legend_content(&self) {
        let labels = self.get_legend_labels();
        // let formatter =
        //     options["legend"]["labelFormatter"] ?? default_label_formatter;
        // legend_item_subscription_tracker.clear();
        // legend.innerHtml = "";
        // for (let i = 0; i < labels.length; i++) {
        //   let label = labels[i];
        //   let formattedLabel = formatter(label);
        //   let e = create_tooltip_or_legendItem(get_color(i), formattedLabel);
        //   if (label != formattedLabel) {
        //     e.title = label;
        //   }
        //   e.style.cursor = "pointer";
        //   e.style.userSelect = "none";
        //   legend_item_subscription_tracker
        //     ..add(e.onClick.listen(legend_item_click))
        //     ..add(e.onMouseOver.listen(legend_item_mouse_over))
        //     ..add(e.onMouseOut.listen(legend_item_mouse_out));

        //   let state = series_states[i];
        //   if (state == Visibility::hidden ||
        //       state == Visibility::hiding) {
        //     e.style.opacity = ".4";
        //   }

        //   // Display the items in one row if the legend"s position is "top" or
        //   // "bottom".
        //   let pos = options["legend"]["position"];
        //   if (pos == "top" || pos == "bottom") {
        //     e.style.display = "inline-block";
        //   }
        //   self.legend.append(e);
        // }
    }

    pub fn get_legend_labels(&self) -> Vec<String> {
        // data_table.columns.skip(1).map((e) => e.name).toList();
        unimplemented!()
    }

    pub fn legend_item_click(&self, e: MouseEvent) {
        if !self.is_interactive() {
            return;
        }

        // let item = e.currentTarget as Element;
        // let index = item.parent.children.indexOf(item);

        // if (series_states[index] == Visibility::shown) {
        //   series_states[index] = Visibility::hiding;
        //   item.style.opacity = ".4";
        // } else {
        //   series_states[index] = Visibility::showing;
        //   item.style.opacity = "";
        // }

        // series_visibility_changed(index);
        self.start_animation();
    }

    pub fn legend_item_mouse_over(&self, e: MouseEvent) {
        if !self.is_interactive() {
            return;
        }

        // let item = e.currentTarget as Element;
        // focused_series_index = item.parent.children.indexOf(item);
        // draw_frame(null);
    }

    pub fn legend_item_mouse_out(&self, e: MouseEvent) {
        if !self.is_interactive() {
            return;
        }

        // focused_series_index = -1;
        // draw_frame(null);
    }

    /// Called when the visibility of a series is changed.
    ///
    /// [index] is the index of the affected series.
    ///
    /// To be overridden.
    pub fn series_visibility_changed(&self, index: usize) {}

    /// Returns the index of the point group/bar group/pie/... near the position
    /// specified by [x] and [y].
    ///
    /// To be overridden.
    pub fn get_entity_group_index(&self, x: f64, num: f64) -> i64 {
        -1
    }

    /// Handles `mousemove` or `touchstart` events to highlight appropriate
    /// points/bars/pies/... as well as update the tooltip.
    pub fn mouse_move(&self, e: MouseEvent) {
        // if !self.is_interactive() || e.buttons != 0 {
        //     return;
        // }

        // let rect = context.canvas.getBoundingClientRect();
        // let x = e.client.x - rect.left;
        // let y = e.client.y - rect.top;
        // let index = _getEntityGroupIndex(x, y);

        // if (index != focused_entity_index) {
        //   focused_entity_index = index;
        //   draw_frame(null);
        //   if (index >= 0) {
        //     update_tooltip_content();
        //     tooltip.hidden = false;
        //     let p = getTooltipPosition();
        //     tooltip.style.transform = "translate(${p.x}px, ${p.y}px)";
        //   } else {
        //     tooltip.hidden = true;
        //   }
        // }
    }

    pub fn initialize_tooltip(&self) {
        // if self.tooltip != null {
        //   tooltip.remove();
        //   tooltip = null;
        // }

        // let opt = options["tooltip"];
        // if (!opt["enabled"]) return;

        // tooltip_label_formatter = opt["labelFormatter"] ?? default_label_formatter;
        // tooltip_value_formatter = opt["valueFormatter"] ?? _defaultValueFormatter;
        // tooltip = create_tooltip_or_legend(opt["style"])
        //   ..hidden = true
        //   ..style.left = "0"
        //   ..style.top = "0"
        //   ..style.boxShadow = "4px 4px 4px rgba(0,0,0,.25)"
        //   ..style.transition = "transform .4s cubic-bezier(.4,1,.4,1)";
        // container.append(_tooltip);

        // mouse_move_sub?.cancel();
        // mouse_move_sub = container.onMouseMove.listen(_mouseMove);
    }

    pub fn update_tooltip_content(&self) {
        // let columnCount = data_table.columns.length;
        // let row = data_table.rows[focused_entity_index];
        // tooltip.innerHtml = "";

        // // Tooltip title.
        // tooltip.append(DivElement()
        //   ..text = row[0]
        //   ..style.padding = "4px 12px"
        //   ..style.fontWeight = "bold");

        // // Tooltip items.
        // for (let i = 1; i < columnCount; i++) {
        //   let state = series_states[i - 1];
        //   if (state == Visibility::hidden) continue;
        //   if (state == Visibility::hiding) continue;

        //   let series = series_list[i - 1];
        //   let value = row[i];
        //   if (value == null) continue;

        //   value = tooltip_value_formatter(value);
        //   let label = tooltip_label_formatter(series.name);

        //   let e = create_tooltip_or_legendItem(
        //       series.color, "$label: <strong>$value</strong>");
        //   tooltip.append(e);
        // }
    }

    // /// Creates an absolute positioned div with styles specified by [style].
    // pub fn create_tooltip_or_legend(&self, style: HashMap<String, String>) -> Element {
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

    // pub fn create_tooltip_or_legend_item(&self, color: String, text: String) -> Element {
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

    pub fn start_animation(&self) {
        // animation_frame_id = window.requestAnimationFrame(draw_frame);
    }

    pub fn stop_animation(&self) {
        // animation_start_time = null;
        // if self.animation_frame_id != 0 {
        //     //   window.cancelAnimationFrame(animation_frame_id);
        //     self.animation_frame_id = 0;
        // }
    }

    // @Deprecated("Use [isAnimating] instead")
    pub fn animating(&self) -> bool {
        self.is_animating()
    }

    /// Whether the chart is animating.
    pub fn is_animating(&self) -> bool {
        self.animation_start_time != None
    }

    /// Whether the chart is interactive.
    ///
    /// This property returns `false` if the chart is animating or there are no
    /// series to draw.
    pub fn is_interactive(&self) -> bool {
        !self.is_animating() && self.series_list.len() != 0
    }

    // /// The element that contains this chart.
    // let Element container;

    /// Disposes of resources used by this chart. The chart will become unusable
    /// until [draw] is called again.
    ///
    /// Be sure to call this method when the chart is no longer used to afn any
    /// memory leaks.
    ///
    /// @mustCallSuper
    pub fn dispose(&self) {
        // // This causes [canHandleInteraction] to be `false`.
        // series_list = null;
        // mouse_move_sub?.cancel();
        // mouse_move_sub = null;
        // data_tableSubscriptionTracker.clear();
        // legend_item_subscription_tracker.clear();
    }

    /// Draws the chart given a data table [dataTable] and an optional set of
    /// options [options].
    pub fn draw(&mut self, data_table: DataStream<'a, M, D>, easing: Easing) {
        self.dispose();
        self.data_table = data_table;
        // data_tableSubscriptionTracker
        //   ..add(dataTable.onCellChange.listen(_data_cell_changed))
        //   ..add(dataTable.onColumnsChange.listen(_dataColumnsChanged))
        //   ..add(dataTable.onRowsChange.listen(_dataRowsChanged));
        // options = mergeMaps(default_options, options);
        // self.easing_function = get_easing(options["animation"]["easing"]);
        self.initialize_legend();
        self.initialize_tooltip();
        // self.resize(container.clientWidth, container.clientHeight, true);
    }

    /// Resizes the chart to fit the new size of the container.
    /// w = container.clientWidth;
    /// h = container.clientHeight;
    // [bool force_redraw = false]
    pub fn resize(&self, w: f64, h: f64, force_redraw: bool) {
        let mut force_redraw = force_redraw;

        if w == 0_f64 || h == 0_f64 {
            return;
        }

        let width = *self.width.borrow();
        let height = *self.height.borrow();
        if w != width || h != height {
            *self.width.borrow_mut() += w;
            *self.height.borrow_mut() += h;
            force_redraw = true;

            //   let dpr = window.devicePixelRatio;
            //   let scaledW = (w * dpr).round();
            //   let scaledH = (h * dpr).round();

            self.set_canvas_size(&self.context);
            self.set_canvas_size(&self.axes_context);
            self.set_canvas_size(&self.series_context);
        }

        if force_redraw {
            self.stop_animation();
            self.data_table_changed();
            self.position_legend();
            self.update(Default::default());
        }
    }

    fn set_canvas_size(&self, ctx: &Option<C>) {
        // Scale the drawing canvas by [dpr] to ensure sharp rendering on
        // high pixel density displays.
        if let Some(ctx) = ctx {
            // ctx.canvas
            //   ..style.width = "${w}px"
            //   ..style.height = "${h}px"
            //   ..width = scaledW
            //   ..height = scaledH;
            // ctx.set_transform(dpr, 0, 0, dpr, 0, 0);
        }
    }

    /// Updates the chart.
    ///
    ///  This method should be called after [dataTable] has been modified.
    // TODO: handle updates while animation is happening.
    pub fn update(&self, options: HashMap<String, String>) {
        if *self.width.borrow() == 0_f64 || *self.height.borrow() == 0_f64 {
            return;
        }

        // if (options != null) {
        //   self.options = mergeMaps(self.options, options);
        // }

        // This call is redundant for row and column changes but necessary for
        // cell changes.
        self.calculate_drawing_sizes();
        self.update_series(0);
        // self.axes_context.clearRect(0, 0, self.width, self.height);
        self.draw_axes_and_grid();
        self.start_animation();
        unimplemented!()
    }
}

impl<'a, C, E, M, D, O> Chart<E> for BaseChart<'a, C, E, M, D, O>
where
    C: CanvasContext,
    E: Entity,
    M: fmt::Display,
    D: fmt::Display,
    O: BaseOption<'a>,
{
    fn calculate_drawing_sizes(&self) {
        todo!()
    }

    fn create_entity(
        &self,
        series_index: usize,
        entity_index: usize,
        value: String,
        color: String,
        highlight_color: String,
    ) -> E {
        todo!()
    }

    fn get_tooltip_position(&self) -> Point<f64> {
        todo!()
    }
}
