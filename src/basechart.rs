#![allow(unused_variables)]
#![allow(clippy::explicit_counter_loop, clippy::float_cmp)]

use animate::easing::EasingFunction;
use animate::{CanvasContext, Color, Point, Rect, RgbColor, Size, TextStyle, TextWeight};
use dataflow::*;
use std::{cell::RefCell, collections::HashMap, fmt};

use super::*;

// channel_states moved to channels

#[derive(Default, Clone)]
pub struct BaseChartProperties {
    /// ID of the current animation frame.
    pub animation_frame_id: usize,

    /// The starting time of an animation cycle.
    pub animation_start_time: Option<i64>,

    // dataTableSubscriptionTracker: StreamSubscriptionTracker, // = StreamSubscriptionTracker();
    pub easing: Option<EasingFunction>,

    /// The chart"s width.
    pub height: f64,

    /// The chart"s height.
    pub width: f64,

    /// Index of the highlighted point group/bar group/pie/...
    pub focused_entity_index: i64, // = -1;

    pub focused_channel_index: i64, // = -1;

    pub entity_value_formatter: Option<ValueFormatter>,

    /// The legend element.
    legend: Option<bool>, //Element,

    // /// The subscription tracker for legend items" events.
    // legendItemSubscriptionTracker: StreamSubscriptionTracker, // = StreamSubscriptionTracker();

    // mouseMoveSub: StreamSubscription,
    /// The tooltip element. To position the tooltip, change its transform CSS.
    tooltip: Option<bool>, //Element,
    /// The function used to format channel names to display in the tooltip.
    pub tooltip_label_formatter: Option<LabelFormatter>,

    /// The function used to format channel data to display in the tooltip.
    pub tooltip_value_formatter: Option<ValueFormatter>,

    /// Bounding box of the channel and axes.
    pub area: Rect<f64>,

    /// Bounding box of the chart title.
    pub title_box: Rect<f64>,
}

/// Base class for all charts.
#[derive(Default, Clone)]
pub struct BaseChart<C, E, M, D, O>
where
    C: CanvasContext,
    E: Entity,
    M: fmt::Display,
    D: fmt::Display + Copy,
    O: BaseOption,
{
    pub props: RefCell<BaseChartProperties>,
    /// The data table that stores chart data
    /// Row 0 contains column names.
    /// Column 0 contains x-axis/pie labels.
    /// Column 1..n - 1 contain channel data.
    pub data: DataStream<M, D>,

    /// The drawing options initialized in the constructor.
    pub options: O,

    /// The main rendering context.
    pub context: Option<C>,

    /// The rendering context for the axes.
    // pub ctx: Option<C>,

    /// The rendering context for the channel.
    // pub ctx: Option<C>,

    /// The precalcuated datas
    /// A ChartChannel keep track of the visibility of the channel.
    pub channels: RefCell<Vec<ChartChannel<E>>>,
}

impl<C, E, M, D, O> BaseChart<C, E, M, D, O>
where
    C: CanvasContext,
    E: Entity,
    M: fmt::Display,
    D: fmt::Display + Copy,
    O: BaseOption,
{
    // /// The element that contains this chart.
    // container: Element;

    /// Creates a chart given a container.
    ///
    /// If the CSS position of [container] is "static", it will be changed to
    /// "relative".
    pub fn new(options: O) -> Self {
        Self {
            props: Default::default(),
            data: Default::default(),
            options,
            context: None,
            channels: RefCell::new(Vec::new()),
        }
    }

    /// Creates a new color by combining the R, G, B components of [color] with
    /// [alpha] from 0 to 1.
    /// TODO: There are question about set the alpha or change from existing alpha
    ///
    pub fn change_fill_alpha(&self, value: &Fill, alpha: f64) -> Fill {
        if !(0. ..=1.).contains(&alpha) {
            panic!("Wrong alpha value {}", alpha);
        }

        match value {
            Fill::Solid(value) => {
                let alpha = (alpha * 0xFF as f64).round() as u8;
                let color: RgbColor = (*value).into();
                Fill::Solid(Color::rgba(color.red, color.green, color.blue, alpha))
            }
            Fill::Gradient(gradient) => {
                let gradient = gradient.clone();
                {
                    let mut stops = gradient.stops.borrow_mut();
                    let alpha = (alpha * 0xFF as f64).round() as u8;
                    for stop in stops.iter_mut() {
                        let color: RgbColor = (stop.color).into();
                        stop.color = Color::rgba(color.red, color.green, color.blue, alpha);
                    }
                }
                Fill::Gradient(gradient)
            },
            Fill::None => Fill::None,
        }
    }

    pub fn get_fill(&self, index: usize) -> Fill {
        let colors = self.options.colors();
        let color = colors.get(index % colors.len()).unwrap();
        color.clone()
    }

    pub fn get_highlight_color(&self, value: &Fill) -> Fill {
        self.change_fill_alpha(value, 0.5)
    }

    /// Called when the animation ends.
    pub fn animation_end(&self) {
        let mut props = self.props.borrow_mut();
        props.animation_start_time = None;

        let channels = self.channels.borrow();
        for channel in channels.iter() {
            for entity in &channel.entities {
                //         entity.save();
            }
        }

        let animation = self.options.animation();

        if let Some(callback) = animation.on_end {
            callback();
        }
    }

    /// Event handler for [DataTable.onCellChanged].
    ///
    /// NOTE: This method only handles the case when [record.columnIndex] >= 1;
    pub fn data_cell_changed(&self, record: DataCellChangeRecord<D>) {
        if record.column_index >= 1 {
            //   let f = entity_value_formatter != null && record.newValue != null
            //       ? entity_value_formatter(record.newValue)
            //       : null;
            //   channels[record.columnIndex - 1].entities[record.rowIndex]
            //     ..value = record.newValue
            //     ..formatted_value = f;
        }
    }

    /// Event handler for [DataTable.onRowsChanged].
    pub fn data_rows_changed(&self, record: DataCollectionChangeRecord) {
        // self.calculate_drawing_sizes(ctx);
        let entity_count = self.data.frames.len();
        let removed_end = record.index + record.removed_count;
        let added_end = record.index + record.added_count;
        let channels = self.channels.borrow();
        for channel in channels.iter() {
            // Remove old entities.
            if record.removed_count > 0 {
                // channel.freeEntities(record.index, removedEnd);
                // channel.entities.remove_range(record.index, removedEnd);
            }

            // Insert new entities.
            if record.added_count > 0 {
                // let newEntities = create_entities(
                //     i, record.index, addedEnd, channel.color, channel.highlight_color);
                // channel.entities.insertAll(record.index, newEntities);

                // // Update entity indexes.
                // for (let j = addedEnd; j < entity_count; j++) {
                //     channel.entities[j].index = j;
                // }
            }
        }
    }

    /// Event handler for [DataTable.onColumnsChanged].
    pub fn data_columns_changed(&self, record: DataCollectionChangeRecord) {
        debug!(
            "data_columns_changed remove[{}] add[{}]",
            record.removed_count, record.added_count
        );
        // self.calculate_drawing_sizes(ctx);
        let start = record.index - 1;
        self.update_channel_visible(start, record.removed_count, record.added_count);
        if record.removed_count > 0 {
            let end = start + record.removed_count;
            for idx in start..end {
                // self.channels[idx].freeEntities(0);
            }
            // self.channels.remove_range(start, end);
        }

        if record.added_count > 0 {
            // let list = self.create_channels(start, start + record.added_count);
            //   self.channels.insertAll(start, list);
        }
        self.update_legend_content();
    }

    pub fn update_channel_visible(&self, index: usize, removed_count: usize, added_count: usize) {
        if removed_count > 0 {
            // self.channel_states.remove_range(index, index + removed_count);
        }
        if added_count > 0 {
            // let list = List.filled(added_count, Visibility::showing);
            // self.channel_states.insertAll(index, list);
        }
        unimplemented!()
    }

    /// Draws the chart title using the main rendering context.
    pub fn draw_title(&self, ctx: &C) {
        let title = self.options.title();
        if let Some(text) = &title.text {
            let props = self.props.borrow();
            // let x = ((props.title_box.origin.x + props.title_box.size.width) / 2.).trunc();
            let x = props.title_box.origin.x;
            let y = (props.title_box.origin.y + props.title_box.size.height) - TITLE_PADDING;
            let style = &title.style;

            let fontfamily = match &style.fontfamily {
                Some(val) => val.as_str(),
                None => DEFAULT_FONT_FAMILY,
            };

            ctx.set_font(
                fontfamily,
                style.fontstyle.unwrap_or(TextStyle::Normal),
                TextWeight::Normal,
                style.fontsize.unwrap_or(12.),
            );
            ctx.set_fill_color(title.style.color);
            // ctx.set_text_align(TextAlign::Center);
            ctx.fill_text(text.as_str(), x, y);
        }
    }

    pub fn initialize_legend(&self) {
        let new_len = self.get_legend_labels().len();
        let props = self.props.borrow_mut();

        if let Some(legend) = props.legend {
            //   self.legend.remove();
            //   self.legend = null;
        }

        if let Position::None = self.options.legend().position {
            return;
        }

        // props.legend = self.create_tooltip_or_legend(self.options.legend().style);
        // props.legend.style.lineHeight = "180%";
        self.update_legend_content();
        // container.append(props.legend);
    }

    /// This must be called after [calculate_drawing_sizes] as we need to know
    /// where the title is in order to position the legend correctly.
    pub fn position_legend(&self) {
        // println!("BaseChart position_legend");
        let props = self.props.borrow();
        if let Some(legend) = props.legend {
            // let s = legend.style;
            // switch (self.options.legend().position) {
            // case "right":
            //     s.right = "${CHART_PADDING}px";
            //     s.top = "50%";
            //     s.transform = "translateY(-50%)";
            //     break;
            // case "bottom":
            //     let bottom = CHART_PADDING;
            //     if (self.options.title().position == "below" && title_box.height > 0) {
            //     bottom += title_box.height;
            //     }
            //     s.bottom = "${bottom}px";
            //     s.left = "50%";
            //     s.transform = "translateX(-50%)";
            //     break;
            // case "left":
            //     s.left = "${CHART_PADDING}px";
            //     s.top = "50%";
            //     s.transform = "translateY(-50%)";
            //     break;
            // case "top":
            //     let top = CHART_PADDING;
            //     if (self.options.title().position == "above" && title_box.height > 0) {
            //     top += title_box.height;
            //     }
            //     s.top = "${top}px";
            //     s.left = "50%";
            //     s.transform = "translateX(-50%)";
            //     break;
            // }
        }
    }

    pub fn update_legend_content(&self) {
        let labels = self.get_legend_labels();
        // let formatter =
        //     self.options.legend().labelFormatter ?? default_label_formatter;
        // legend_item_subscription_tracker.clear();
        // legend.innerHtml = "";
        // for (let i = 0; i < labels.len(); i++) {
        //   let label = labels[i];
        //   let formattedLabel = formatter(label);
        //   let e = create_tooltip_or_legendItem(self.get_color(i), formattedLabel);
        //   if (label != formattedLabel) {
        //     e.title = label;
        //   }
        //   e.style.cursor = "pointer";
        //   e.style.userSelect = "none";
        //   legend_item_subscription_tracker
        //     ..add(e.onClick.listen(legend_item_click))
        //     ..add(e.onMouseOver.listen(legend_item_mouse_over))
        //     ..add(e.onMouseOut.listen(legend_item_mouse_out));

        //   let state = channel_states[i];
        //   if (state == Visibility::hidden ||
        //       state == Visibility::hiding) {
        //     e.style.opacity = ".4";
        //   }

        //   // Display the items in one row if the legend"s position is "top" or
        //   // "bottom".
        //   let pos = self.options.legend().position;
        //   if (pos == "top" || pos == "bottom") {
        //     e.style.display = "inline-block";
        //   }
        //   self.legend.append(e);
        // }
    }

    pub fn get_legend_labels(&self) -> Vec<String> {
        self.data
            .meta
            .iter()
            .map(|channel| channel.name.to_string())
            .collect()
    }

    pub fn legend_item_click(&self, e: MouseEvent) {
        if !self.is_interactive() {
            return;
        }

        // let item = e.currentTarget as Element;
        // let index = item.parent.children.indexOf(item);

        // if (channel_states[index] == Visibility::shown) {
        //   channel_states[index] = Visibility::hiding;
        //   item.style.opacity = ".4";
        // } else {
        //   channel_states[index] = Visibility::showing;
        //   item.style.opacity = "";
        // }

        // channel_visibility_changed(index);
        self.start_animation();
    }

    pub fn legend_item_mouse_over(&self, e: MouseEvent) {
        if !self.is_interactive() {
            return;
        }

        // let item = e.currentTarget as Element;
        // focused_channel_index = item.parent.children.indexOf(item);
        // draw_frame(null);
    }

    pub fn legend_item_mouse_out(&self, e: MouseEvent) {
        if !self.is_interactive() {
            return;
        }

        // focused_channel_index = -1;
        // draw_frame(null);
    }

    /// Called when the visibility of a channel is changed.
    ///
    /// [index] is the index of the affected channel.
    ///
    pub fn channel_visibility_changed(&self, index: usize) {}

    /// Returns the index of the point group/bar group/pie/... near the position
    /// specified by [x] and [y].
    ///
    pub fn get_entity_group_index(&self, x: f64, y: f64) -> i64 {
        -1
    }

    /// Handles `mousemove` or `touchstart` events to highlight appropriate
    /// points/bars/pies/... as well as update the tooltip.
    pub fn mouse_move(&self, e: MouseEvent) {
        // if !self.is_interactive() || e.buttons != 0 {
        //     return;
        // }

        // let rect = ctx.canvas.getBoundingClientRect();
        // let x = e.client.x - rect.left;
        // let y = e.client.y - rect.top;
        // let index = getEntityGroupIndex(x, y);

        // if index != focused_entity_index {
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
        // println!("BaseChart initialize_tooltip");
        // if self.tooltip != null {
        //   tooltip.remove();
        //   tooltip = null;
        // }

        // let opt = self.options.tooltip;
        // if (!opt["enabled"]) return;

        // tooltip_label_formatter = opt["labelFormatter"] ?? default_label_formatter;
        // tooltip_value_formatter = opt["value_formatter"] ?? default_value_formatter;
        // tooltip = create_tooltip_or_legend(opt.style.)
        //   ..hidden = true
        //   ..style.left = "0"
        //   ..style.top = "0"
        //   ..style.boxShadow = "4px 4px 4px rgba(0,0,0,.25)"
        //   ..style.transition = "transform .4s cubic-bezier(.4,1,.4,1)";
        // container.append(tooltip);

        // mouse_move_sub?.cancel();
        // mouse_move_sub = container.onMouseMove.listen(mouseMove);
    }

    pub fn update_tooltip_content(&self) {
        let props = self.props.borrow();
        let column_count = self.data.meta.len();
        let row = self.data.frames.get(props.focused_entity_index as usize);
        // tooltip.innerHtml = "";

        // // Tooltip title
        // tooltip.append(DivElement()
        //   ..text = row[0]
        //   ..style.padding = "4px 12px"
        //   ..style.fontWeight = "bold");

        // Tooltip items.
        for idx in 1..column_count {
            // let state = props.channel_states.get(idx - 1);
            //   if (state == Visibility::hidden) continue;
            //   if (state == Visibility::hiding) continue;

            //   let channel = channels[i - 1];
            //   let value = row[i];
            //   if (value == null) continue;

            //   value = tooltip_value_formatter(value);
            //   let label = tooltip_label_formatter(channel.name);

            //   let e = create_tooltip_or_legendItem(
            //       channel.color, "$label: <strong>$value</strong>");
            //   tooltip.append(e);
        }
    }

    /// Creates an absolute positioned div with styles specified by [style].
    // TODO: retusns Element
    pub fn create_tooltip_or_legend(&self, style: HashMap<String, String>) -> Option<bool> {
        // return DivElement()
        //   ..style.background_color = style["background_color"]
        //   ..style.borderColor = style["borderColor"]
        //   ..style.borderStyle = "solid"
        //   ..style.borderWidth = "${style["borderWidth"]}px"
        //   ..style.color = style["color"]
        //   ..style.fontFamily = style["fontFamily"]
        //   ..style.font_size = "${style["font_size"]}px"
        //   ..style.fontStyle = style["fontStyle"]
        //   ..style.position = "absolute";
        unimplemented!()
    }

    // TODO: return Element
    pub fn create_tooltip_or_legend_item(&self, color: String, text: String) -> Option<bool> {
        // let e = DivElement()
        //   ..innerHtml = "<span></span> $text"
        //   ..style.padding = "4px 12px";
        // e.children.first.style
        //   ..background_color = color
        //   ..display = "inline-block"
        //   ..width = "12px"
        //   ..height = "12px";
        // return e;
        unimplemented!()
    }

    // real drawing
    pub fn start_animation(&self) {
        // println!("BaseChart start_animation");
        // animation_frame_id = window.requestAnimationFrame(draw_frame);
    }

    pub fn stop_animation(&self) {
        // println!("BaseChart stop_animation");
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
        self.props.borrow().animation_start_time.is_some()
    }

    /// Whether the chart is interactive.
    ///
    /// This property returns `false` if the chart is animating or there are no
    /// channel to draw.
    pub fn is_interactive(&self) -> bool {
        !self.is_animating() && self.channels.borrow().len() != 0
    }

    /// Disposes of resources used by this chart. The chart will become unusable
    /// until [draw] is called again.
    ///
    /// Be sure to call this method when the chart is no longer used to afn any
    /// memory leaks.
    ///
    /// @mustCallSuper
    pub fn dispose(&self) {
        // println!("BaseChart dispose");
        // // This causes [canHandleInteraction] to be `false`.
        // channels = null;
        // mouse_move_sub?.cancel();
        // mouse_move_sub = null;
        // data_tableSubscriptionTracker.clear();
        // legend_item_subscription_tracker.clear();
    }

    pub fn calculate_percent(&self, time: Option<i64>) -> f64 {
        let mut percent = 1.0;
        let mut props = self.props.borrow_mut();

        if props.animation_start_time.is_none() {
            props.animation_start_time = time
        }

        if let Some(time) = time {
            let duration = self.options.animation().duration;
            if duration > 0 {
                percent = (time - props.animation_start_time.unwrap()) as f64 / duration as f64;
            }
        }
        percent
    }
}

impl<C, E, M, D, O> Chart<C, M, D, E> for BaseChart<C, E, M, D, O>
where
    C: CanvasContext,
    E: Entity,
    M: fmt::Display,
    D: fmt::Display + Copy,
    O: BaseOption,
{
    /// Calculates various drawing sizes.
    ///
    /// Overriding methods must call this method first to have [area]
    /// calculated.
    ///
    fn calculate_drawing_sizes(&self, ctx: &C) {
        let title = self.options.title();

        let mut title_x = 0.0;
        let mut title_y = 0.0;
        let mut title_w = 0.0;
        let mut title_h = 0.0;

        let mut props = self.props.borrow_mut();

        let prepare_title = match title.position {
            Position::Above => {
                title_h = title.style.fontsize.unwrap_or(12.) + 2.0 * TITLE_PADDING;
                title_y = CHART_PADDING;
                props.area.origin.y += title_h + CHART_TITLE_MARGIN;
                props.area.size.height -= title_h + CHART_TITLE_MARGIN;
                true
            }
            Position::Middle => {
                title_h = title.style.fontsize.unwrap_or(12.) + 2.0 * TITLE_PADDING;
                title_y = f64::floor((props.height - title_h) / 2.0);
                true
            }
            Position::Below => {
                title_h = title.style.fontsize.unwrap_or(12.) + 2.0 * TITLE_PADDING;
                title_y = props.height - title_h - CHART_PADDING;
                props.area.size.height -= title_h + CHART_TITLE_MARGIN;
                true
            }
            _ => false,
        };

        if prepare_title {
            if let Some(text) = &title.text {
                let style = &title.style;

                let fontfamily = match &style.fontfamily {
                    Some(val) => val.as_str(),
                    None => DEFAULT_FONT_FAMILY,
                };

                ctx.set_font(
                    fontfamily,
                    style.fontstyle.unwrap_or(TextStyle::Normal),
                    TextWeight::Normal,
                    style.fontsize.unwrap_or(12.),
                );
                title_w = ctx.measure_text(text.as_str()).width.round() + 2. * TITLE_PADDING;
                title_x = ((props.width - title_w - 2. * TITLE_PADDING) / 2.).trunc();
            }

            // Consider the title.
            props.title_box = Rect {
                origin: Point::new(title_x, title_y),
                size: Size::new(title_w, title_h),
            };
        }

        // Consider the legend.
        if props.legend.is_some() {
            //   let lwm = self.legend.offset_width + legend_margin;
            //   let lhm = self.legend.offset_height + legend_margin;
            let opt = self.options.legend();
            match opt.position {
                Position::Right => {
                    // props.area.size.width -= lwm;
                }
                Position::Bottom => {
                    // props.area.size.height -= lhm;
                }
                Position::Left => {
                    // props.area.origin.x += lwm;
                    // props.area.size.width -= lwm;
                }
                Position::Top => {
                    // props.area.origin.y += lhm;
                    // props.area.size.height -= lhm;
                }
                _ => {}
            }
        }
    }

    fn set_stream(&mut self, stream: DataStream<M, D>) {
        error!("set stream");
    }

    /// Draws the chart given a data table [dataTable] and an optional set of
    /// options [options].
    // TODO: handle updates while animation is happening.
    fn draw(&self, ctx: &C) {
        // TODO: use this not_eq
        // let props = self.props.borrow();
        // if props.width == 0_f64 || props.height == 0_f64 {
        //     return;
        // }

        // data_tableSubscriptionTracker
        //   ..add(dataTable.onCellChange.listen(data_cell_changed))
        //   ..add(dataTable.onColumnsChange.listen(dataColumnsChanged))
        //   ..add(dataTable.onRowsChange.listen(data_rows_changed));

        // self.ctx.clearRect(0, 0, self.width, self.height);
    }

    /// Resizes just only change size state for chart and do not resize the container/canvas.
    fn resize(&self, w: f64, h: f64) {
        // println!("BaseChart resize {} {}", w, h);
        if w == 0_f64 || h == 0_f64 {
            println!("BaseChart resize OOOPS");
            return;
        }

        let mut props = self.props.borrow_mut();
        if w != props.width || h != props.height {
            props.width = w;
            props.height = h;
            // force_redraw = true; // now_eq
        }

        props.area = Rect {
            origin: Point::new(CHART_PADDING, CHART_PADDING),
            size: Size::new(
                props.width - 2.0 * CHART_PADDING,
                props.height - 2.0 * CHART_PADDING,
            ),
        };
    }

    /// Draws the axes and the grid.
    ///
    fn draw_axes_and_grid(&self, ctx: &C) {}

    /// Updates the channel at index [index]. If [index] is `null`, updates all
    /// channel.
    ///
    fn update_channel(&self, _: usize) {}

    // println!("SIZE {} {}", width, height);
    // println!("BACKGROUND {}", self.options.background());

    /// Draws the current animation frame.
    ///
    /// If [time] is `null`, draws the last frame (i.e. no animation).
    fn draw_frame(&self, ctx: &C, time: Option<i64>) {
        let props = self.props.borrow();
        let width = props.width;
        let height = props.height;

        ctx.clear_rect(0.0, 0.0, width, height);

        match self.options.background() {
            Fill::Solid(color) => {
                ctx.set_fill_color(*color);
                ctx.fill_rect(0., 0., width, height);
            }
            Fill::Gradient(gradient) => {
                ctx.set_fill_gradient(gradient);
                ctx.fill_rect(0., 0., width, height);
            }
            Fill::None => {}
        }
    }

    /// Draws the channel given the current animation percent [percent].
    ///
    /// If this method returns `false`, the animation is continued until [percent]
    /// reaches 1.0.
    ///
    /// If this method returns `true`, the animation is stopped immediately.
    /// This is useful as there are cases where no animation is expected.
    /// In those cases, the overriding method will return `true` to stop the
    /// animation.
    ///
    fn draw_channels(&self, ctx: &C, percent: f64) -> bool {
        error!("draw_channels");
        false
    }

    fn create_entity(
        &self,
        channel_index: usize,
        entity_index: usize,
        value: Option<D>,
        color: Fill,
        highlight_color: Fill,
    ) -> E {
        todo!()
    }

    fn create_entities(
        &self,
        channel_index: usize,
        start: usize,
        end: usize,
        color: Fill,
        highlight: Fill,
    ) -> Vec<E> {
        error!("create_entities");
        Vec::new()
    }

    fn create_channels(&self, start: usize, end: usize) {
        error!("create_channels");
    }

    fn get_tooltip_position(&self, tooltip_width: f64, tooltip_height: f64) -> Point<f64> {
        todo!()
    }
}
