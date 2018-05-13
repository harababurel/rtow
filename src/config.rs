use regex::Regex;

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
#[derive(Clone, Debug)]
pub struct Configuration {
    pub resolution: Resolution,
    pub n_samples: u32,
    pub output_filename: String,
    pub n_threads: u32,
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
