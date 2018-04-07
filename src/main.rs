extern crate appa;

use appa::core::consumer;
use appa::config::{ConfigurationFile, Processor};

fn main () {
    let config = AppaConfig::new("tests/mocks/config1.yml");
    let processor = Processor::new(config.processors, config.tasks_as_map());

    processor.start();
}