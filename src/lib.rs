extern crate num;

use num::traits::{ Float, ToPrimitive };

pub fn parse_numbers (args : &[String]) -> Vec<f64> {
    args.iter().enumerate().map(|(i, x)| {
        x.parse::<f64>().ok().expect(&format!("Argument \"{}\" was not a number :(", args[i]))
    }).collect()
}

/// Find the minimum and maximum of a vector of f64 values, but constained by
/// Option<f64> types for min and max.
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
pub fn min_max_for_data<T>(numbers: &[T], min_opt: Option<T>, max_opt: Option<T>) -> (T, T) where T: Float {
    let max = match max_opt {
        Some(m) => m,
        None => numbers.iter().fold(T::min_value(), |a, b| a.max(*b)),
    };
    let min = match min_opt {
        Some(m) => m,
        None => numbers.iter().fold(T::max_value(), |a, b| a.min(*b)),
    };
    (min, max)
}

pub enum SparkThemeName {
    Classic,
    Colour,
    Color,
}

pub struct SparkTheme {
    pub sparks : Vec<String>,
}

impl SparkTheme {
    pub fn spark<T>(&self, min : T, max : T, num : T) -> &String where T : Float{
        let increments = T::from(self.sparks.len()).unwrap();

        let mut proportion = (increments) * ((num - min) / (max - min));

        // If num == max, then proportion will be out of bounds, so drop one
        if proportion == increments {
            proportion = proportion - T::one();
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

pub fn select_sparkline(st : SparkThemeName) -> SparkTheme {
    let sparks = "▁▂▃▄▅▆▇█";
    match st {
        SparkThemeName::Classic => {
            SparkTheme {
                sparks: sparks.chars().map(|x| x.to_string()).collect()
            }
        },
        SparkThemeName::Colour | SparkThemeName::Color => {
            let spark_chars : Vec<String> = sparks.chars().map(|x| colorise(&x.to_string())).collect();
            SparkTheme {
                sparks: spark_chars
            }
        },
    }
}


#[test]
fn test_sparkline_mapping() {
    use SparkTheme;
    let (min, max) : (f64, f64) = (0.0, 10.0);
    let values = vec![2.0, 3.0, 2.0, 6.0, 9.0];
    let expected = "▂▃▂▅█".to_owned();
    let sparky = select_sparkline(SparkThemeName::Classic);

    for (num, compare) in values.iter().zip(expected.chars()) {
        let s : &String = sparky.spark(min, max, *num);
        println!("{}", num);
        assert_eq!(*s, compare.to_string());
    }

}
