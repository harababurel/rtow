extern crate clap;
extern crate num_cpus;
extern crate rtow;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use clap::Parser;
use rtow::Config;

fn main() {
    let cfg = Config::parse();
    pretty_env_logger::init();

    info!("Running with the following configuration: {:#?}", &cfg);
    rtow::run(cfg);
}
