> A fork of textplot-rs that allows you to customize the display of your labels for any T, U
>
> This works by operating over Point<T,U> where T/U implement  Display, Into<f64>, From<f64>, Clone
> 
> Additionally, this now assumes that each line is associated with a PixelColor to help add detail and visually separate out each plotted line.
# pointplot

A terminal plotting library for using in Rust CLI applications. This should work well in any unicode terminal with monospaced font.

It is inspired by [textplot](https://github.com/loony-bean/textplots-rs) which is inspired from [TextPlots.jl](https://github.com/sunetos/TextPlots.jl) which is inspired by [Drawille](https://github.com/asciimoo/drawille).

Currently it features only drawing line charts on Braille canvas, adding labels to the bottom of charts, coloring lines, and adding axis labels through the display trait. It could be extended to support other canvas and chart types just like [UnicodePlots.jl](https://github.com/Evizero/UnicodePlots.jl) or another cool terminal plotting library.

Contributions are very much welcome!

# Usage

## Using as a library

```rust
use pointplotss::{Chart, Plot, Shape};

fn main() {
    println!("y = sin(x) / x");
    
    Chart::default()
        .lineplot(&Shape::Continuous(Box::new(|x| x.sin() / x)))
        .display();
}
```

<img src="https://raw.githubusercontent.com/loony-bean/textplots-rs/master/doc/demo.png">

## Using as a binary

```sh
$ pointplots '10*x + x^2 + 10*sin(x)*abs(x)' --xmin=-20 --xmax=20
```

<img src="https://raw.githubusercontent.com/loony-bean/textplots-rs/master/doc/demo4.png">
