use textplots::{Chart, Plot, Point, Shape};

fn main() {
    // Display multiple plots.
    // https://github.com/loony-bean/textplots-rs/issues/8
    println!("y = -x^2; y = x^2");
    Chart::<'_, f32, f32>::default()
        .lineplot(&Shape::Continuous(Box::new(|x| (-x.powf(2.0)))))
        .lineplot(&Shape::Continuous(Box::new(|x| (x.powf(2.0)))))
        .display();

    // https://github.com/loony-bean/textplots-rs/issues/15
    let (mut l1, mut l2, mut l3) = (vec![], vec![], vec![]);
    for n in -2..=2 {
        l1.push(Point {
            x: n as f32,
            y: n as f32,
        });
        l2.push(Point {
            x: n as f32,
            y: n as f32 - 1.,
        });
        l3.push(Point {
            x: n as f32,
            y: n as f32 - 2.,
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
            x: n as f32,
            y: n as f32,
        });
        l5.push(Point {
            x: n as f32,
            y: n as f32,
        });
        l6.push(Point {
            x: n as f32,
            y: n as f32 + 2.,
        });
    }

    println!("\nf(x)=x; f(x)=x+1; f(x)=x+2");
    Chart::new(120, 80, -2., 2.)
        .lineplot(&Shape::Lines(l4.as_slice()))
        .lineplot(&Shape::Lines(l5.as_slice()))
        .lineplot(&Shape::Lines(l6.as_slice()))
        .nice();
}
