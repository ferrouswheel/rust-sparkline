extern crate sparkline;
extern crate rustc_serialize;
extern crate docopt;
extern crate lodepng;
extern crate num;

use sparkline::*;

use std::io;
use std::io::BufRead;
use std::path::Path;

use lodepng::RGB;
use rustc_serialize::base64::{self, ToBase64};
use docopt::Docopt;

use num::traits::{ Float, ToPrimitive };

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
        Some(ref x) if x == "colour" => &*x,
        Some(ref x) if x == "classic" => &*x,
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
    let width = 200;
    let height = 30;
    let mut image : Vec<u8> = vec![0; (width * height * 4) as usize];

    let length = good_numbers.len();
    for (i, num) in good_numbers.iter().enumerate() {
        draw_number(&mut image, width, height, min, max, length, i, num);
    }

    //Result<CVec<u8>, Error>
    let png_mem = lodepng::encode_memory(&image, width, height, lodepng::LCT_RGBA, 8).ok().expect("Failed to generate PNG");
    
    let mut png_array : Vec<u8> = Vec::with_capacity(png_mem.len());
    for i in (0..png_mem.len()) {
        png_array.push(*png_mem.get(i).unwrap());
    }
    print!("\x1B]1337;File=inline=1:");
    print!("{}", png_array.to_base64(base64::STANDARD));
    println!("\x07");

    let path = &Path::new("write_test.png");
    // code assumes we are 4 bytes ber pixel
    let result = lodepng::encode_file(path, &image, width, height, lodepng::LCT_RGBA, 8);

    match result {
        Ok(_) => println!("ok"),
        Err(e) => println!("{}", e),
    }
    
}

fn draw_number(image : &mut Vec<u8>, width: usize, height: usize, min: f64, max: f64, total_numbers : usize, pos: usize, num: &f64) {
    let segment_size = width / total_numbers;

    let x = (segment_size * pos) as f64;
    let y = (height as f64 / (max - min)) * (*num - min);

    let x_i : usize = x as usize;
    let y_i : usize =
        if y >= (height as f64) {
            0
        } else if y < 1.0 {
            height - 1
        } else {
            height - (y as usize)
        };

    let pixel_pos = (y_i * width + x_i) * 4;
    //println!("x: {} y: {} pixel: {}", x_i, y_i, pixel_pos);
    image[pixel_pos] = 254; 
    image[pixel_pos + 1] = 254;
    image[pixel_pos + 2] = 254;
    image[pixel_pos + 3] = 254;

    let colours = [
        [170u8, 60, 57, 255,],
        [255u8, 218, 41, 255,],
        [54u8, 186, 46, 255,]
        ];
    let proportion = (*num - min) / (max - min);
    let mut proportion = (colours.len() as f64) * proportion;
    if proportion == colours.len() as f64 {
        proportion = proportion - 1.0;
    }
    let colour = colours[proportion.to_usize().unwrap()];

    fill_bar(image, width, x_i, x_i + (segment_size - 1), y_i, height - 0, &colour);
}

fn fill_bar(image : &mut Vec<u8>, width: usize, x1: usize, x2 : usize, y1 : usize, y2: usize, colour: &[u8; 4] ) {
    assert!(x1 < x2);
    assert!(y1 < y2);


    for x in (x1 .. x2) {
        for y in (y1 .. y2) {
            let pixel_pos = (y * width + x) * 4;
            for (p, c) in colour.iter().enumerate() {
                image[pixel_pos + p] = *c; 
            }
        }
    }
}
