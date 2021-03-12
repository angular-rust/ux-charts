#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};
use ux_dataflow::*;
use ux_primitives::{canvas::CanvasContext, color::Color, geom::Point};

#[macro_use]
extern crate lazy_static;

mod basechart;
pub use basechart::*;

mod bar;
pub use bar::*;

mod gauge;
pub use gauge::*;

mod line;
pub use line::*;

mod options;
pub use options::*;

mod pie;
pub use pie::*;

mod radar;
pub use radar::*;

mod utils;
pub use utils::*;

// pub(crate) const CLOCKWISE: i64 = 1;
// pub(crate) const COUNTERCLOCKWISE: i64 = -1;
pub(crate) const HIGHLIGHT_OUTER_RADIUS_FACTOR: f64 = 1.05;

pub const PI: f64 = std::f64::consts::PI;
/// The 2*pi constant - TAU
pub const TAU: f64 = std::f64::consts::TAU;

/// The pi/2 constant.
pub const PI_2: f64 = std::f64::consts::FRAC_PI_2;

pub const DEFAULT_FONT_FAMILY: &str = "Roboto";

/// The padding of the chart itself.
pub const CHART_PADDING: f64 = 12.0;

/// The margin between the legend and the chart-axes box in pixels.
pub const LEGEND_MARGIN: f64 = 12.0;

pub const CHART_TITLE_MARGIN: f64 = 12.0;

/// The padding around the chart title and axis titles.
pub const TITLE_PADDING: f64 = 6.0;

/// The top-and/or-bottom margin of x-axis labels and the right-and/or-left
/// margin of y-axis labels.
///
/// x-axis labels always have top margin. If the x-axis title is N/A, x-axis
/// labels also have bottom margin.
///
/// y-axis labels always have right margin. If the y-axis title is N/A, y-axis
/// labels also have left margin.
pub const AXIS_LABEL_MARGIN: usize = 12;

pub type LabelFormatter = fn(label: String) -> String;

pub type ValueFormatter = fn(value: f64) -> String;

pub fn default_label_formatter(label: String) -> String {
    label
}

pub fn default_value_formatter(value: f64) -> String {
    format!("{}", value)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Visibility {
    Hidden,
    Hiding,
    Showing,
    Shown,
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Hidden
    }
}

pub struct MouseEvent;

/// A chart entity such as a point, a bar, a pie...
pub trait Entity: Default {
    fn free(&mut self);
    fn save(&self);
}

pub trait Drawable<C>
where
    C: CanvasContext,
{
    fn draw(&self, ctx: &C, percent: f64, highlight: bool);
}

#[derive(Default, Debug, Clone)]
pub struct Series<E>
where
    E: Entity,
{
    name: String,
    color: Color,
    highlight_color: Color,
    entities: Vec<E>,
}

impl<E> Series<E>
where
    E: Entity,
{
    pub fn new(name: &str, color: Color, highlight_color: Color, entities: Vec<E>) -> Self {
        Self {
            name: name.into(),
            color,
            highlight_color,
            entities,
        }
    }

    pub fn free_entities(&self, start: usize, end: Option<usize>) {
        let end = match end {
            Some(end) => end,
            None => self.entities.len(),
        };

        let mut start = start;
        while start < end {
            //   self.entities[start].free();
            start = start + 1;
        }
        unimplemented!()
    }
}

pub trait Chart<'a, C, M, D, E>
where
    E: Entity,
    C: CanvasContext,
    M: fmt::Display,
    D: fmt::Display,
{
    /// Calculates various drawing sizes.
    ///
    /// Overriding methods must call this method first to have [series_and_axes_box]
    /// calculated.
    ///
    fn calculate_drawing_sizes(&self);

    /// Updates the series at index [index]. If [index] is `null`, updates all
    /// series.
    ///
    fn update_series(&self, index: usize);

    /// Draws the axes and the grid.
    ///
    fn draw_axes_and_grid(&self, ctx: &C);

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
    fn draw_series(&self, ctx: &C, percent: f64) -> bool;

    /// Draws the current animation frame.
    ///
    /// If [time] is `null`, draws the last frame (i.e. no animation).
    fn draw_frame(&self, ctx: &C, time: Option<i64>);

    // when we impl for concrete chart implementation then it call concrete
    fn create_entities(
        &self,
        series_index: i64,
        start: i64,
        end: i64,
        color: String,
        highlight_color: String,
    ) -> Vec<E> {
        println!("Chart Trait create_entities");
        let result = Vec::new();
        // while (start < end) {
        //   let value = self.base.data_table.rows[start][seriesIndex + 1];
        //   let e = create_entity(seriesIndex, start, value, color, highlight_color);
        //   e.chart = this;
        //   result.add(e);
        //   start++;
        // }
        result
    }

    fn create_entity(
        &self,
        series_index: usize,
        entity_index: usize,
        value: f64,
        color: Color,
        highlight_color: Color,
    ) -> E;

    fn create_series_list(&self, start: usize, end: usize) -> Vec<Series<E>> {
        println!("Chart Trait create_series_list");
        let result = Vec::new();
        // let entity_count = self.data_table.frames.len();
        // while (start < end) {
        //   let name = self.base.data_table.columns[start + 1].name;
        //   let color = get_color(start);
        //   let highlight_color = get_highlight_color(color);
        //   let entities =
        //       create_entities(start, 0, entity_count, color, highlight_color);
        //   result.add(Series(name, color, highlight_color, entities));
        //   start++;
        // }
        result
    }

    /// Returns the position of the tooltip based on
    /// [focused_entity_index].
    // tooltip_width - oltip.offset_width
    // tooltip_height - tooltip.offset_height
    fn get_tooltip_position(&self, tooltip_width: f64, tooltip_height: f64) -> Point<f64>;

    fn set_stream(&self, stream: DataStream<'a, M, D>);

    /// called to commit calculations using non_eq pattern
    fn update(&self, ctx: &C);

    /// called to redraw using non_eq pattern
    fn draw(&self, ctx: &C);

    /// Resizes the chart to fit the new size of the container.
    /// w = container.clientWidth;
    /// h = container.clientHeight;
    fn resize(&self, w: f64, h: f64);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
