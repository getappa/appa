extern crate appa;
extern crate rayon;

use appa::{consumer, ConfigurationFile, ProcessorHub};
use rayon::prelude::*;

fn main () {
    let config = ConfigurationFile::new("tests/mocks/config1.yml");
    let tasks = config.tasks_as_map();
    let hub = ProcessorHub::new(config.processors, tasks);

    hub.collectors.par_iter().for_each(|collector| {
        let command = collector.task.get_cmd("");
        consumer::exec(command, |d| {
            println!("{}", d);
        }, |e| {
            println!("{:?}", e);
        });
    });
}