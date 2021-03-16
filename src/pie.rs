#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use animate::easing::{get_easing, Easing};
use dataflow::*;
use primitives::{
    palette, BaseLine, CanvasContext, Color, Point, TextAlign, TextStyle, TextWeight,
};
use std::{borrow::Borrow, cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::*;

/// A pie in a pie chart.
#[derive(Default, Clone)]
pub struct PieEntity<D> {
    // Chart chart,
    color: Color,
    highlight_color: Color,
    // formatted_value: String,
    index: usize,
    old_value: Option<D>,
    value: Option<D>,

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

impl<D> PieEntity<D> {
    pub fn is_empty(&self) -> bool {
        self.start_angle == self.end_angle
    }

    fn contains_point(&self, p: Point<f64>) -> bool {
        // p -= center;
        let mag = p.distance_to(Point::default()); //p.magnitude();
        if mag > self.outer_radius || mag < self.inner_radius {
            return false;
        }

        let mut angle = f64::atan2(p.y, p.x);

        //TODO: complete it
        // let chartStartAngle = (chart as dynamic).start_angle;

        // // Make sure [angle] is in range [chartStartAngle]..[chartStartAngle] + TAU.
        // angle = (angle - chartStartAngle) % TAU + chartStartAngle;

        // If counterclockwise, make sure [angle] is in range
        // [start] - 2*pi..[start].
        if self.start_angle > self.end_angle {
            angle -= TAU;
        }

        if self.start_angle <= self.end_angle {
            // Clockwise.
            utils::is_in_range(angle, self.start_angle, self.end_angle)
        } else {
            // Counterclockwise.
            utils::is_in_range(angle, self.end_angle, self.start_angle)
        }
    }
}

impl<D> Entity for PieEntity<D> {
    fn free(&mut self) {
        // chart = null;
    }

    fn save(&self) {
        // self.old_start_angle = self.start_angle;
        // self.old_end_angle = self.end_angle;
        // self.old_value = self.value;
    }
}

impl<C, D> Drawable<C> for PieEntity<D>
where
    C: CanvasContext,
{
    fn draw(&self, ctx: &C, percent: f64, highlight: bool) {
        let mut a1 = utils::lerp(self.old_start_angle, self.start_angle, percent);
        let mut a2 = utils::lerp(self.old_end_angle, self.end_angle, percent);
        if a1 > a2 {
            let tmp = a1;
            a1 = a2;
            a2 = tmp;
        }
        let center = &self.center;
        if highlight {
            let highlight_outer_radius = HIGHLIGHT_OUTER_RADIUS_FACTOR * self.outer_radius;
            ctx.set_fill_color(self.highlight_color);
            ctx.begin_path();
            ctx.arc(center.x, center.y, highlight_outer_radius, a1, a2, false);
            ctx.arc(center.x, center.y, self.inner_radius, a2, a1, true);
            ctx.fill();
        }

        ctx.set_fill_color(self.color);
        ctx.begin_path();
        ctx.arc(center.x, center.y, self.outer_radius, a1, a2, false);
        ctx.arc(center.x, center.y, self.inner_radius, a2, a1, true);
        ctx.fill();
        ctx.stroke();

        // && chart is PieChart
        // if !self.formatted_value.is_empty() && a2 - a1 > PI / 36.0 {
        //     // let labels = self.chart.options.channel.labels;
        //     // if labels.enabled {
        //     //     let r = 0.25 * self.inner_radius + 0.75 * self.outer_radius;
        //     //     let a = 0.5 * (a1 + a2);
        //     //     let p = utils::polar2cartesian(center, r, a);
        //     //     ctx.set_fill_color(labels.style.color);
        //     //     ctx.fill_text(self.formatted_value.as_str(), p.x, p.y);
        //     // }
        // }
    }
}

#[derive(Default, Clone)]
struct PieChartProperties {
    center: Point<f64>,
    outer_radius: f64,
    inner_radius: f64,

    /// The start angle in radians.
    start_angle: f64,

    /// 1 means clockwise and -1 means counterclockwise.
    direction: i64,
}

pub struct PieChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display + Copy,
{
    props: RefCell<PieChartProperties>,
    base: BaseChart<'a, C, PieEntity<D>, M, D, PieChartOptions<'a>>,
}

impl<'a, C, M, D> PieChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display + Copy + Into<f64> + Ord + Default,
{
    pub fn new(options: PieChartOptions<'a>) -> Self {
        Self {
            props: Default::default(),
            base: BaseChart::new(options),
        }
    }

    fn data_rows_changed(&self, record: DataCollectionChangeRecord) {
        self.base
            .update_channel_visible(record.index, record.removed_count, record.added_count);
        self.base.data_rows_changed(record);
        self.base.update_legend_content();
    }

    fn get_entity_group_index(&self, x: f64, y: f64) -> i64 {
        let p = Point::new(x, y);
        // let entities = channels.first.entities;
        // for (let i = entities.len(); i >= 0; i--) {
        //   let pie = entities[i] as Pie;
        //   if (pie.containsPoint(p)) return i;
        // }
        // return -1;
        unimplemented!()
    }

    pub fn get_legend_labels(&self) -> Vec<String> {
        //self.data.getColumnValues<String>(0)
        unimplemented!()
    }

    fn channel_visibility_changed(&self, index: usize) {
        self.update_channel(0);
    }

    fn update_tooltip_content(&self) {
        // let pie = channels[0].entities[focused_entity_index] as Pie;
        // tooltip.style
        //   ..borderColor = pie.color
        //   ..padding = "4px 12px";
        // let label = tooltip_label_formatter(pie.name);
        // let value = tooltip_value_formatter(pie.value);
        // tooltip.innerHtml = "$label: <strong>$value</strong>";
        unimplemented!()
    }

    // Called when [data_table] has been changed.
    // fn data_changed(&self) {
    //     info!("data_changed");
    //     // self.calculate_drawing_sizes(ctx);
    //     self.create_channels(0, self.base.data.meta.len());
    // }
}

impl<'a, C, M, D> Chart<'a, C, M, D, PieEntity<D>> for PieChart<'a, C, M, D>
where
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display + Copy + Into<f64> + Ord + Default,
{
    fn calculate_drawing_sizes(&self, ctx: &C) {
        self.base.calculate_drawing_sizes(ctx);
        let mut props = self.props.borrow_mut();

        props.center = {
            let rect = &self.base.props.borrow().area;

            let half_w = rect.size.width as i64 >> 1;
            let half_h = rect.size.height as i64 >> 1;
            props.outer_radius = (half_w.min(half_h) as f64) / HIGHLIGHT_OUTER_RADIUS_FACTOR;

            let x = rect.origin.x + half_w as f64;
            let y = rect.origin.y + half_h as f64;
            Point::new(x, y)
        };

        let mut pie_hole = self.base.options.pie_hole;

        if pie_hole > 1.0 {
            pie_hole = 0.0;
        }

        if pie_hole < 0.0 {
            pie_hole = 0.0;
        }

        props.inner_radius = pie_hole * props.outer_radius;

        let opt = &self.base.options.channel;
        let mut baseprops = self.base.props.borrow_mut();
        baseprops.entity_value_formatter = if let Some(formatter) = opt.labels.formatter {
            Some(formatter)
        } else {
            Some(default_value_formatter)
        };

        props.direction = if opt.counterclockwise {
            COUNTERCLOCKWISE
        } else {
            CLOCKWISE
        };

        props.start_angle = utils::deg2rad(opt.start_angle);
    }

    fn set_stream(&mut self, stream: DataStream<'a, M, D>) {
        self.base.data = stream;
        self.create_channels(0, self.base.data.meta.len());
    }

    fn draw(&self, ctx: &C) {
        self.base.dispose();
        // data_tableSubscriptionTracker
        //   ..add(dataTable.onCellChange.listen(data_cell_changed))
        //   ..add(dataTable.onColumnsChange.listen(dataColumnsChanged))
        //   ..add(dataTable.onRowsChange.listen(data_rows_changed));
        // self.easing = get_easing(self.options.animation().easing);
        // self.base.initialize_legend();
        // self.base.initialize_tooltip();
        // self.base.position_legend();

        // This call is redundant for row and column changes but necessary for
        // cell changes.
        self.calculate_drawing_sizes(ctx);
        self.update_channel(0);

        self.draw_frame(ctx, None);
    }

    fn resize(&self, w: f64, h: f64) {
        self.base.resize(w, h);
    }

    /// Draws the axes and the grid.
    ///
    fn draw_axes_and_grid(&self, ctx: &C) {
        self.base.draw_axes_and_grid(ctx);
    }

    /// Draws the current animation frame.
    ///
    /// If [time] is `null`, draws the last frame (i.e. no animation).
    fn draw_frame(&self, ctx: &C, time: Option<i64>) {
        // clear surface
        self.base.draw_frame(ctx, time);

        self.draw_axes_and_grid(ctx);

        let mut percent = self.base.calculate_percent(time);

        if percent >= 1.0 {
            percent = 1.0;

            // Update the visibility states of all channel before the last frame.
            let mut channels = self.base.channels.borrow_mut();
            for channel in channels.iter_mut() {
                if channel.state == Visibility::Showing {
                    channel.state = Visibility::Shown;
                } else if channel.state == Visibility::Hiding {
                    channel.state = Visibility::Hidden;
                }
            }
        }

        let props = self.base.props.borrow();

        let ease = match props.easing {
            Some(val) => val,
            None => get_easing(Easing::Linear),
        };
        self.draw_channels(ctx, ease(percent));
        // ctx.drawImageScaled(ctx.canvas, 0, 0, width, height);
        // ctx.drawImageScaled(ctx.canvas, 0, 0, width, height);
        self.base.draw_title(ctx);

        if percent < 1.0 {
            // animation_frame_id = window.requestAnimationFrame(draw_frame);
        } else if time.is_some() {
            self.base.animation_end();
        }
    }

    fn draw_channels(&self, ctx: &C, percent: f64) -> bool {
        info!("draw_channels");
        ctx.set_line_width(2.);
        ctx.set_stroke_color(palette::WHITE);
        // ctx.set_text_align(TextAlign::Center);
        ctx.set_text_baseline(BaseLine::Middle);

        let channels = self.base.channels.borrow();
        let channel = channels.first().unwrap();
        let labels = &self.base.options.channel.labels.style;

        ctx.set_font(
            labels.fontfamily.unwrap_or(DEFAULT_FONT_FAMILY),
            labels.fontstyle.unwrap_or(TextStyle::Normal),
            TextWeight::Normal,
            labels.fontsize.unwrap_or(12.),
        );

        let baseprops = self.base.props.borrow();
        let mut focused_channel_index = baseprops.focused_channel_index;
        focused_channel_index = -1; //FIXME:
        let mut focused_entity_index = baseprops.focused_entity_index;
        focused_entity_index = -1; //FIXME:

        for entity in channel.entities.iter() {
            if entity.is_empty() && percent == 1.0 {
                continue;
            }
            
            let highlight = entity.index as i64 == focused_channel_index
                || entity.index as i64 == focused_entity_index;
            info!("draw entity");
            entity.draw(ctx, percent, highlight);
        }

        return false;
    }

    fn update_channel(&self, _: usize) {
        info!("update_channel");

        let props = self.props.borrow();
        let mut channels = self.base.channels.borrow_mut();

        let mut idx = 0;
        for channel in channels.iter_mut() {
            if channel.state == Visibility::Showing || channel.state == Visibility::Shown {
                let mut sum: f64 = 0.0;
                // Sum the values of all visible pies.
                for entity in channel.entities.iter() {
                    match entity.value {
                        Some(value) => sum += value.into(),
                        None => {}
                    }
                }

                let mut start_angle = props.start_angle;
                for entity in channel.entities.iter_mut() {
                    match entity.value {
                        Some(value) => {
                            let color = self.base.get_color(idx);
                            entity.index = idx;
                            entity.color = color;
                            entity.highlight_color = self.base.get_highlight_color(color);
                            entity.center = props.center;
                            entity.inner_radius = props.inner_radius;
                            entity.outer_radius = props.outer_radius;
                            entity.start_angle = start_angle;
                            entity.end_angle =
                                start_angle + props.direction as f64 * value.into() * TAU / sum;
                            start_angle = entity.end_angle;

                            let val:f64 = value.into();
                            info!(
                                "update item [{}] {} {}",
                                entity.name,
                                val,
                                props.direction as f64 * value.into() * TAU / sum,                                
                            );
                        }
                        None => {
                            // hole in channel data
                        }
                    }
                }
            }
            idx += 1;
        }
    }

    fn create_entity(
        &self,
        channel_index: usize,
        entity_index: usize,
        value: Option<D>,
        color: Color,
        highlight_color: Color,
    ) -> PieEntity<D> {
        // Override the colors.
        let color = self.base.get_color(entity_index);
        let highlight_color = self.base.change_color_alpha(color, 0.5);

        let stream = &self.base.data;
        let frame = stream.frames.get(entity_index).unwrap();
        let name = format!("{}", frame.metric);

        let props = self.props.borrow();

        let start_angle = props.start_angle;

        // FIXME: should be handled in update_channel
        // if entity_index > 0 {
        //     let channels = self.base.channels.borrow();
        //     let channel = channels.first().unwrap();
        //     let prev = channel.entities.get(entity_index - 1).unwrap();
        //     start_angle = prev.end_angle;
        // }

        // let formatted_value = if value != 0. {
        //     self.entity_value_formatter(value)
        // } else {
        //     null
        // };

        PieEntity {
            index: entity_index,
            old_value: None,
            value,
            // formatted_value,
            name,
            color,
            highlight_color,
            old_start_angle: start_angle,
            old_end_angle: start_angle,
            center: props.center,
            inner_radius: props.inner_radius,
            outer_radius: props.outer_radius,
            start_angle,
            end_angle: start_angle, // To be updated in `update_channel`
        }
    }

    fn create_channels(&self, start: usize, end: usize) {
        let mut start = start;
        let mut result = Vec::new();
        let count = self.base.data.frames.len();
        let meta = &self.base.data.meta;

        while start < end {
            let channel = meta.get(start).unwrap();
            let name = channel.name;
            let color = self.base.get_color(start);
            let highlight = self.base.get_highlight_color(color);

            let entities = self.create_entities(start, 0, count, color, highlight);
            result.push(ChartChannel::new(name, color, highlight, entities));
            start += 1;
        }

        let mut channels = self.base.channels.borrow_mut();
        *channels = result;
    }

    fn create_entities(
        &self,
        channel_index: usize,
        start: usize,
        end: usize,
        color: Color,
        highlight: Color,
    ) -> Vec<PieEntity<D>> {
        let mut start = start;
        let mut result = Vec::new();
        while start < end {
            let frame = self.base.data.frames.get(start).unwrap();
            let value = frame.data.get(channel_index as u64);
            let entity = match frame.data.get(channel_index as u64) {
                Some(value) => {
                    let value = value.clone();
                    self.create_entity(channel_index, start, Some(value), color, highlight)
                }
                None => self.create_entity(channel_index, start, None, color, highlight),
            };

            result.push(entity);
            start += 1;
        }
        result
    }

    fn get_tooltip_position(&self, tooltip_width: f64, tooltip_height: f64) -> Point<f64> {
        let channels = self.base.channels.borrow();
        let channel = channels.first().unwrap();

        let focused_entity_index = self.base.props.borrow().focused_entity_index as usize;

        let props = self.props.borrow();

        let pie = channel.entities.get(focused_entity_index).unwrap();
        let angle = 0.5 * (pie.start_angle + pie.end_angle);
        let radius = 0.5 * (props.inner_radius + props.outer_radius);
        let point = utils::polar2cartesian(&props.center, radius, angle);
        let x = point.x - 0.5 * tooltip_width;
        let y = point.y - tooltip_height;
        Point::new(x, y)
    }
}
