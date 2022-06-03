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
/// * `samples`: the number of rays that are randomly sent through each pixel and then averaged
/// together; a high number of samples provides more accurate colors, less noise and better
/// anti-aliasing.
/// * `output_filename`: self explanatory
#[derive(Clone, Debug, Parser)]
pub struct Config {
    #[clap(short, long, default_value_t = Resolution::from_str("1080p").unwrap())]
    pub resolution: Resolution,
    #[clap(long, default_value_t = 60.0)]
    pub fov: f64,
    #[clap(short, long, default_value_t = 10)]
    pub samples: u32,
    #[clap(short, long, default_value_t = String::from("out.png"))]
    pub output_filename: String,
    #[clap(short, long, default_value_t = num_cpus::get())]
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
        match s {
            "720p" => Ok(Resolution {
                width: 1280,
                height: 720,
            }),
            "1080p" => Ok(Resolution {
                width: 1920,
                height: 1080,
            }),
            "4k" => Ok(Resolution {
                width: 3840,
                height: 2160,
            }),
            "8k" => Ok(Resolution {
                width: 7680,
                height: 4320,
            }),
            s_ => {
                let re = Regex::new(r"(\d+)[xX](\d+)")?;
                let cap = re.captures(s_).unwrap();

                let width = cap[1].parse::<u32>().unwrap();
                let height = cap[2].parse::<u32>().unwrap();

                Ok(Resolution { width, height })
            }
        }
    }
}

impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}x{}", self.width, self.height)
    }
}
