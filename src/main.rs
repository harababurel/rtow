#[macro_use]
extern crate clap;
extern crate num_cpus;
extern crate rtow;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use clap::App;
use rtow::{Configuration, Resolution};

fn main() {
    pretty_env_logger::init();

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let resolution = matches.value_of("resolution").unwrap_or("800x400");
    let n_samples = matches.value_of("samples").unwrap_or("100");
    let n_threads = matches.value_of("threads").unwrap_or_default();

    let cfg = Configuration {
        resolution: Resolution::from(resolution),
        n_samples: n_samples.parse::<u32>().unwrap_or(100),
        output_filename: matches.value_of("output").unwrap().to_string(),
        n_threads: n_threads.parse::<u32>().unwrap_or(num_cpus::get() as u32),
    };

    info!("Running with the following configuration: {:#?}", &cfg);
    rtow::run(cfg);
}
