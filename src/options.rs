pub struct AnimationOptions {
    // num - The animation duration in ms.
    pub duration: usize,

    // String|EasingFunction - Name of a predefined easing function or an
    // easing function itself.
    //
    // See [animation.dart] for the full list of predefined functions.
    pub easing: String,

    // () -> fn - The function that is called when the animation is complete.
    pub on_end: Option<Box<dyn FnOnce()>>,
}

pub struct LegendOptions<'a> {
    // (String label) -> String - A function that format the labels.
    pub label_formatter: Option<Box<dyn FnOnce()>>,

    // String - The position of the legend relative to the chart area.
    // Supported values: "left", "top", "bottom", "right", "none".
    pub position: String,

    // An object that controls the styling of the legend.
    pub style: StyleOption<'a>,
}

pub struct TitleOptions<'a> {
    // String - The position of the title relative to the chart area.
    // Supported values: "above", "below", "middle", "none";
    pub position: String,

    // An object that controls the styling of the chart title.
    pub style: StyleOption<'a>,

    // The title text. A `null` value means the title is hidden.
    pub text: Option<String>,
}

pub struct TooltipOptions<'a> {
    // bool - Whether to show the tooltip.
    pub enabled: bool,

    // (String label) -> String - A function that format the labels.
    pub label_formatter: Option<Box<dyn FnOnce()>>,

    // An object that controls the styling of the tooltip.
    pub style: StyleOption<'a>,

    // (num value) -> String - A function that formats the values.
    pub value_formatter: Option<Box<dyn FnOnce()>>,
}

/// The global drawing options.
pub struct GlobalOptions<'a> {
    // An object that controls the animation.
    pub animation: AnimationOptions,

    // String - The background color of the chart.
    pub background_color: String,

    // The color list used to render the series. If there are more series than
    // colors, the colors will be reused.
    pub colors: Vec<String>,

    // An object that controls the legend.
    pub legend: LegendOptions<'a>,

    // An object that controls the chart title.
    pub title: TitleOptions<'a>,

    // An object that controls the tooltip.
    pub tooltip: TooltipOptions<'a>,
}

impl<'a> Default for GlobalOptions<'a> {
    fn default() -> Self {
        // animation: AnimationOptions {
        //       duration: 800,
        //       easing: "easeOutQuint".into(),
        //       on_end: None
        // },
        // background_color: "white",
        // colors: vec![
        //     "#7cb5ec",
        //     "#434348",
        //     "#90ed7d",
        //     "#f7a35c",
        //     "#8085e9",
        //     "#f15c80",
        //     "#e4d354",
        //     "#8085e8",
        //     "#8d4653",
        //     "#91e8e1"
        // ],
        // legend: {
        //     label_formatter: None,
        //     position: "right",
        //     style: StyleOption
        // },
        // title: TitleOption {
        //     position: "above",
        //     style: StyleOption,
        //     text: None
        // },
        // tooltip: TooltipOptions {
        //     enabled: true,
        //     label_formatter: None,
        //     style: StyleOption,
        //     value_formatter: None
        // }
        todo!()
    }
}

pub struct BarChartSeriesOptions<'a> {
    // An object that controls the series labels.
    // bool - Whether to show the labels.
    pub labels: Option<StyleOption<'a>>,
}

pub struct BarChartCrosshairOptions {
    // String - The fill color of the crosshair.
    pub color: String,
}

pub struct BarChartXAxisLabelsOptions<'a> {
    // num - The maximum rotation angle in degrees. Must be <= 90.
    pub max_rotation: i64,

    // num - The minimum rotation angle in degrees. Must be >= -90.
    pub min_rotation: i64,

    pub style: StyleOption<'a>,
}

pub struct BarChartXAxisOptions<'a> {
    // An object that controls the crosshair.
    pub crosshair: Option<BarChartCrosshairOptions>,

    // String - The color of the horizontal grid lines.
    pub grid_line_color: String,

    // num - The width of the horizontal grid lines.
    pub grid_line_width: usize,

    // String - The color of the axis itself.
    pub line_color: String,

    // num - The width of the axis itself.
    pub line_width: usize,

    // An object that controls the axis labels.
    pub labels: BarChartXAxisLabelsOptions<'a>,

    // String - The position of the axis relative to the chart area.
    // Supported values: "bottom".
    pub position: String,

    // An object that controls the axis title.
    pub title: TitleOption<'a>,
}

pub struct BarChartYAxisLabelsOptions<'a> {
    // (num value) -> String - A function that formats the labels.
    pub formatter: Option<Box<dyn FnOnce()>>,

    // An object that controls the styling of the axis labels.
    pub style: StyleOption<'a>,
}

pub struct BarChartYAxisOptions<'a> {
    // String - The color of the vertical grid lines.
    pub grid_line_color: String,

    // num - The width of the vertical grid lines.
    pub grid_line_width: usize,

    // String - The color of the axis itself.
    pub line_color: String,

    // num - The width of the axis itself.
    pub line_width: usize,

    // num - The interval of the tick marks in axis unit. If `null`, this value
    // is automatically calculated.
    pub interval: Option<usize>,

    // An object that controls the axis labels.
    pub labels: BarChartYAxisLabelsOptions<'a>,

    // num - The desired maximum value on the axis. If set, the calculated value
    // is guaranteed to be >= this value.
    pub max_value: Option<usize>,

    // num - The minimum interval. If `null`, this value is automatically
    // calculated.
    pub min_interval: Option<usize>,

    // num - The desired minimum value on the axis. If set, the calculated value
    // is guaranteed to be <= this value.
    pub min_value: Option<usize>,

    // String - The position of the axis relative to the chart area.
    // Supported values: "left".
    pub position: String,

    // An object that controls the axis title.
    pub title: TitleOption<'a>,
}

pub struct BarChartOptions<'a> {
    // An object that controls the series.
    pub series: BarChartSeriesOptions<'a>,

    // An object that controls the x-axis.
    pub x_axis: BarChartXAxisOptions<'a>,

    // An object that controls the y-axis.
    pub y_axis: BarChartYAxisOptions<'a>,
}

pub struct TitleOption<'a> {
    // An object that controls the styling of the axis title.
    pub style: StyleOption<'a>,

    // The title text. A `null` value means the title is hidden.
    pub text: Option<String>,
}

impl<'a> Default for BarChartOptions<'a> {
    fn default() -> Self {
        //   series: {
        //       labels: {
        //           enabled: false,
        //           style: StyleOption
        //       }
        //   },
        //   x_axis: {
        //       crosshair: {
        //           color: "rgba(0, 0, 0, .02)",
        //           enabled: false,
        //       },
        //       grid_line_color: "#c0c0c0",
        //       grid_line_width: 1,
        //       line_color: "#c0c0c0",
        //       line_width: 1,
        //       labels: {
        //           max_rotation: 0,
        //           min_rotation: -90,
        //           style: StyleOption
        //       },
        //       position: "bottom",
        //       title: {
        //           style: StyleOption,
        //           text: None
        //       }
        //   },
        //   y_axis: {
        //       grid_line_color: "#c0c0c0",
        //       grid_line_width: 0,
        //       line_color: "#c0c0c0",
        //       line_width: 0,
        //       interval: None,
        //       labels: {
        //           formatter: None,
        //           style: StyleOption,
        //       },
        //       max_value: None,
        //       min_interval: None,
        //       min_value: None,
        //       position: "left",
        //       title: {
        //           style: StyleOption,
        //           text: None
        //       }
        //   }
        todo!()
    }
}

pub struct StyleOption<'a> {
    pub background_color: &'a str,
    pub border_color: &'a str,
    pub border_width: f32, // i32?
    // String - The title"s color
    pub color: &'a str,
    // String - The title"s font family.
    pub font_family: &'a str,
    // num - The title"s font size.
    pub font_size: f32,
    // String - The title"s font style.
    pub font_style: &'a str, // "normal"
}

impl<'a> Default for StyleOption<'a> {
    fn default() -> Self {
        // background_color: "",
        // border_color: "",
        // border_width: 0_f32,
        // color: "",
        // font_family: "",
        // font_size: 0_f32,
        // font_style: "normal", // "normal"
        todo!()
    }
}

pub struct GaugeChartOptions<'a> {
    // String - The background color of the gauges.
    pub background_color: &'a str,

    // An object that controls the gauge labels.
    // Whether to show the labels
    // An object that controls the styling of the gauge labels
    pub labels: Option<StyleOption<'a>>,
}

impl<'a> Default for GaugeChartOptions<'a> {
    fn default() -> Self {
        // background_color: "#dbdbdb",
        // labels: Default::default()
        todo!()
    }
}

pub struct LineChartSeriesMarkersOptions {
    // bool - Whether markers are enabled.
    pub enabled: bool,

    // String - The fill color. If `null`, the stroke color of the series
    // will be used.
    pub fill_color: Option<String>,

    // num - The line width of the markers.
    pub line_width: usize,

    // String - The stroke color. If `null`, the stroke color of the series
    // will be used.
    pub stroke_color: String,

    // num - Size of the markers.
    pub size: usize,
}

pub struct LineChartSeriesOptions<'a> {
    // num - The curve tension. The typical value is from 0.3 to 0.5.
    // To draw straight lines, set this to zero.
    pub curve_tension: f64,

    // num - The opacity of the area between a series and the x-axis.
    pub fill_opacity: f64,

    // num - The line width of the series.
    pub line_width: f64,

    // An object that controls the series labels.
    // Whether to show the labels
    pub labels: Option<StyleOption<'a>>,

    // An object that controls the markers.
    pub markers: LineChartSeriesMarkersOptions,
}

pub struct LineChartXAxisLabelsOptions<'a> {
    // num - The maximum rotation angle in degrees. Must be <= 90.
    pub max_rotation: i64,

    // num - The minimum rotation angle in degrees. Must be >= -90.
    pub min_rotation: i64,

    pub style: StyleOption<'a>,
}

pub struct LineChartXAxisOptions<'a> {
    // String - The color of the horizontal grid lines.
    pub grid_line_color: String,

    // num - The width of the horizontal grid lines.
    pub grid_line_width: usize,

    // String - The color of the axis itself.
    pub line_color: String,

    // num - The width of the axis itself.
    pub line_width: usize,

    // An object that controls the axis labels.
    pub labels: LineChartXAxisLabelsOptions<'a>,

    // String - The position of the axis relative to the chart area.
    // Supported values: "bottom".
    pub position: String,

    // An object that controls the axis title.
    pub title: TitleOption<'a>,
}

pub struct LineChartYAxisLabelsOptions<'a> {
    // (num value) -> String - A function that formats the labels.
    pub formatter: Option<Box<dyn FnOnce()>>,

    // An object that controls the styling of the axis labels.
    pub style: StyleOption<'a>,
}
pub struct LineChartYAxisOptions<'a> {
    // String - The color of the vertical grid lines.
    pub grid_line_color: String,

    // num - The width of the vertical grid lines.
    pub grid_line_width: usize,

    // String - The color of the axis itself.
    pub line_color: String,

    // num - The width of the axis itself.
    pub line_width: usize,

    // num - The interval of the tick marks in axis unit. If `null`, this value
    // is automatically calculated.
    pub interval: Option<usize>,

    // An object that controls the axis labels.
    pub labels: LineChartYAxisLabelsOptions<'a>,

    // num - The desired maximum value on the axis. If set, the calculated value
    // is guaranteed to be >= this value.
    pub max_value: Option<usize>,

    // num - The minimum interval. If `null`, this value is automatically
    // calculated.
    pub min_interval: Option<usize>,

    // num - The desired minimum value on the axis. If set, the calculated value
    // is guaranteed to be <= this value.
    pub min_value: Option<usize>,

    // String - The position of the axis relative to the chart area.
    // Supported values: "left".
    pub position: String,

    // An object that controls the axis title.
    pub title: TitleOption<'a>,
}

pub struct LineChartOptions<'a> {
    // An object that controls the series.
    pub series: LineChartSeriesOptions<'a>,

    // An object that controls the x-axis.
    pub x_axis: LineChartXAxisOptions<'a>,

    // An object that controls the y-axis.
    pub y_axis: LineChartYAxisOptions<'a>,
}

impl<'a> Default for LineChartOptions<'a> {
    fn default() -> Self {
        //   series: {
        //       curve_tension: .4,
        //       fill_opacity: .25,
        //       line_width: 2,
        //       labels: {
        //           enabled: false,
        //           style: StyleOption
        //       },
        //       markers: {
        //           enabled: true,
        //           fill_color: None,
        //           line_width: 1,
        //           stroke_color: "white",
        //           size: 4
        //       }
        //   },
        //   x_axis: {
        //       grid_line_color: "#c0c0c0",
        //       grid_line_width: 1,
        //       line_color: "#c0c0c0",
        //       line_width: 1,
        //       labels: {
        //           max_rotation: 0,
        //           min_rotation: -90,
        //           style: StyleOption
        //       },
        //       position: "bottom",
        //       title: {
        //           style: StyleOption,
        //           text: None
        //       }
        //   },
        //   y_axis: {
        //       grid_line_color: "#c0c0c0",
        //       grid_line_width: 0,
        //       line_color: "#c0c0c0",
        //       line_width: 0,
        //       interval: None,
        //       labels: {
        //           formatter: None,
        //           style: StyleOption
        //       },
        //       max_value: None,
        //       min_interval: None,
        //       min_value: None,
        //       position: "left",
        //       title: {
        //           style: StyleOption,
        //           text: None
        //       }
        //   }
        todo!()
    }
}

pub struct PieChartSeriesLabelsOptions<'a> {
    // bool - Whether to show the labels.
    pub enabled: bool,

    // (num) -> String - A function used to format the labels.
    pub formatter: Option<Box<dyn FnOnce()>>,

    pub style: StyleOption<'a>,
}

pub struct PieChartSeriesOptions<'a> {
    /// bool - Whether to draw the slices counterclockwise.
    pub counterclockwise: bool,

    // An object that controls the series labels.
    pub labels: PieChartSeriesLabelsOptions<'a>,

    // num - The start angle in degrees. Default is -90, which is 12 o"clock.
    pub start_angle: i64,
}

pub struct PieChartOptions<'a> {
    // num - If between 0 and 1, displays a donut chart. The hole will have a
    // radius equal to this value times the radius of the chart.
    pub pie_hole: usize,

    // An object that controls the series.
    pub series: PieChartSeriesOptions<'a>,
}

impl<'a> Default for PieChartOptions<'a> {
    fn default() -> Self {
        //   pie_hole: 0,
        //   series: {
        //       counterclockwise: false,
        //       labels: {
        //           enabled: false,
        //           formatter: None,
        //           style: StyleOption,
        //       },
        //       start_angle: -90,
        //   }
        todo!()
    }
}

pub struct RadarChartSeriesMarkersOptions {
    // bool - Whether markers are enabled.
    pub enabled: bool,

    // String - The fill color. If `null`, the stroke color of the series
    // will be used.
    pub fill_color: Option<String>,

    // num - The line width of the markers.
    pub line_width: usize,

    // String - The stroke color. If `null`, the stroke color of the series
    // will be used.
    pub stroke_color: String,

    // num - Size of the markers. To disable markers, set this to zero.
    pub size: usize,
}

pub struct RadarChartSeriesOptions<'a> {
    // num - The opacity of the area between a series and the x-axis.
    pub fill_opacity: f64,

    // num - The line width of the series.
    pub line_width: usize,

    // An object that controls the series labels.
    //   Whether to show the labels.
    pub labels: Option<StyleOption<'a>>,

    // An object that controls the markers.
    pub markers: RadarChartSeriesMarkersOptions,
}

pub struct RadarChartXAxisOptions<'a> {
    // String - The color of the horizontal grid lines.
    pub grid_line_color: String,

    // num - The width of the horizontal grid lines.
    pub grid_line_width: f64,

    // An object that controls the axis labels.
    pub labels: Option<StyleOption<'a>>,
}

pub struct RadarChartYAxisLabelsOptions<'a> {
    // (num value) -> String - A function that formats the labels.
    pub formatter: Option<Box<dyn FnOnce()>>,

    // An object that controls the styling of the axis labels.
    pub style: StyleOption<'a>,
}

pub struct RadarChartYAxisOptions<'a> {
    // String - The color of the vertical grid lines.
    pub grid_line_color: String,

    // num - The width of the vertical grid lines.
    pub grid_line_width: f64,

    // num - The interval of the tick marks in axis unit. If `null`, this value
    // is automatically calculated.
    pub interval: Option<usize>,

    // An object that controls the axis labels.
    pub labels: RadarChartYAxisLabelsOptions<'a>,

    // num - The minimum interval. If `null`, this value is automatically
    // calculated.
    pub min_interval: Option<usize>,
}

pub struct RadarChartOptions<'a> {
    // An object that controls the series.
    pub series: RadarChartSeriesOptions<'a>,

    // An object that controls the x-axis.
    pub x_axis: RadarChartXAxisOptions<'a>,

    // An object that controls the y-axis.
    pub y_axis: RadarChartYAxisOptions<'a>,
}

impl<'a> Default for RadarChartOptions<'a> {
    fn default() -> Self {
        //   series: {
        //       fill_opacity: .25,
        //       line_width: 2,
        //       labels: {
        //           enabled: false,
        //           style: StyleOption
        //       },
        //       markers: {
        //           enabled: true,
        //           fill_color: None,
        //           line_width: 1,
        //           stroke_color: "white",
        //           size: 4
        //       }
        //   },
        //   x_axis: {
        //       grid_line_color: "#c0c0c0",
        //       grid_line_width: 1,
        //       labels: {
        //           style: StyleOption
        //       },
        //   },
        //   y_axis: {
        //       grid_line_color: "#c0c0c0",
        //       grid_line_width: 1,
        //       interval: None,
        //       labels: {
        //           formatter: None,
        //           style: StyleOption
        //       },
        //       min_interval: None,
        //   }
        todo!()
    }
}
