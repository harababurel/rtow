#[macro_use]
extern crate clap;
extern crate rtow;

use clap::App;
use rtow::{Configuration, Resolution};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let resolution = matches.value_of("resolution").unwrap_or("800x400");
    let n_samples = matches.value_of("samples").unwrap_or("100");

    let cfg = Configuration {
        resolution: Resolution::from(resolution),
        n_samples: n_samples.parse::<u32>().unwrap_or(100),
        output_filename: matches.value_of("output").unwrap().to_string(),
    };

    rtow::run(&cfg);
}
