use pointplots::{Chart, PixelColor, Plot, Point, Shape};

fn main() {
    // Display multiple plots.
    // https://github.com/loony-bean/textplots-rs/issues/8
    println!("y = -x^2; y = x^2");
    Chart::<'_, f64, f64>::default()
        .lineplot(&Shape::Continuous(Box::new(|x| (-x.powf(2.0)))))
        .lineplot(&Shape::Continuous(Box::new(|x| (x.powf(2.0)))))
        .display();

    // https://github.com/loony-bean/textplots-rs/issues/15
    let (mut l1, mut l2, mut l3) = (vec![], vec![], vec![]);
    for n in -2..=2 {
        l1.push(Point {
            x: n as f64,
            y: n as f64,
        });
        l2.push(Point {
            x: n as f64,
            y: n as f64 - 1.,
        });
        l3.push(Point {
            x: n as f64,
            y: n as f64 - 2.,
        });
    }

    println!("\nf(x)=x; f(x)=x-1; f(x)=x-2");
    Chart::new(120, 80, -2., 2.)
        .lineplot(&Shape::Lines(l1.as_slice()))
        .lineplot(&Shape::Lines(l2.as_slice()))
        .lineplot(&Shape::Lines(l3.as_slice()))
        .nice();

    let (mut l4, mut l5, mut l6) = (vec![], vec![], vec![]);
    for n in -2..=2 {
        l4.push(Point {
            x: n as f64,
            y: n as f64,
        });
        l5.push(Point {
            x: n as f64,
            y: n as f64 + 1.,
        });
        l6.push(Point {
            x: n as f64,
            y: n as f64 + 2.,
        });
    }

    println!("\nf(x)=x; f(x)=x+1; f(x)=x+2");
    Chart::new(120, 80, -2., 2.)
        .lineplot_with_tags(&Shape::Lines(l4.as_slice()), None, PixelColor::Yellow)
        .lineplot_with_tags(&Shape::Lines(l5.as_slice()), None, PixelColor::Red)
        .lineplot_with_tags(&Shape::Lines(l6.as_slice()), None, PixelColor::Green)
        .nice();
}
