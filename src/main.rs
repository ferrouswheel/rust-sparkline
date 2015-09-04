extern crate sparkline;
extern crate rustc_serialize;
extern crate docopt;

use sparkline::*;

use std::io;
use std::io::BufRead;

use docopt::Docopt;

const USAGE: &'static str = "
sparkr

Usage:
  sparkr [--min=<min>] [--max=<max>] [--theme=<theme>] [--statline] [--gap=<gap>] [<values>...]
  sparkr (-h | --help)
  sparkr --version

Options:
  -h --help       Show this screen.
  --version       Show version.
  --min=<min>     Specify minimum value instead of calculating it.
  --max=<max>     Specify maximum value instead of calculating it.
  --gap=<max>     Gap between symbols [default=1]
  --statline      Show a line of stats after the sparkline.
  --theme=<theme>   What theme to use, 'colour' or 'classic' (default).
  <values>        Just values.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    pub flag_min: Option<f64>,
    pub flag_max: Option<f64>,
    pub flag_gap: Option<usize>,
    pub flag_theme: Option<String>,
    pub flag_statline: bool,
    pub arg_values: Vec<f64>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    //let mut args: Vec<_> = env::args().collect();
    //if args.len() < 2 {
        //println!("{} expects a series of numbers as arguments.", args[0]);
        //std::process::exit(1);
    //}

    //args.remove(0);
    //let good_numbers = parse_numbers(&args);
    
    let mut good_numbers: Vec<_> = args.arg_values;
    if good_numbers.len() == 0 {
        let mut input_numbers : Vec<String> = vec![];
        let stdin = io::stdin();

        for line in stdin.lock().lines() {
            match line {
                Ok(l) => {
                    input_numbers.extend(
                        l.split_whitespace()
                         .filter(|x| !x.is_empty())
                         .map(|x| x.to_owned()));
                },
                Err(_) => {
                    break;
                },
            };
        }
        good_numbers = parse_numbers(&input_numbers);
    }

    let (min, max) = min_max_for_data(&good_numbers, args.flag_min, args.flag_max);

    let theme_name : &str = match args.flag_theme {
        Some(ref x) if x == "color" => "colour",
        Some(ref x) if x == "colour" => &**x,
        Some(ref x) if x == "classic" => &**x,
        Some(ref x) => { println!("Unknown theme {} falling back to classic", x); "classic" },
        _ => "classic",
    };
    let mut sparky = select_sparkline(theme_name);
    sparky.start(min, max);

    let gap_str : String = match args.flag_gap {
        Some(x) => std::iter::repeat(" ").take(x).collect(),
        None => " ".to_owned(),
    };
    for num in good_numbers.iter() {
        let s = sparky.spark(*num);
        print!("{}{}", s, gap_str);
    }
    sparky.end();
    println!("");

    if args.flag_statline {
        println!("min: {}, max: {}, range: {}", min, max, max-min);
    }
}

