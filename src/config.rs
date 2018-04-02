use regex::Regex;

pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

pub struct Configuration {
    pub resolution: Resolution,
    pub n_samples: u32,
    pub output_filename: String,
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
