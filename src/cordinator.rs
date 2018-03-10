extern crate appa;

use appa:processor;
use appa:queue;

struct TaskProcessor;
impl TaskProcessor {
    fn run(&self, pe: processor::Command) -> String {
        processor::consume_task(pe)
    }
}

