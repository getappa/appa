extern crate appa;
extern crate rayon;

use rayon::prelude::*;
use appa::{
    consumer,
    error_handler,
    ConfigurationFile,
    ProcessorHub
};

fn main () {
    let config = ConfigurationFile::new("tests/mocks/config1.yml");
    let tasks = config.tasks_as_map();
    let hub = ProcessorHub::new(config.processors, tasks);

    hub.collectors.par_iter().for_each(|collector| {
        let command = collector.task.get_cmd("");
        consumer::exec(
            command,
            |d| consumer::entries(
                collector.entries.clone(), &d),
            |e| error_handler(
                &format!("Collector {}", collector.task.to_string()), e)
        );
    });
}