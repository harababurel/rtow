use clap::Parser;
use regex::Regex;

use std::str::FromStr;

/// Models the size of an image.
#[derive(Clone, Debug)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

/// Describes the properties of the output image.
///
/// * `resolution`: self explanatory
/// * `n_samples`: the number of rays that are randomly sent through each pixel and then averaged
/// together; a high number of samples provides more accurate colors, less noise and better
/// anti-aliasing.
/// * `output_filename`: self explanatory
#[derive(Clone, Debug, Parser)]
pub struct Config {
    #[clap(long, default_value_t = Resolution::from_str("800x600").unwrap())]
    pub resolution: Resolution,
    #[clap(long, default_value_t = 10)]
    pub n_samples: u32,
    #[clap(long, default_value_t = String::from("out.png"))]
    pub output_filename: String,
    #[clap(long, default_value_t = num_cpus::get())]
    pub threads: usize,
}

impl<T> From<T> for Resolution
where
    T: Into<String>,
{
    fn from(s: T) -> Self {
        let s = s.into();

        let re = Regex::new(r"(\d+)[xX](\d+)").unwrap();
        let cap = re.captures(&s).unwrap();

        Resolution {
            width: cap[1].parse::<u32>().unwrap(),
            height: cap[2].parse::<u32>().unwrap(),
        }
    }
}

impl std::str::FromStr for Resolution {
    type Err = regex::Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let re = Regex::new(r"(\d+)[xX](\d+)")?;
        let cap = re.captures(s).unwrap();

        let width = cap[1].parse::<u32>().unwrap();
        let height = cap[2].parse::<u32>().unwrap();

        Ok(Resolution { width, height })
    }
}

impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}x{}", self.width, self.height)
    }
}
