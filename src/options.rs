use super::{LabelFormatter, ValueFormatter};
use primitives::{color, Color, TextStyle};

pub enum Position {
    Above,
    Middle,
    Below,
    Left, 
    Top, 
    Bottom, 
    Right,
    None, 
}

pub trait BaseOption<'a> {
    fn animation(&self) -> &AnimationOptions;
    fn colors(&self) -> &Vec<Color>;
    fn title(&self) -> &TitleOptions<'a>;
    fn legend(&self) -> &LegendOptions<'a>;
    fn tooltip(&self) -> &TooltipOptions<'a>;
    fn background(&self) -> &Color;
}

pub struct AnimationOptions {
    /// The animation duration in ms.
    pub duration: usize,

    /// String|EasingFunction - Name of a predefined easing function or an
    /// easing function itself.
    ///
    /// See [animation.dart] for the full list of predefined functions.
    pub easing: String,

    /// () -> fn - The function that is called when the animation is complete.
    pub on_end: Option<fn()>,
}

pub struct LegendOptions<'a> {
    /// (String label) -> String - A function that format the labels.
    pub label_formatter: Option<LabelFormatter>,

    /// The position of the legend relative to the chart area.
    /// Supported values: "left", "top", "bottom", "right", "none".
    pub position: Position,

    /// An object that controls the styling of the legend.
    pub style: StyleOption<'a>,
}

pub struct TitleOptions<'a> {
    /// The position of the title relative to the chart area.
    /// Supported values: "above", "below", "middle", "none";
    pub position: Position,

    /// An object that controls the styling of the chart title.
    pub style: StyleOption<'a>,

    /// The title text. A `null` value means the title is hidden.
    pub text: Option<&'a str>,
}

pub struct TooltipOptions<'a> {
    /// bool - Whether to show the tooltip.
    pub enabled: bool,

    /// (String label) -> String - A function that format the labels.
    pub label_formatter: Option<LabelFormatter>,

    /// An object that controls the styling of the tooltip.
    pub style: StyleOption<'a>,

    /// (num value) -> String - A function that formats the values.
    pub value_formatter: Option<ValueFormatter>,
}

#[derive(Debug, Clone)]
pub struct BarChartSeriesOptions<'a> {
    /// An object that controls the channel labels.
    /// bool - Whether to show the labels.
    pub labels: Option<StyleOption<'a>>,
}

pub struct BarChartCrosshairOptions {
    /// The fill color of the crosshair.
    pub color: Color,
}

pub struct BarChartXAxisLabelsOptions<'a> {
    /// The maximum rotation angle in degrees. Must be <= 90.
    pub max_rotation: i64,

    /// The minimum rotation angle in degrees. Must be >= -90.
    pub min_rotation: i64,

    pub style: StyleOption<'a>,
}

pub struct BarChartXAxisOptions<'a> {
    /// An object that controls the crosshair.
    pub crosshair: Option<BarChartCrosshairOptions>,

    /// The color of the horizontal grid lines.
    pub grid_line_color: Color,

    /// The width of the horizontal grid lines.
    pub grid_line_width: f64,

    /// The color of the axis itself.
    pub line_color: Color,

    /// The width of the axis itself.
    pub line_width: f64,

    /// An object that controls the axis labels.
    pub labels: BarChartXAxisLabelsOptions<'a>,

    /// The position of the axis relative to the chart area.
    /// Supported values: "bottom".
    pub position: Position,

    /// An object that controls the axis title.
    pub title: TitleOption<'a>,
}

pub struct BarChartYAxisLabelsOptions<'a> {
    /// (num value) -> String - A function that formats the labels.
    pub formatter: Option<ValueFormatter>,

    /// An object that controls the styling of the axis labels.
    pub style: StyleOption<'a>,
}

pub struct BarChartYAxisOptions<'a> {
    /// The color of the vertical grid lines.
    pub grid_line_color: Color,

    /// The width of the vertical grid lines.
    pub grid_line_width: f64,

    /// The color of the axis itself.
    pub line_color: Color,

    /// The width of the axis itself.
    pub line_width: f64,

    /// The interval of the tick marks in axis unit. If `null`, this value
    /// is automatically calculated.
    pub interval: Option<f64>,

    /// An object that controls the axis labels.
    pub labels: BarChartYAxisLabelsOptions<'a>,

    /// The desired maximum value on the axis. If set, the calculated value
    /// is guaranteed to be >= this value.
    pub max_value: Option<usize>,

    /// The minimum interval. If `null`, this value is automatically
    /// calculated.
    pub min_interval: Option<f64>,

    /// The desired minimum value on the axis. If set, the calculated value
    /// is guaranteed to be <= this value.
    pub min_value: Option<usize>,

    /// The position of the axis relative to the chart area.
    /// Supported values: "left".
    pub position: Position,

    /// An object that controls the axis title.
    pub title: TitleOption<'a>,
}

pub struct BarChartOptions<'a> {
    /// An object that controls the channel.
    pub channel: BarChartSeriesOptions<'a>,

    /// An object that controls the x-axis.
    pub xaxis: BarChartXAxisOptions<'a>,

    /// An object that controls the y-axis.
    pub yaxis: BarChartYAxisOptions<'a>,

    /// An object that controls the animation.
    pub animation: AnimationOptions,

    /// The background color of the chart.
    pub background: Color,

    /// The color list used to render the channel. If there are more channel than
    /// colors, the colors will be reused.
    pub colors: Vec<Color>,

    /// An object that controls the legend.
    pub legend: LegendOptions<'a>,

    /// An object that controls the chart title.
    pub title: TitleOptions<'a>,

    /// An object that controls the tooltip.
    pub tooltip: TooltipOptions<'a>,
}

impl<'a> BaseOption<'a> for BarChartOptions<'a> {
    fn animation(&self) -> &AnimationOptions {
        &self.animation
    }

    fn colors(&self) -> &Vec<Color> {
        &self.colors
    }

    fn title(&self) -> &TitleOptions<'a> {
        &self.title
    }

    fn legend(&self) -> &LegendOptions<'a> {
        &self.legend
    }

    fn tooltip(&self) -> &TooltipOptions<'a> {
        &self.tooltip
    }

    fn background(&self) -> &Color {
        &self.background
    }
}

pub struct TitleOption<'a> {
    /// An object that controls the styling of the axis title.
    pub style: StyleOption<'a>,

    /// The title text. A `null` value means the title is hidden.
    pub text: Option<&'a str>,
}

impl<'a> Default for BarChartOptions<'a> {
    fn default() -> Self {
        Self {
            channel: BarChartSeriesOptions { labels: None },
            xaxis: BarChartXAxisOptions {
                crosshair: None,
                grid_line_color: color::GRAY_5,
                grid_line_width: 1.,
                line_color: color::GRAY_5,
                line_width: 1.,
                labels: BarChartXAxisLabelsOptions {
                    max_rotation: 0,
                    min_rotation: -90,
                    style: Default::default(),
                },
                position: Position::Bottom,
                title: TitleOption {
                    style: Default::default(),
                    text: None,
                },
            },
            yaxis: BarChartYAxisOptions {
                grid_line_color: color::GRAY_5,
                grid_line_width: 0.,
                line_color: color::GRAY_5,
                line_width: 0.,
                interval: None,
                labels: BarChartYAxisLabelsOptions {
                    formatter: None,
                    style: Default::default(),
                },
                max_value: None,
                min_interval: None,
                min_value: None,
                position: Position::Left,
                title: TitleOption {
                    style: Default::default(),
                    text: None,
                },
            },
            animation: AnimationOptions {
                duration: 800,
                easing: "easeOutQuint".into(),
                on_end: None,
            },
            background: color::WHITE,
            colors: vec![
                Color::RGB(0x7c, 0xb5, 0xec),
                Color::RGB(0x43, 0x43, 0x48),
                Color::RGB(0x90, 0xed, 0x7d),
                Color::RGB(0xf7, 0xa3, 0x5c),
                Color::RGB(0x80, 0x85, 0xe9),
                Color::RGB(0xf1, 0x5c, 0x80),
                Color::RGB(0xe4, 0xd3, 0x54),
                Color::RGB(0x80, 0x85, 0xe8),
                Color::RGB(0x8d, 0x46, 0x53),
                Color::RGB(0x91, 0xe8, 0xe1),
            ],
            legend: LegendOptions {
                label_formatter: None,
                position: Position::Right,
                style: Default::default(),
            },
            title: TitleOptions {
                position: Position::Above,
                style: Default::default(),
                text: None,
            },
            tooltip: TooltipOptions {
                enabled: true,
                label_formatter: None,
                style: Default::default(),
                value_formatter: None,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyleOption<'a> {
    pub background: Color,
    pub border_color: Color,
    pub border_width: f64, // i32?
    /// The title"s color
    pub color: Color,
    /// The title"s font family.
    pub fontfamily: Option<&'a str>,
    /// The title"s font size.
    pub fontsize: Option<f64>,
    /// The title"s font style.
    pub fontstyle: Option<TextStyle>,
}

impl<'a> Default for StyleOption<'a> {
    fn default() -> Self {
        Self {
            background: color::WHITE,
            border_color: color::GRAY_4,
            border_width: 0_f64,
            color: color::GRAY_9,
            fontfamily: Some("Roboto"),
            fontsize: Some(12_f64),
            fontstyle: Some(TextStyle::Normal),
        }
    }
}

pub struct GaugeChartOptions<'a> {
    /// An object that controls the gauge labels.
    /// Whether to show the labels
    /// An object that controls the styling of the gauge labels
    pub labels: Option<StyleOption<'a>>,

    /// An object that controls the animation.
    pub animation: AnimationOptions,

    /// The background color of the chart.
    pub background: Color,

    /// The background color of the gauge.
    pub gauge_background: Color,

    /// The color list used to render the channel. If there are more channel than
    /// colors, the colors will be reused.
    pub colors: Vec<Color>,

    /// An object that controls the legend.
    pub legend: LegendOptions<'a>,

    /// An object that controls the chart title.
    pub title: TitleOptions<'a>,

    /// An object that controls the tooltip.
    pub tooltip: TooltipOptions<'a>,
}

impl<'a> BaseOption<'a> for GaugeChartOptions<'a> {
    fn animation(&self) -> &AnimationOptions {
        &self.animation
    }

    fn colors(&self) -> &Vec<Color> {
        &self.colors
    }

    fn title(&self) -> &TitleOptions<'a> {
        &self.title
    }

    fn legend(&self) -> &LegendOptions<'a> {
        &self.legend
    }

    fn tooltip(&self) -> &TooltipOptions<'a> {
        &self.tooltip
    }

    fn background(&self) -> &Color {
        &self.background
    }
}

impl<'a> Default for GaugeChartOptions<'a> {
    fn default() -> Self {
        Self {
            labels: Default::default(),
            animation: AnimationOptions {
                duration: 800,
                easing: "easeOutQuint".into(),
                on_end: None,
            },
            background: color::WHITE,
            gauge_background: color::GRAY_3,
            colors: vec![
                Color::RGB(0x7c, 0xb5, 0xec),
                Color::RGB(0x43, 0x43, 0x48),
                Color::RGB(0x90, 0xed, 0x7d),
                Color::RGB(0xf7, 0xa3, 0x5c),
                Color::RGB(0x80, 0x85, 0xe9),
                Color::RGB(0xf1, 0x5c, 0x80),
                Color::RGB(0xe4, 0xd3, 0x54),
                Color::RGB(0x80, 0x85, 0xe8),
                Color::RGB(0x8d, 0x46, 0x53),
                Color::RGB(0x91, 0xe8, 0xe1),
            ],
            legend: LegendOptions {
                label_formatter: None,
                position: Position::Right,
                style: Default::default(),
            },
            title: TitleOptions {
                position: Position::Above,
                style: Default::default(),
                text: None,
            },
            tooltip: TooltipOptions {
                enabled: true,
                label_formatter: None,
                style: Default::default(),
                value_formatter: None,
            },
        }
    }
}

pub struct LineChartSeriesMarkersOptions {
    /// bool - Whether markers are enabled.
    pub enabled: bool,

    /// The fill color. If `null`, the stroke color of the channel
    /// will be used.
    pub fill_color: Option<Color>,

    /// The line width of the markers.
    pub line_width: usize,

    /// The stroke color. If `null`, the stroke color of the channel
    /// will be used.
    pub stroke_color: Option<Color>,

    /// Size of the markers.
    pub size: f64,
}

pub struct LineChartSeriesOptions<'a> {
    /// The curve tension. The typical value is from 0.3 to 0.5.
    /// To draw straight lines, set this to zero.
    pub curve_tension: f64,

    /// The opacity of the area between a channel and the x-axis.
    pub fill_opacity: f64,

    /// The line width of the channel.
    pub line_width: f64,

    /// An object that controls the channel labels.
    /// Whether to show the labels
    pub labels: Option<StyleOption<'a>>,

    /// An object that controls the markers.
    pub markers: LineChartSeriesMarkersOptions,
}

pub struct LineChartXAxisLabelsOptions<'a> {
    /// The maximum rotation angle in degrees. Must be <= 90.
    pub max_rotation: i64,

    /// The minimum rotation angle in degrees. Must be >= -90.
    pub min_rotation: i64,

    pub style: StyleOption<'a>,
}

pub struct LineChartXAxisOptions<'a> {
    /// The color of the horizontal grid lines.
    pub grid_line_color: Color,

    /// The width of the horizontal grid lines.
    pub grid_line_width: f64,

    /// The color of the axis itself.
    pub line_color: Color,

    /// The width of the axis itself.
    pub line_width: f64,

    /// An object that controls the axis labels.
    pub labels: LineChartXAxisLabelsOptions<'a>,

    /// The position of the axis relative to the chart area.
    /// Supported values: "bottom".
    pub position: Position,

    /// An object that controls the axis title.
    pub title: TitleOption<'a>,
}

pub struct LineChartYAxisLabelsOptions<'a> {
    /// (num value) -> String - A function that formats the labels.
    pub formatter: Option<ValueFormatter>,

    /// An object that controls the styling of the axis labels.
    pub style: StyleOption<'a>,
}
pub struct LineChartYAxisOptions<'a> {
    /// The color of the vertical grid lines.
    pub grid_line_color: Color,

    /// The width of the vertical grid lines.
    pub grid_line_width: f64,

    /// The color of the axis itself.
    pub line_color: Color,

    /// The width of the axis itself.
    pub line_width: f64,

    /// The interval of the tick marks in axis unit. If `null`, this value
    /// is automatically calculated.
    pub interval: Option<f64>,

    /// An object that controls the axis labels.
    pub labels: LineChartYAxisLabelsOptions<'a>,

    /// The desired maximum value on the axis. If set, the calculated value
    /// is guaranteed to be >= this value.
    pub max_value: Option<usize>,

    /// The minimum interval. If `null`, this value is automatically
    /// calculated.
    pub min_interval: Option<f64>,

    /// The desired minimum value on the axis. If set, the calculated value
    /// is guaranteed to be <= this value.
    pub min_value: Option<usize>,

    /// The position of the axis relative to the chart area.
    /// Supported values: "left".
    pub position: Position,

    /// An object that controls the axis title.
    pub title: TitleOption<'a>,
}

pub struct LineChartOptions<'a> {
    /// An object that controls the channel.
    pub channel: LineChartSeriesOptions<'a>,

    /// An object that controls the x-axis.
    pub xaxis: LineChartXAxisOptions<'a>,

    /// An object that controls the y-axis.
    pub yaxis: LineChartYAxisOptions<'a>,

    /// An object that controls the animation.
    pub animation: AnimationOptions,

    /// The background color of the chart.
    pub background: Color,

    /// The color list used to render the channel. If there are more channel than
    /// colors, the colors will be reused.
    pub colors: Vec<Color>,

    /// An object that controls the legend.
    pub legend: LegendOptions<'a>,

    /// An object that controls the chart title.
    pub title: TitleOptions<'a>,

    /// An object that controls the tooltip.
    pub tooltip: TooltipOptions<'a>,
}

impl<'a> BaseOption<'a> for LineChartOptions<'a> {
    fn animation(&self) -> &AnimationOptions {
        &self.animation
    }

    fn colors(&self) -> &Vec<Color> {
        &self.colors
    }

    fn title(&self) -> &TitleOptions<'a> {
        &self.title
    }

    fn legend(&self) -> &LegendOptions<'a> {
        &self.legend
    }

    fn tooltip(&self) -> &TooltipOptions<'a> {
        &self.tooltip
    }

    fn background(&self) -> &Color {
        &self.background
    }
}

impl<'a> Default for LineChartOptions<'a> {
    fn default() -> Self {
        Self {
            channel: LineChartSeriesOptions {
                curve_tension: 0.4,
                fill_opacity: 0.25,
                line_width: 2_f64,
                labels: None,
                markers: LineChartSeriesMarkersOptions {
                    enabled: true,
                    fill_color: None,
                    line_width: 1,
                    stroke_color: Some(color::WHITE),
                    size: 4.,
                },
            },
            xaxis: LineChartXAxisOptions {
                grid_line_color: color::GRAY_5,
                grid_line_width: 1.,
                line_color: color::GRAY_5,
                line_width: 1.,
                labels: LineChartXAxisLabelsOptions {
                    max_rotation: 0,
                    min_rotation: -90,
                    style: Default::default(),
                },
                position: Position::Bottom,
                title: TitleOption {
                    style: Default::default(),
                    text: None,
                },
            },
            yaxis: LineChartYAxisOptions {
                grid_line_color: color::GRAY_5,
                grid_line_width: 0.,
                line_color: color::GRAY_5,
                line_width: 0.,
                interval: None,
                labels: LineChartYAxisLabelsOptions {
                    formatter: None,
                    style: Default::default(),
                },
                max_value: None,
                min_interval: None,
                min_value: None,
                position: Position::Left,
                title: TitleOption {
                    style: Default::default(),
                    text: None,
                },
            },
            animation: AnimationOptions {
                duration: 800,
                easing: "easeOutQuint".into(),
                on_end: None,
            },
            background: color::WHITE,
            colors: vec![
                Color::RGB(0x7c, 0xb5, 0xec),
                Color::RGB(0x43, 0x43, 0x48),
                Color::RGB(0x90, 0xed, 0x7d),
                Color::RGB(0xf7, 0xa3, 0x5c),
                Color::RGB(0x80, 0x85, 0xe9),
                Color::RGB(0xf1, 0x5c, 0x80),
                Color::RGB(0xe4, 0xd3, 0x54),
                Color::RGB(0x80, 0x85, 0xe8),
                Color::RGB(0x8d, 0x46, 0x53),
                Color::RGB(0x91, 0xe8, 0xe1),
            ],
            legend: LegendOptions {
                label_formatter: None,
                position: Position::Right,
                style: Default::default(),
            },
            title: TitleOptions {
                position: Position::Above,
                style: Default::default(),
                text: None,
            },
            tooltip: TooltipOptions {
                enabled: true,
                label_formatter: None,
                style: Default::default(),
                value_formatter: None,
            },
        }
    }
}

pub struct PieChartSeriesLabelsOptions<'a> {
    /// bool - Whether to show the labels.
    pub enabled: bool,

    /// (num) -> String - A function used to format the labels.
    pub formatter: Option<ValueFormatter>,

    pub style: StyleOption<'a>,
}

pub struct PieChartSeriesOptions<'a> {
    /// bool - Whether to draw the slices counterclockwise.
    pub counterclockwise: bool,

    /// An object that controls the channel labels.
    pub labels: PieChartSeriesLabelsOptions<'a>,

    /// The start angle in degrees. Default is -90, which is 12 o'clock.
    pub start_angle: f64,
}

pub struct PieChartOptions<'a> {
    /// If between 0 and 1, displays a donut chart. The hole will have a
    /// radius equal to this value times the radius of the chart.
    pub pie_hole: f64,

    /// An object that controls the channel.
    pub channel: PieChartSeriesOptions<'a>,

    /// An object that controls the animation.
    pub animation: AnimationOptions,

    /// The background color of the chart.
    pub background: Color,

    /// The color list used to render the channel. If there are more channel than
    /// colors, the colors will be reused.
    pub colors: Vec<Color>,

    /// An object that controls the legend.
    pub legend: LegendOptions<'a>,

    /// An object that controls the chart title.
    pub title: TitleOptions<'a>,

    /// An object that controls the tooltip.
    pub tooltip: TooltipOptions<'a>,
}

impl<'a> BaseOption<'a> for PieChartOptions<'a> {
    fn animation(&self) -> &AnimationOptions {
        &self.animation
    }

    fn colors(&self) -> &Vec<Color> {
        &self.colors
    }

    fn title(&self) -> &TitleOptions<'a> {
        &self.title
    }

    fn legend(&self) -> &LegendOptions<'a> {
        &self.legend
    }

    fn tooltip(&self) -> &TooltipOptions<'a> {
        &self.tooltip
    }

    fn background(&self) -> &Color {
        &self.background
    }
}

impl<'a> Default for PieChartOptions<'a> {
    fn default() -> Self {
        Self {
            pie_hole: 0_f64,
            channel: PieChartSeriesOptions {
                counterclockwise: false,
                labels: PieChartSeriesLabelsOptions {
                    enabled: false,
                    formatter: None,
                    style: Default::default(),
                },
                start_angle: -90_f64,
            },
            animation: AnimationOptions {
                duration: 800,
                easing: "easeOutQuint".into(),
                on_end: None,
            },
            background: color::WHITE,
            colors: vec![
                Color::RGB(0x7c, 0xb5, 0xec),
                Color::RGB(0x43, 0x43, 0x48),
                Color::RGB(0x90, 0xed, 0x7d),
                Color::RGB(0xf7, 0xa3, 0x5c),
                Color::RGB(0x80, 0x85, 0xe9),
                Color::RGB(0xf1, 0x5c, 0x80),
                Color::RGB(0xe4, 0xd3, 0x54),
                Color::RGB(0x80, 0x85, 0xe8),
                Color::RGB(0x8d, 0x46, 0x53),
                Color::RGB(0x91, 0xe8, 0xe1),
            ],
            legend: LegendOptions {
                label_formatter: None,
                position: Position::Right,
                style: Default::default(),
            },
            title: TitleOptions {
                position: Position::Above,
                style: Default::default(),
                text: None,
            },
            tooltip: TooltipOptions {
                enabled: true,
                label_formatter: None,
                style: Default::default(),
                value_formatter: None,
            },
        }
    }
}

pub struct RadarChartSeriesMarkersOptions {
    /// bool - Whether markers are enabled.
    pub enabled: bool,

    /// The fill color. If `null`, the stroke color of the channel
    /// will be used.
    pub fill_color: Option<Color>,

    /// The line width of the markers.
    pub line_width: f64,

    /// The stroke color. If `null`, the stroke color of the channel
    /// will be used.
    pub stroke_color: Option<Color>,

    /// Size of the markers. To disable markers, set this to zero.
    pub size: f64,
}

pub struct RadarChartSeriesOptions<'a> {
    /// The opacity of the area between a channel and the x-axis.
    pub fill_opacity: f64,

    /// The line width of the channel.
    pub line_width: f64,

    /// An object that controls the channel labels.
    ///   Whether to show the labels.
    pub labels: Option<StyleOption<'a>>,

    /// An object that controls the markers.
    pub markers: RadarChartSeriesMarkersOptions,
}

pub struct RadarChartXAxisLabelsOptions<'a> {
    /// (num value) -> String - A function that formats the labels.
    pub formatter: Option<ValueFormatter>,

    /// An object that controls the styling of the axis labels.
    pub style: StyleOption<'a>,
}

pub struct RadarChartXAxisOptions<'a> {
    /// The color of the horizontal grid lines.
    pub grid_line_color: Color,

    /// The width of the horizontal grid lines.
    pub grid_line_width: f64,

    /// An object that controls the axis labels.
    pub labels: RadarChartXAxisLabelsOptions<'a>,
}

pub struct RadarChartYAxisLabelsOptions<'a> {
    /// (num value) -> String - A function that formats the labels.
    pub formatter: Option<ValueFormatter>,

    /// An object that controls the styling of the axis labels.
    pub style: StyleOption<'a>,
}

pub struct RadarChartYAxisOptions<'a> {
    /// The color of the vertical grid lines.
    pub grid_line_color: Color,

    /// The width of the vertical grid lines.
    pub grid_line_width: f64,

    /// The interval of the tick marks in axis unit. If `null`, this value
    /// is automatically calculated.
    pub interval: Option<f64>,

    /// An object that controls the axis labels.
    pub labels: RadarChartYAxisLabelsOptions<'a>,

    /// The minimum interval. If `null`, this value is automatically
    /// calculated.
    pub min_interval: Option<f64>,
}

pub struct RadarChartOptions<'a> {
    // An object that controls the channel.
    pub channel: RadarChartSeriesOptions<'a>,

    /// An object that controls the x-axis.
    pub xaxis: RadarChartXAxisOptions<'a>,

    /// An object that controls the y-axis.
    pub yaxis: RadarChartYAxisOptions<'a>,

    /// An object that controls the animation.
    pub animation: AnimationOptions,

    /// The background color of the chart.
    pub background_color: Color,

    /// The color list used to render the channel. If there are more channel than
    /// colors, the colors will be reused.
    pub colors: Vec<Color>,

    /// An object that controls the legend.
    pub legend: LegendOptions<'a>,

    /// An object that controls the chart title.
    pub title: TitleOptions<'a>,

    /// An object that controls the tooltip.
    pub tooltip: TooltipOptions<'a>,
}

impl<'a> BaseOption<'a> for RadarChartOptions<'a> {
    fn animation(&self) -> &AnimationOptions {
        &self.animation
    }

    fn colors(&self) -> &Vec<Color> {
        &self.colors
    }

    fn title(&self) -> &TitleOptions<'a> {
        &self.title
    }

    fn legend(&self) -> &LegendOptions<'a> {
        &self.legend
    }

    fn tooltip(&self) -> &TooltipOptions<'a> {
        &self.tooltip
    }

    fn background(&self) -> &Color {
        &self.background_color
    }
}

impl<'a> Default for RadarChartOptions<'a> {
    fn default() -> Self {
        Self {
            channel: RadarChartSeriesOptions {
                fill_opacity: 0.25,
                line_width: 2.,
                labels: None,
                markers: RadarChartSeriesMarkersOptions {
                    enabled: true,
                    fill_color: None,
                    line_width: 1.,
                    stroke_color: Some(color::WHITE),
                    size: 4.,
                },
            },
            xaxis: RadarChartXAxisOptions {
                grid_line_color: color::GRAY_5,
                grid_line_width: 1_f64,
                labels: RadarChartXAxisLabelsOptions {
                    formatter: None,
                    style: Default::default(),
                },
            },
            yaxis: RadarChartYAxisOptions {
                grid_line_color: color::GRAY_5,
                grid_line_width: 1_f64,
                interval: None,
                labels: RadarChartYAxisLabelsOptions {
                    formatter: None,
                    style: Default::default(),
                },
                min_interval: None,
            },
            animation: AnimationOptions {
                duration: 800,
                easing: "easeOutQuint".into(),
                on_end: None,
            },
            background_color: color::WHITE,
            colors: vec![
                Color::RGB(0x7c, 0xb5, 0xec),
                Color::RGB(0x43, 0x43, 0x48),
                Color::RGB(0x90, 0xed, 0x7d),
                Color::RGB(0xf7, 0xa3, 0x5c),
                Color::RGB(0x80, 0x85, 0xe9),
                Color::RGB(0xf1, 0x5c, 0x80),
                Color::RGB(0xe4, 0xd3, 0x54),
                Color::RGB(0x80, 0x85, 0xe8),
                Color::RGB(0x8d, 0x46, 0x53),
                Color::RGB(0x91, 0xe8, 0xe1),
            ],
            legend: LegendOptions {
                label_formatter: None,
                position: Position::Right,
                style: Default::default(),
            },
            title: TitleOptions {
                position: Position::Above,
                style: Default::default(),
                text: None,
            },
            tooltip: TooltipOptions {
                enabled: true,
                label_formatter: None,
                style: Default::default(),
                value_formatter: None,
            },
        }
    }
}
