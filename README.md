# Sparklines for Rust

[![Build Status](https://travis-ci.org/ferrouswheel/rust-sparkline.svg?branch=master)](https://travis-ci.org/ferrouswheel/rust-sparkline)
[![](http://meritbadge.herokuapp.com/sparkline)](https://crates.io/crates/sparkline)

I needed a project to learn Rust. This is it!

Inspired by https://github.com/holman/spark and https://gist.github.com/stefanv/1371985

This provides a Rust library `sparkline` and executable `sparkr`.

## sparkr

```
$ sparkr --theme classic --min -10 2 4 0 3 9 10 8 2 5 6
▅ ▆ ▅ ▆ █ █ █ ▅ ▇ ▇
$ sparkr --statline --theme colour 2 4 0 3 9 10 8 2 5 6
▂ ▄ ▁ ▃ █ █ ▇ ▂ ▅ ▅
min: 0, max: 10, range: 10
$ sparkr -h
sparkr

Usage:
  sparkr [--min=<min>] [--max=<max>] [--theme=<theme>] [--statline] <values>...
  sparkr [--min=<min>] [--max=<max>] [--theme=<theme>] [--statline]
  sparkr (-h | --help)
  sparkr --version

Options:
  -h --help       Show this screen.
  --version       Show version.
  --min=<min>     Specify minimum value instead of calculating it.
  --max=<max>     Specify maximum value instead of calculating it.
  --statline      Show a line of stats after the sparkline.
  --theme=<theme>   What theme to use, 'colour' or 'classic' (default).
  <values>        Just values.
```

Note, if the spark line elements are not bottom-aligned, are irregular spaced,
or ther overlap one another, try another console font. Using "Menlo Regular" I get
nicely aligned, fixed-width, elements.

If you use iTerm2 then you can use `--theme png` to show the sparkline as an
[inline image](http://www.iterm2.com/images.html).

## library - sparkline

Add this to your `Cargo.toml`:
```
[dependencies]
sparkline=0.1
```

This takes a vec of numbers and prints out a sparkline:
```
extern crate sparkline;
use sparkline::*;

let (min, max) : (f64, f64) = (0.0, 10.0);
let values = vec![2.0, 3.0, 2.0, 6.0, 9.0];
let sparky = select_sparkline(SparkThemeName::Colour);
for num in values.iter() {
    let s : &String = sparky.spark(min, max, *num);
    print!("{} ", s);
}

```

Currently the library expects values to be `f64`.
