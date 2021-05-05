//! Terminal plotting library for using in CLI applications.
//! Should work well in any unicode terminal with monospaced font.
//!
//! It is inspired by [TextPlots.jl](https://github.com/sunetos/TextPlots.jl) which is inspired by [Drawille](https://github.com/asciimoo/drawille).
//!
//! Currently it features only drawing line plots on Braille canvas, but could be extended
//! to support other canvas and chart types just like [UnicodePlots.jl](https://github.com/Evizero/UnicodePlots.jl)
//! or any other cool terminal plotting library.
//!
//! Contributions are very much welcome!
//!
//! # Usage
//! ```toml
//! [dependencies]
//! textplots = "0.6"
//! ```
//!
//! ```rust
//! use textplots::{Chart, Plot, Shape};
//!
//! fn main() {
//!     println!("y = sin(x) / x");
//!
//!     Chart::default()
//!     	.lineplot(&Shape::Continuous(Box::new(|x| x.sin() / x)))
//!     	.display();
//! }
//! ```
//!
//! It will display something like this:
//!
//! <img src="https://github.com/loony-bean/textplots-rs/blob/master/doc/demo.png?raw=true"/>
//!
//! Default viewport size is 120 x 60 points, with X values ranging from -10 to 10.
//! You can override the defaults calling `new`.
//!
//! ```rust
//! use textplots::{Chart, Plot, Shape};
//!
//! println!("y = cos(x), y = sin(x) / 2");
//!
//! Chart::new(180, 60, -5.0, 5.0)
//!     .lineplot(&Shape::Continuous(Box::new(|x| x.cos())))
//!     .lineplot(&Shape::Continuous(Box::new(|x| x.sin() / 2.0)))
//!     .display();
//! ```
//!
//! <img src="https://github.com/loony-bean/textplots-rs/blob/master/doc/demo2.png?raw=true"/>
//!
//! You could also plot series of points. See [Shape](enum.Shape.html) and [examples](https://github.com/loony-bean/textplots-rs/tree/master/examples) for more details.
//!
//! <img src="https://github.com/loony-bean/textplots-rs/blob/master/doc/demo3.png?raw=true"/>

pub mod scale;
pub mod utils;

use colored::*;
use drawille::Canvas as BrailleCanvas;
pub use drawille::PixelColor;
use scale::Scale;
use std::any::type_name;
use std::default::Default;
use std::f64;
use std::{cmp, fmt::Display};

pub struct Point<
    T: Into<f64> + From<f64> + Display + Clone,
    U: Into<f64> + From<f64> + Display + Clone,
> {
    pub x: T,
    pub y: U,
}

/// Controls the drawing.
pub struct Chart<'a, T, U>
where
    T: Into<f64> + From<f64> + Display + Clone,
    U: Into<f64> + From<f64> + Display + Clone,
{
    /// Canvas width in points.
    width: u32,
    /// Canvas height in points.
    height: u32,
    /// X-axis start value.
    xmin: f64,
    /// X-axis end value.
    xmax: f64,
    /// Y-axis start value (calculated automatically to display all the domain values).
    ymin: f64,
    /// Y-axis end value (calculated automatically to display all the domain values).
    ymax: f64,
    /// Collection of shapes to be presented on the canvas.
    shapes: Vec<(&'a Shape<'a, Point<T, U>>, PixelColor)>,
    /// Labels associated with shapes on the screen (label will print out in associated PixelColor).
    labels: Vec<(String, PixelColor)>,
    /// Underlying canvas object.
    canvas: BrailleCanvas,
}

/// Specifies different kinds of plotted data.
pub enum Shape<'a, Point> {
    /// Real value function.
    Continuous(Box<dyn Fn(f64) -> f64 + 'a>),
    /// Points of a scatter plot.
    Points(&'a [Point]),
    /// Points connected with lines.
    Lines(&'a [Point]),
    /// Points connected in step fashion.
    Steps(&'a [Point]),
    /// Points represented with bars.
    Bars(&'a [Point]),
}

/// Provides an interface for drawing plots.
pub trait Plot<'a, T, U>
where
    T: Into<f64> + From<f64> + Display + Clone,
    U: Into<f64> + From<f64> + Display + Clone,
{
    /// Draws a [line chart](https://en.wikipedia.org/wiki/Line_chart) of points connected by straight line segments.
    fn lineplot(&'a mut self, shape: &'a Shape<Point<T, U>>) -> &'a mut Chart<'a, T, U>;
    /// Tags drawing in a line chart, with an optional label and a specified colour.
    fn lineplot_with_tags(
        &'a mut self,
        shape: &'a Shape<Point<T, U>>,
        label: Option<String>,
        colour: PixelColor,
    ) -> &'a mut Chart<'a, T, U>;
}

impl<T, U> Default for Chart<'_, T, U>
where
    T: Into<f64> + From<f64> + Display + Clone,
    U: Into<f64> + From<f64> + Display + Clone,
{
    fn default() -> Self {
        Self::new(120, 60, -10.0, 10.0)
    }
}

impl<T, U> Chart<'_, T, U>
where
    T: Into<f64> + From<f64> + Display + Clone,
    U: Into<f64> + From<f64> + Display + Clone,
{
    /// Creates a new `Chart` object.
    ///
    /// # Panics
    ///
    /// Panics if `width` or `height` is less than 32.
    pub fn new(width: u32, height: u32, xmin: f64, xmax: f64) -> Self {
        if width < 32 {
            panic!("width should be more then 32, {} is provided", width);
        }

        if height < 32 {
            panic!("height should be more then 32, {} is provided", height);
        }

        Self {
            xmin,
            xmax,
            ymin: f64::INFINITY,
            ymax: f64::NEG_INFINITY,
            width,
            height,
            shapes: vec![],
            labels: vec![],
            canvas: BrailleCanvas::new(width, height),
        }
    }

    /// Displays bounding rect.
    fn borders(&mut self) {
        let w = self.width;
        let h = self.height;

        self.vline(0);
        self.vline(w);
        self.hline(0);
        self.hline(h);
    }

    /// Draws vertical line.
    fn vline(&mut self, i: u32) {
        if i <= self.width {
            for j in 0..=self.height {
                if j % 3 == 0 {
                    self.canvas.set(i, j);
                }
            }
        }
    }

    /// Draws horizontal line.
    fn hline(&mut self, j: u32) {
        if j <= self.height {
            for i in 0..=self.width {
                if i % 3 == 0 {
                    self.canvas.set(i, self.height - j);
                }
            }
        }
    }

    /// Prints canvas content.
    pub fn display(&mut self) {
        self.figures();
        self.axis();

        let frame = self.canvas.frame();
        let rows = frame.split('\n').count();
        for (i, row) in frame.split('\n').enumerate() {
            if i == 0 {
                let ymax: U = self.ymax.into();
                println!("{0} {1:.1}", row, ymax);
            } else if i == (rows - 1) {
                let ymin: U = self.ymin.into();
                println!("{0} {1:.1}", row, ymin);
            } else {
                println!("{}", row);
            }
        }

        let xmin: T = self.xmin.into();
        let xmax: T = self.xmax.into();
        if type_name::<T>() == type_name::<f64>() {
            println!(
                "{0: <width$.1}{1:.1}",
                self.xmin,
                self.xmax,
                width = (self.width as usize) / 2 - 3
            );
        } else {
            // properly balance x labels when printing
            let label_lengths: usize = xmin.to_string().len() + xmax.to_string().len();
            println!(
                "{0: <spacing$}{1:spacing$}{2:}",
                xmin,
                " ",
                xmax,
                spacing = (self.width as usize) / 2 - label_lengths
            );
        }
    }
    /// Prints a legend that gives names to shapes and aligns them to colours.
    pub fn legends(&mut self) {
        println!("");
        for label in &self.labels {
            println!("{}", format!("{}: ⠉⠉⠉", label.0).color(label.1));
        }
    }
    /// Prints canvas content with some additional visual elements (like borders and a legend).
    pub fn nice(&mut self) {
        self.borders();
        self.display();
        self.legends();
    }

    /// Show axis.
    pub fn axis(&mut self) {
        let x_scale = Scale::new(self.xmin..self.xmax, 0.0..self.width as f64);
        let y_scale = Scale::new(self.ymin..self.ymax, 0.0..self.height as f64);

        if self.xmin <= 0.0 && self.xmax >= 0.0 {
            self.vline(x_scale.linear(0.0) as u32);
        }
        if self.ymin <= 0.0 && self.ymax >= 0.0 {
            self.hline(y_scale.linear(0.0) as u32);
        }
    }

    // Show figures.
    pub fn figures(&mut self) {
        for shape in &self.shapes {
            let x_scale = Scale::new(self.xmin..self.xmax, 0.0..self.width as f64);
            let y_scale = Scale::new(self.ymin..self.ymax, 0.0..self.height as f64);

            // translate (x, y) points into screen coordinates
            let points: Vec<_> = match shape {
                (Shape::Continuous(f), color) => (0..self.width)
                    .filter_map(|i| {
                        let x = x_scale.inv_linear(i as f64);
                        let y = f(x);
                        if y.is_normal() {
                            let j = y_scale.linear(y).round();
                            Some((i, self.height - j as u32, color.clone()))
                        } else {
                            None
                        }
                    })
                    .collect(),
                (Shape::Points(dt), color)
                | (Shape::Lines(dt), color)
                | (Shape::Steps(dt), color)
                | (Shape::Bars(dt), color) => dt
                    .iter()
                    .filter_map(|point| {
                        let x: f64 = point.x.clone().into();
                        let y: f64 = point.y.clone().into();
                        let i = x_scale.linear(x).round() as u32;
                        let j = y_scale.linear(y).round() as u32;
                        if i <= self.width && j <= self.height {
                            Some((i, self.height - j, color.clone()))
                        } else {
                            None
                        }
                    })
                    .collect(),
            };

            // display segments
            match shape {
                (Shape::Continuous(_), _) | (Shape::Lines(_), _) => {
                    for pair in points.windows(2) {
                        let (x1, y1, color) = pair[0];
                        let (x2, y2, _) = pair[1];

                        self.canvas.line_colored(x1, y1, x2, y2, color);
                    }
                }
                (Shape::Points(_), _) => {
                    for (x, y, color) in points {
                        self.canvas.set_colored(x, y, color);
                    }
                }
                (Shape::Steps(_), _) => {
                    for pair in points.windows(2) {
                        let (x1, y1, color) = pair[0];
                        let (x2, y2, _) = pair[1];

                        self.canvas.line_colored(x1, y2, x2, y2, color);
                        self.canvas.line_colored(x1, y1, x1, y2, color);
                    }
                }
                (Shape::Bars(_), _) => {
                    for pair in points.windows(2) {
                        let (x1, y1, color) = pair[0];
                        let (x2, y2, _) = pair[1];

                        self.canvas.line_colored(x1, y2, x2, y2, color);
                        self.canvas.line_colored(x1, y1, x1, y2, color);
                        self.canvas.line_colored(x1, self.height, x1, y1, color);
                        self.canvas.line_colored(x2, self.height, x2, y2, color);
                    }
                }
            }
        }
    }

    /// Return the frame.
    pub fn frame(&self) -> String {
        self.canvas.frame()
    }
}

impl<'a, T, U> Plot<'a, T, U> for Chart<'a, T, U>
where
    T: Into<f64> + From<f64> + Display + Clone,
    U: Into<f64> + From<f64> + Display + Clone,
{
    fn lineplot_with_tags(
        &'a mut self,
        shape: &'a Shape<Point<T, U>>,
        label: Option<String>,
        colour: PixelColor,
    ) -> &'a mut Chart<'a, T, U> {
        self.shapes.push((shape, colour));
        if let Some(thing) = label {
            self.labels.push((thing, colour));
        }
        // rescale ymin and ymax
        let x_scale = Scale::new(self.xmin..self.xmax, 0.0..self.width as f64);

        let ys: Vec<_> = match shape {
            Shape::Continuous(f) => (0..self.width)
                .filter_map(|i| {
                    let x = x_scale.inv_linear(i as f64);
                    let y = f(x);
                    if y.is_normal() {
                        Some(y)
                    } else {
                        None
                    }
                })
                .collect(),
            Shape::Points(dt) | Shape::Lines(dt) | Shape::Steps(dt) | Shape::Bars(dt) => dt
                .iter()
                .filter_map(|point| {
                    let x: f64 = point.x.clone().into();
                    let y: f64 = point.y.clone().into();
                    if x >= self.xmin && x <= self.xmax {
                        Some(y)
                    } else {
                        None
                    }
                })
                .collect(),
        };

        let ymax = *ys
            .iter()
            .max_by(|x, y| x.partial_cmp(y).unwrap_or(cmp::Ordering::Equal))
            .unwrap_or(&0.0);
        let ymin = *ys
            .iter()
            .min_by(|x, y| x.partial_cmp(y).unwrap_or(cmp::Ordering::Equal))
            .unwrap_or(&0.0);

        self.ymin = f64::min(self.ymin, ymin);
        self.ymax = f64::max(self.ymax, ymax);

        self
    }

    fn lineplot(&'a mut self, shape: &'a Shape<Point<T, U>>) -> &'a mut Chart<'a, T, U> {
        self.lineplot_with_tags(shape, None, PixelColor::White)
    }
}
