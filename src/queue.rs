extern crate appa;

use std::thread;
use std::collections::VecDeque;

use appa:processor:Command;
use appa:processor:Runner;

pub struct ParallelQueue {
    queue: VecDeque<processor.Command>,
    done: VecDeque<String>
}

impl ParallelQueue {
    pub fn push(&self, String c, String p, String d) {
        &self.push(processor::Command{
            command: c,
            path: p,
            data: d
        })
    }

    pub fn consume(&self, Runner runner) {
        for task in &self.items {
            &self.push(runner(task));
        }
    }
}