extern crate num;

use num::traits::{ Float, ToPrimitive };

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

pub trait SparkTheme {
    fn start(&mut self, min : f64, max : f64);
    fn spark(&mut self, num : f64) -> &str;
    fn end(&mut self) {}

    fn minmax(&self) -> (f64, f64);
    fn proportion(&self, num : f64) -> f64 {
        let (min, max) = self.minmax();

        (num - min) / (max - min)
    }
}

pub struct MappingTheme {
    pub sparks : Vec<String>,
    min : f64,
    max : f64,
}

impl SparkTheme for MappingTheme {
    fn start(&mut self, min : f64, max : f64) {
        self.min = min;
        self.max = max;
    }

    fn minmax(&self) -> (f64, f64) {
        (self.min, self.max)
    }

    fn spark(&mut self, num : f64) -> &str {
        let increments = self.sparks.len() as f64;

        let mut proportion = (increments) * self.proportion(num);

        // If num == max, then proportion will be out of bounds, so drop one
        if proportion == increments {
            proportion = proportion - 1.0;
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

pub fn select_sparkline(st : &str) -> Box<SparkTheme> {
    let sparks = "▁▂▃▄▅▆▇█";
    match st {
        "colour" => {
            let spark_chars : Vec<String> = sparks.chars().map(|x| colorise(&x.to_string())).collect();
            Box::new(MappingTheme {
                min: 0.0, max: 0.0,
                sparks: spark_chars
            })
        },
        _ => {
            Box::new(MappingTheme {
                min: 0.0, max: 0.0,
                sparks: sparks.chars().map(|x| x.to_string()).collect()
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

    sparky.start(min, max);
    for (num, compare) in values.iter().zip(expected.chars()) {
        let s : &str= sparky.spark(*num);
        println!("{}", num);
        assert_eq!(s, &compare.to_string());
    }

}
