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

pub fn min_max_for_data (numbers: &Vec<f64>, min_opt: Option<f64>, max_opt: Option<f64>) -> (f64, f64) {
    let max = match max_opt {
        Some(m) => m,
        None => numbers.iter().map(|x| x.round() as i64).max().unwrap() as f64,
    };
    let min = match min_opt {
        Some(m) => m,
        None => numbers.iter().map(|x| x.round() as i64).min().unwrap() as f64,
    };
    (min, max)
}

pub enum SparkThemeName {
    Classic,
    Color,
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
        SparkThemeName::Color => {
            let spark_chars : Vec<String> = sparks.chars().map(|x| colorise(&x.to_string())).collect();
            SparkTheme {
                sparks: spark_chars
            }
        },
    }
}


#[test]
fn it_works() {
}
