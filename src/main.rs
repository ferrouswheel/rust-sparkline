extern crate sparkline;
extern crate rustc_serialize;
extern crate docopt;
extern crate num;

use sparkline::*;

use std::io;
use std::fs::File;
use std::io::BufRead;
use std::io::Write;
use std::path::{self,Path};

use docopt::Docopt;

const USAGE: &'static str = "
sparkr

Usage:
  sparkr [--min=<min>] [--max=<max>] [--theme=<theme>] [--statline] [--gap=<gap>] [--out=<output>] [--file=<file>] [<values>...]
  sparkr (-h | --help)
  sparkr --version

Options:
  -h --help       Show this screen.
  --version       Show version.
  --min=<min>     Specify minimum value instead of calculating it.
  --max=<max>     Specify maximum value instead of calculating it.
  --gap=<gap>     Gap between symbols [default=1]
  --statline      Show a line of stats after the sparkline, on stderr.
  --theme=<theme>   What theme to use, 'colour', 'png', or 'classic' (default).
  --out=<output>  Destination for the sparkline, 'file', 'pipe', 'console' (default).
  --file=<file>   Filename for output. Implies --out=file. [default=sparkline.EXT]
  <values>        Just values.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    pub flag_min: Option<f64>,
    pub flag_max: Option<f64>,
    pub flag_gap: Option<usize>,
    pub flag_theme: Option<String>,
    pub flag_out: Option<OutputType>,
    pub flag_file: Option<String>,
    pub flag_statline: bool,
    pub arg_values: Vec<f64>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

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
        Some(ref x) if x == "color" => "colour", // 'murica
        Some(ref x) if x == "colour" => &*x,
        Some(ref x) if x == "classic" => &*x,
        Some(ref x) if x == "png" => &*x,
        Some(ref x) => { println!("Unknown theme {} falling back to classic", x); "classic" },
        _ => "classic",
    };
    let mut sparky = select_sparkline(theme_name);
    {
        match sparky.validate_output_options(args.flag_out, &args.flag_file) {
            false => {
                println!("Bad combination of output type and filename");
                panic!("eek")
            },
            _ => (),
        };
        let path = match args.flag_file {
            Some(ref x) => Some(Path::new(x)),
            None => None,
        };
        // Need to move this into the SparkTheme trait, an implementation
        // should be able to decide if a combination of path and OutputType
        // makes sense (or if a path is required)
        //let path = match combo {
            //(Some(OutputType::File), Some(ref x)) => Some(Path::new(x)),
            //(Some(OutputType::File), None) => None,
            //(Some(OutputType::Pipe), None) => None,
            //(Some(OutputType::Console), None) => None,
            //(Some(_), Some(_)) => None,
            //(None, Some(_)) => None,
            //(None, None) => None,
        //};
        println!("Output filename is {:?}", path);
        //let p = path.unwrap();
        //let mut f = File::create(p).unwrap();
        //f.write_all(b"Hello, world!");
        sparky.start(min, max, args.flag_out, path);

        let gap_str : String = match args.flag_gap {
            Some(x) => std::iter::repeat(" ").take(x).collect(),
            None => " ".to_owned(),
        };
        let length = good_numbers.len();
        for (i, num) in good_numbers.iter().enumerate() {
            let s = sparky.spark(i, length, *num);
            print!("{}{}", s, match s {
                "" => "",
                _ => &*gap_str,
            })
        }
        sparky.end();
    }
    println!("");

    if args.flag_statline {
        use std::io::Write;

        match writeln!(&mut std::io::stderr(), "min: {}, max: {}, range: {}", min, max, max-min) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    }

}
