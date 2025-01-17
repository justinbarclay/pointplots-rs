//! Helpers for passing the data into plots.
//!
//! Merely a bunch of functions hanging around while the library API is taking shape.

use crate::Point;

/// Transforms points into frequency distribution (for using in histograms).
/// Values outside of [`min`, `max`] interval are ignored, and everything that
/// falls into the specified interval is grouped into `bins` number of buckets of equal width.
///
/// ```
/// # use pointplots::{utils::histogram, Point};
/// assert_eq!(vec![Point {x: 0.0, y: 1.0}, Point {x: 5.0, y: 1.0}], histogram( &[ (0.0, 0.0), (9.0, 9.0), (10.0, 10.0) ], 0.0, 10.0, 2 ));
/// ```

pub fn histogram(data: &[(f64, f64)], min: f64, max: f64, bins: usize) -> Vec<Point<f64, f64>> {
    let mut output = vec![0; bins];

    let step = (max - min) / bins as f64;

    for &(_x, y) in data.iter() {
        if y < min || y > max {
            continue;
        }

        let bucket_id = ((y - min) / step) as usize;
        if bucket_id < output.len() {
            output[bucket_id as usize] += 1;
        }
    }

    output
        .into_iter()
        .enumerate()
        .map(|(x, y)| -> Point<f64, f64> {
            Point {
                x: (min + (x as f64) * step),
                y: y as f64,
            }
        })
        .collect()
}

pub fn f64s_into_points(data: &[(f64, f64)]) -> Vec<Point<f64, f64>> {
    data.into_iter()
        .map(|(x, y)| -> Point<f64, f64> { Point { x: *x, y: *y } })
        .collect()
}
