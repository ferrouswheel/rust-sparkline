extern crate sparkline;
extern crate rustc_serialize;
extern crate docopt;
extern crate num;

use sparkline::*;

use std::io;
use std::io::BufRead;
use std::io::Write;
use std::fs::File;
use std::path::Path;

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
    pub flag_out: Option<types::OutputType>,
    pub flag_file: Option<String>,
    pub flag_statline: bool,
    pub arg_values: Vec<f64>,
}

fn main() {
    let mut args: Args = Docopt::new(USAGE)
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
    let mut default_fn = "sparkline.".to_owned();
    let mut sparky = select_sparkline(theme_name);
    default_fn.push_str(&(sparky.file_ext().to_owned()));
    //println!("theme name {}", sparky.name());

    {
        match sparky.validate_output_options(args.flag_out, &args.flag_file) {
            false => {
                panic!("Bad combination of output type and filename for {}", sparky.name())
            },
            _ => (),
        };
        let path = match args.flag_file {
            Some(ref x) => {
                // Filename specified on command line implies OutputType::File
                args.flag_out = Some(types::OutputType::File);
                Some(Path::new(x))
            },
            None => Some(Path::new(&*default_fn)),
        };
        let output_stream : Box<Write> = match args.flag_out {
            Some(types::OutputType::File) => {
                println!("Output filename is {:?}", path);
                let p = path.unwrap();
                Box::new(File::create(p).unwrap())
            },
            _ =>
                Box::new(std::io::stdout())
        };
        sparky.start(min, max, args.flag_out, output_stream);

        let gap_str : String = match args.flag_gap {
            Some(x) => std::iter::repeat(" ").take(x).collect(),
            None => " ".to_owned(),
        };
        let length = good_numbers.len();
        for (i, num) in good_numbers.iter().enumerate() {
            let s = sparky.spark(i, length, *num);
            print!("{}", match s {
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
