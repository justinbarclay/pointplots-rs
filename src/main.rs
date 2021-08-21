use pointplots::{Chart, Plot, Shape};
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    /// Formula to plot
    #[structopt(name = "FORMULA")]
    formula: String,
    /// X-axis start value.
    #[structopt(long, default_value = "-10.0")]
    xmin: f64,
    /// X-axis end value.
    #[structopt(long, default_value = "10.0")]
    xmax: f64,
    /// Canvas width in points.
    #[structopt(short, long, default_value = "180")]
    width: u32,
    /// Canvas height in points.
    #[structopt(short, long, default_value = "60")]
    height: u32,
}

fn main() {
    let opt = Opt::from_args();

    let res = opt
        .formula
        .parse()
        .and_then(|expr: meval::Expr| expr.bind("x"));
    let func = match res {
        Ok(func) => func,
        Err(err) => {
            // if there was an error with parsing
            // or binding "x", exit with error

            eprintln!("{}", err);
            exit(1);
        }
    };

    println!("y = {}", opt.formula);
    Chart::<'_, f64, f64>::new(opt.width, opt.height, opt.xmin, opt.xmax)
        .lineplot(&Shape::Continuous(Box::new(|x| func(x.into()) as f64)))
        .display();
}
