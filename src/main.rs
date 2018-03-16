extern crate appa;

use appa::config::AppaConfig;
use appa::queue::AppaQueue;

fn main() {
    let set = AppaConfig::new("tests/mocks/config1.yml".to_string());
    let queue = AppaQueue::new(set);

    queue.exec();
}