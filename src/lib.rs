use std::f64;

pub fn parse_numbers (args : &Vec<String>) -> Vec<f64> {
    let parsed_numbers = args.iter().map(|ref x| x.parse::<f64>()).collect::<Vec<_>>();
    let mut good_numbers = Vec::new();

    for (i, result) in parsed_numbers.iter().enumerate() {
        // Not sure if there is a way to do with without cloning "result"
        let num : f64 = result.clone().ok().expect(&*format!("Argument \"{}\" was not a number :(", args[i]));
        good_numbers.push(num);
    }

    good_numbers
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
pub fn min_max_for_data (numbers: &Vec<f64>, min_opt: Option<f64>, max_opt: Option<f64>) -> (f64, f64) {
    let max = match max_opt {
        Some(m) => m,
        None => numbers.iter().fold(f64::NEG_INFINITY, |a, b| a.max(*b)),
    };
    let min = match min_opt {
        Some(m) => m,
        None => numbers.iter().fold(f64::INFINITY, |a, b| a.min(*b)),
    };
    (min, max)
}

pub enum SparkThemeName {
    Classic,
    Colour,
}

pub struct SparkTheme {
    pub sparks : Vec<String>,
}

impl SparkTheme {
    pub fn spark(&self, min : f64, max : f64, num : f64) -> &String {
        let increments = self.sparks.len() as f64;

        let mut proportion = (increments) * ((num - min) / (max - min));

        // If num == max, then proportion will be out of bounds, so drop one
        if proportion == increments { proportion = proportion - 1.0; }

        &self.sparks[proportion as usize]
    }
}

fn colorise(x : &String) -> String {
    let reset = "\x1B[0m";
    
    match &**x {
        "▁"|"▂" => "\x1B[0;32m".to_string() + x + reset,
        "▃"|"▄" => "\x1B[0;33m".to_string() + x + reset,
        "▅"|"▆" => "\x1B[0;33m".to_string() + x + reset,
        "▇"|"█" => "\x1B[0;31m".to_string() + x + reset,
        _ => x.clone(),
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
        SparkThemeName::Colour => {
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
    let expected = "▂▃▂▅█".to_string();
    let sparky = select_sparkline(SparkThemeName::Classic);

    for (num, compare) in values.iter().zip(expected.chars()) {
        let s : &String = sparky.spark(min, max, *num);
        println!("{}", num);
        assert_eq!(*s, compare.to_string());
    }

}
