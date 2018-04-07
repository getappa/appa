extern crate appa;

use appa::core::consumer;
use appa::config::{ConfigurationFile, ProcessorHub};

fn main () {
    let config = ConfigurationFile::new("tests/mocks/config1.yml");
    let tasks = config.tasks_as_map();
    let hub = ProcessorHub::new();

    hub.setup(config.processors, tasks);
    // processor.start();
}