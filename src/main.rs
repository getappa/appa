extern crate appa;

use appa::core::consumer;
use appa::config::{ConfigurationFile, ProcessorHub};

fn main () {
    let config = ConfigurationFile::new("tests/mocks/config1.yml");
    let processor = Processor::new(config.processors, config.tasks_as_map());

    // processor.start();
}