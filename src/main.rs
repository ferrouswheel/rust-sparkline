extern crate sparkline;

use sparkline::*;

use std::env;

fn main() {
    let mut args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("{} expects a series of numbers as arguments.", args[0]);
        std::process::exit(1);
    }

    args.remove(0);
    let good_numbers = parse_numbers(&args);

    let (min, max) = min_max_for_data(&good_numbers);

    let sparky = select_sparkline(SparkThemeName::Color);

    for num in good_numbers.iter() {
        let s = sparky.spark(min, max, *num);
        print!("{} ", s);
    }
    println!("");

    println!("min: {}, max: {}, range: {}", min, max, max-min);
}

