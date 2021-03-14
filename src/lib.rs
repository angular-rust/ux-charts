#![allow(unused_imports)]
#![allow(unused_variables)]

use dataflow::*;
use primitives::{CanvasContext, Color, Point};
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

#[macro_use]
extern crate log;

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

pub mod utils;

pub(crate) const CLOCKWISE: i64 = 1;
pub(crate) const COUNTERCLOCKWISE: i64 = -1;
pub(crate) const HIGHLIGHT_OUTER_RADIUS_FACTOR: f64 = 1.05;

pub const PI: f64 = std::f64::consts::PI;
/// The 2*pi constant - TAU
pub const TAU: f64 = std::f64::consts::TAU;

/// The pi/2 constant.
pub const PI_2: f64 = std::f64::consts::FRAC_PI_2;

pub const DEFAULT_FONT_FAMILY: &str = "monospace";

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
pub trait Entity {
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
pub struct ChartChannel<E>
where
    E: Entity,
{
    name: String,
    color: Color,
    highlight: Color,
    state: Visibility,
    entities: Vec<E>,
}

impl<E> ChartChannel<E>
where
    E: Entity,
{
    pub fn new(name: &str, color: Color, highlight: Color, entities: Vec<E>) -> Self {
        Self {
            name: name.into(),
            color,
            highlight,
            state: Visibility::Shown,
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
    D: fmt::Display + Copy,
{
    /// Calculates various drawing sizes.
    ///
    /// Overriding methods must call this method first to have [channel_and_axes_box]
    /// calculated.
    ///
    fn calculate_drawing_sizes(&self, ctx: &C);

    /// Updates the channel at index [index]. If [index] is `null`, updates all
    /// channel.
    ///
    fn update_channel(&self, index: usize);

    /// Draws the axes and the grid.
    ///
    fn draw_axes_and_grid(&self, ctx: &C);

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
    fn draw_channels(&self, ctx: &C, percent: f64) -> bool;

    /// Draws the current animation frame.
    ///
    /// If [time] is `null`, draws the last frame (i.e. no animation).
    fn draw_frame(&self, ctx: &C, time: Option<i64>);

    // when we impl for concrete chart implementation then it call concrete
    fn create_entities(
        &self,
        channel_index: usize,
        start: usize,
        end: usize,
        color: Color,
        highlight_color: Color,
    ) -> Vec<E>;

    fn create_entity(
        &self,
        channel_index: usize,
        entity_index: usize,
        value: Option<D>,
        color: Color,
        highlight_color: Color,
    ) -> E;

    fn create_channels(&self, start: usize, end: usize);

    /// Returns the position of the tooltip based on
    /// [focused_entity_index].
    // tooltip_width - oltip.offset_width
    // tooltip_height - tooltip.offset_height
    fn get_tooltip_position(&self, tooltip_width: f64, tooltip_height: f64) -> Point<f64>;

    fn set_stream(&mut self, stream: DataStream<'a, M, D>);

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
