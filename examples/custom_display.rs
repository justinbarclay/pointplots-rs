use core::fmt;

use textplots::{utils, Chart, Labelable, Plot, Point, Shape};

#[derive(Clone)]
struct Temp(f32);

#[derive(Clone)]
enum Month {
    January,
    Febuary,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl fmt::Display for Temp {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("{}°C", self.0))
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Month::January => f.write_str("January"),
            Month::Febuary => f.write_str("Febuary"),
            Month::March => f.write_str("March"),
            Month::April => f.write_str("April"),
            Month::May => f.write_str("May"),
            Month::June => f.write_str("June"),
            Month::July => f.write_str("July"),
            Month::August => f.write_str("August"),
            Month::September => f.write_str("September"),
            Month::October => f.write_str("October"),
            Month::November => f.write_str("November"),
            Month::December => f.write_str("December"),
        }
    }
}

impl From<Month> for f32 {
    fn from(month: Month) -> Self {
        match month {
            Month::January => 0.,
            Month::Febuary => 1.,
            Month::March => 2.,
            Month::April => 3.,
            Month::May => 4.,
            Month::June => 5.,
            Month::July => 6.,
            Month::August => 7.,
            Month::September => 8.,
            Month::October => 9.,
            Month::November => 10.,
            Month::December => 11.,
        }
    }
}

impl From<f32> for Month {
    fn from(number: f32) -> Self {
        match number.floor() as u32 {
            0 => Month::January,
            1 => Month::Febuary,
            2 => Month::March,
            3 => Month::April,
            4 => Month::May,
            5 => Month::June,
            6 => Month::July,
            7 => Month::August,
            8 => Month::September,
            9 => Month::October,
            10 => Month::November,
            11 => Month::December,
            _ => panic!("Can not convert {} to month.", number),
        }
    }
}

impl From<Temp> for f32 {
    fn from(temp: Temp) -> Self {
        temp.0
    }
}

impl From<f32> for Temp {
    fn from(number: f32) -> Self {
        Temp(number)
    }
}

impl Labelable for Month {}
impl Labelable for Temp {}

fn main() {
    let data = [
        (Month::January, Temp(-8.0)),
        (Month::Febuary, Temp(-8.0)),
        (Month::March, Temp(-3.0)),
        (Month::April, Temp(5.0)),
        (Month::May, Temp(12.0)),
        (Month::June, Temp(16.0)),
        (Month::July, Temp(19.0)),
        (Month::August, Temp(18.0)),
        (Month::September, Temp(13.0)),
        (Month::October, Temp(6.0)),
        (Month::November, Temp(-4.0)),
        (Month::December, Temp(-10.0)),
    ];
    let points: Vec<Point<Month, Temp>> = data
        .into_iter()
        .map(|(x, y)| -> Point<Month, Temp> {
            Point {
                x: x.clone(),
                y: y.clone(),
            }
        })
        .collect();

    println!("\nMean Monthly Temperature in Edmonton, Alberta\n");
    Chart::<'_, Month, Temp>::new(120, 60, 0., 11.0)
        .lineplot(&Shape::Lines(&points))
        .display();
}
