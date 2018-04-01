extern crate appa;

use std::env;
use appa::config::AppaConfig;
use appa::queue::AppaQueue;

fn main() {
    let args: Vec<String> = env::args().collect();
    let set = AppaConfig::new(args[1].to_string());
    let queue = AppaQueue::new(set);

    queue.exec();
}