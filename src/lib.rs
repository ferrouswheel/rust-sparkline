extern crate num;

extern crate rustc_serialize;

use num::traits::{ Float, ToPrimitive };
use std::io::Write;


pub mod types {
    use std::io::Write;

    #[derive(Copy, Clone, Debug, RustcDecodable)]
    pub enum OutputType { File, Pipe, Console }

    pub trait SparkTheme {
        fn name(&self) -> &'static str;
        fn file_ext(&self) -> &'static str {
            "txt"
        }
        fn validate_output_options(&self, ot : Option<OutputType>, file : &Option<String>) -> bool;

        fn start(&mut self, min : f64, max : f64, output : Option<OutputType>, out: Box<Write>);
        fn spark(&mut self, pos: usize, length: usize, num : f64) -> &str;
        fn end(&mut self) {}

        fn minmax(&self) -> (f64, f64);
        fn proportion(&self, num : f64) -> f64 {
            let (min, max) = self.minmax();

            (num - min) / (max - min)
        }
    }

}

#[cfg(unix)]
mod sparkimage {
    extern crate lodepng;
    use types::OutputType;
    use types::SparkTheme;
    use rustc_serialize::base64::{self, ToBase64};
    use num::traits::ToPrimitive;
    use std::io::Write;

    pub struct ImageTheme {
        min : f64,
        max : f64,
        width : usize,
        height : usize,
        image : Vec<u8>,
        output : Option<OutputType>,
        out : Option<Box<Write>>,
    }

    impl ImageTheme {
        pub fn new(width: usize, height: usize) -> ImageTheme {
            return ImageTheme {
                min: 0.0, max: 0.0,
                width: width, height: height,
                image: vec![0; (width * height * 4) as usize],
                output: None,
                out: None,
            }
        }

        fn draw_number(&mut self, total_numbers : usize, pos: usize, num: f64) {
            let segment_size = self.width / total_numbers;

            let (min, max) = self.minmax();
            let x = (segment_size * pos) as f64;
            let y = (self.height as f64 / (max - min)) * (num - min);

            let x_i : usize = x as usize;
            let y_i : usize =
                if y >= (self.height as f64) {
                    0
                } else if y < 1.0 {
                    self.height - 1
                } else {
                    self.height - (y as usize)
                };

            let colours = [
                [54u8, 186, 46, 255,],
                [255u8, 218, 41, 255,],
                [255u8, 218, 41, 255,],
                [220u8, 60, 57, 255,],
                ];
            let proportion = (num - min) / (max - min);
            let mut proportion = (colours.len() as f64) * proportion;
            if proportion == colours.len() as f64 {
                proportion = proportion - 1.0;
            }
            let colour = colours[proportion.to_usize().unwrap()];
            let h = self.height;
            self.fill_bar(x_i, x_i + (segment_size - 1), y_i, h - 0, &colour);
        }

        fn fill_bar(&mut self, x1: usize, x2 : usize, y1 : usize, y2: usize, colour: &[u8; 4] ) {
            assert!(x1 < x2);
            assert!(y1 < y2);


            for x in x1 .. x2 {
                for y in y1 .. y2 {
                    let pixel_pos = (y * self.width + x) * 4;
                    for (p, c) in colour.iter().enumerate() {
                        self.image[pixel_pos + p] = *c; 
                    }
                }
            }
        }

    }

    impl SparkTheme for ImageTheme {
        fn name(&self) -> &'static str {
            "png"
        }

        fn file_ext(&self) -> &'static str {
            "png"
        }

        fn validate_output_options(&self, ot : Option<OutputType>, file : &Option<String>) -> bool {
            let combo = (ot, file);
            match combo {
                (Some(OutputType::File), &Some(_)) => true,
                (Some(OutputType::File), &None) => true,
                (Some(OutputType::Pipe), &None) => true,
                (Some(OutputType::Console), &None) => true,
                (Some(_), &Some(_)) => false,
                (None, &Some(_)) => true,
                (None, &None) => true,
            }
        }

        fn start(&mut self, min : f64, max : f64, output : Option<OutputType>, out: Box<Write>) {
            self.min = min;
            self.max = max;
            self.output = output;
            self.out = Some(out);
        }

        fn minmax(&self) -> (f64, f64) {
            (self.min, self.max)
        }

        fn spark(&mut self, pos: usize, length: usize, num : f64) -> &str {
            self.draw_number(length, pos, num);
            return "";
        }

        fn end(&mut self) {
            let png_mem = lodepng::encode_memory(&self.image, self.width, self.height, lodepng::LCT_RGBA, 8)
                .ok()
                .expect("Failed to generate PNG");

            // This conversion to a Vec is wasteful, but I don't know how to easily iterate on a CVec
            // or get a u8 array from it.
            let mut png_array : Vec<u8> = Vec::with_capacity(png_mem.len());
            for i in 0 .. png_mem.len() {
                png_array.push(*png_mem.get(i).unwrap());
            }

            // Dump the png to term using iTerm2 extension:
            // http://www.iterm2.com/images.html
            print!("\n\x1B]1337;File=inline=1:");
            print!("{}", png_array.to_base64(base64::STANDARD));
            println!("\x07");

            // TODO: generally support dumping spark lines to files
            //let path = &Path::new("write_test.png");
            // code assumes we are 4 bytes ber pixel
            //let result = lodepng::encode_file(path, &self.image, self.width, self.height, lodepng::LCT_RGBA, 8);

            //match result {
                //Ok(_) => println!("ok"),
                //Err(e) => println!("{}", e),
            //}
        }
    }
}

use types::{OutputType, SparkTheme};

pub fn parse_numbers (args : &[String]) -> Vec<f64> {
    args.iter().enumerate().map(|(i, x)| {
        x.parse::<f64>().ok().expect(&format!("Argument \"{}\" was not a number :(", args[i]))
    }).collect()
}


/// Find the minimum and maximum for a slice of Float values.
///
/// ```
/// use sparkline::*;
/// let (x, y) = min_max_for_data(&vec![0.0, 1.0, 2.0], None, None);
/// assert_eq!(x, 0.0);
/// assert_eq!(y, 2.0);
/// let (x, y) = min_max_for_data(&vec![0.0, 1.0, 2.0], Some(-1.0), None);
/// assert_eq!(x, -1.0);
/// assert_eq!(y, 2.0);
/// let (x, y) = min_max_for_data(&vec![0.0, 1.0, 2.0], Some(1.0), None);
/// assert_eq!(x, 1.0);
/// assert_eq!(y, 2.0);
/// let (x, y) = min_max_for_data(&vec![0.0, 1.0, 2.0], None, Some(1.5));
/// assert_eq!(x, 0.0);
/// assert_eq!(y, 1.5);
/// ```
pub fn min_max_for_data<T>(numbers: &[T], min_opt: Option<T>, max_opt: Option<T>) -> (f64, f64) where T: Float {
    let max = match max_opt {
        Some(m) => m,
        None => numbers.iter().fold(T::min_value(), |a, b| a.max(*b)),
    };
    let min = match min_opt {
        Some(m) => m,
        None => numbers.iter().fold(T::max_value(), |a, b| a.min(*b)),
    };
    (min.to_f64().unwrap(), max.to_f64().unwrap())
}

pub struct MappingTheme {
    pub sparks : Vec<String>,
    name : &'static str,
    min : f64,
    max : f64,
    output : Option<OutputType>,
    out : Option<Box<Write>>,
}


impl SparkTheme for MappingTheme {
    fn name(&self) -> &'static str {
        return self.name;
    }

    fn validate_output_options(&self, ot : Option<OutputType>, file : &Option<String>) -> bool {
        let combo = (ot, file);
        match combo {
            (Some(OutputType::File), &Some(_)) => true,
            (Some(OutputType::File), &None) => true,
            (Some(OutputType::Pipe), &None) => true,
            (Some(OutputType::Console), &None) => true,
            (Some(_), &Some(_)) => false,
            (None, &Some(_)) => true,
            (None, &None) => true,
        }
    }

    fn start(&mut self, min : f64, max : f64, output : Option<OutputType>, out: Box<Write>) {
        self.min = min;
        self.max = max;
        self.output = output;
        self.out = Some(out);
    }

    fn minmax(&self) -> (f64, f64) {
        (self.min, self.max)
    }

    fn spark(&mut self, _pos: usize, _length: usize, num : f64) -> &str {
        let increments = self.sparks.len() as f64;

        let mut proportion = (increments) * self.proportion(num);

        // If num == max, then proportion will be out of bounds, so drop one
        if proportion == increments {
            proportion = proportion - 1.0;
        }

        let index = proportion.to_usize().unwrap();
        let next_char = self.sparks[index].clone().into_bytes();
        match self.out {
            Some(ref mut out) => { out.write(&next_char[..]).ok().expect("you crazy"); },
            None => (),
        }

        &self.sparks[proportion.to_usize().unwrap()]
    }
}



fn colorise(x : &str) -> String {
    let reset = "\x1B[0m";
    
    match x {
        "▁"|"▂" => "\x1B[0;32m".to_owned() + x + reset,
        "▃"|"▄" => "\x1B[0;33m".to_owned() + x + reset,
        "▅"|"▆" => "\x1B[0;33m".to_owned() + x + reset,
        "▇"|"█" => "\x1B[0;31m".to_owned() + x + reset,
        _ => x.to_string(),
    }
}

#[cfg(unix)]
pub fn select_sparkline<'a>(st : &'a str) -> Box<SparkTheme + 'a> {
    let sparks = "▁▂▃▄▅▆▇█";
    match st {
        "colour" => {
            let spark_chars : Vec<String> = sparks.chars().map(|x| colorise(&x.to_string())).collect();
            Box::new(MappingTheme {
                min: 0.0, max: 0.0,
                name: "colour",
                sparks: spark_chars,
                output: None,
                out: None
            })
        },
        "png" => {
            Box::new(sparkimage::ImageTheme::new(200, 30))
        },
        _ => {
            Box::new(MappingTheme {
                name: "classic",
                min: 0.0, max: 0.0,
                sparks: sparks.chars().map(|x| x.to_string()).collect(),
                output: None,
                out: None
            })
        },
    }
}

#[cfg(not(unix))]
pub fn select_sparkline<'a>(st : &'a str) -> Box<SparkTheme + 'a> {
    let sparks = "▁▂▃▄▅▆▇█";
    match st {
        "colour" => {
            let spark_chars : Vec<String> = sparks.chars().map(|x| colorise(&x.to_string())).collect();
            Box::new(MappingTheme {
                min: 0.0, max: 0.0,
                name: "colour",
                sparks: spark_chars,
                output: None,
                out: None
            })
        },
        _ => {
            Box::new(MappingTheme {
                name: "classic",
                min: 0.0, max: 0.0,
                sparks: sparks.chars().map(|x| x.to_string()).collect(),
                output: None,
                out: None
            })
        },
    }
}

#[test]
fn test_sparkline_mapping() {
    let (min, max) : (f64, f64) = (0.0, 10.0);
    let values = vec![2.0, 3.0, 2.0, 6.0, 9.0];
    let expected = "▂▃▂▅█".to_owned();
    let mut sparky = select_sparkline("classic");
    let out_stream = Box::new(std::io::stdout());

    sparky.start(min, max, None, out_stream);
    let length = values.len();
    for (pos, (num, compare)) in values.iter().zip(expected.chars()).enumerate() {
        let s : &str= sparky.spark(pos, length, *num);
        println!("{}", num);
        assert_eq!(s, &compare.to_string());
    }

}
