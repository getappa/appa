extern crate appa;

// use std::env;
// use appa::{ Hub, ConfigurationFile };
use appa::cli;

pub fn main() {
    cli();

    // let args: Vec<String> = env::args().collect();
    // let config = ConfigurationFile::new(&args[1].to_string());
    // let hub = Hub::new(config);

    // hub.start();
}