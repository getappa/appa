use std::vec::Vec;
use std::collections::HashMap;
use rayon::prelude::*;

use super::Task;

#[derive(Serialize, Deserialize)]
pub struct ProcessorEntity {
    pub name: String,
    pub collector_tasks: HashMap<String, String>,
    pub sync_tasks: HashMap<String, String>,
    pub async_tasks: HashMap<String, String>
}

pub struct ProcessorHub {
    collectors: Vec<Task>,
    processors: HashMap<String,
        HashMap<String, ProcessorEntity>>
}

impl ProcessorHub {
    pub fn new(
        processors: Vec<ProcessorEntity>, tasks: HashMap<String, Task>
    ) -> ProcessorHub {
        let mut collectors:Vec<String> = Vec::new();
        let mut processor_set:HashMap<String,
            HashMap<String, ProcessorEntity>> = HashMap::new();

        processors.iter().for_each(|p| {
            p.collector_tasks.par_iter().for_each(|(col, pr)| {
                let collector = col.clone();
                let prop = pr.clone();

                if !collectors.contains(&collector) {
                    collectors.push(collector.clone());
                }

                if processor_set.contains_key(&collector.clone()) {
                    processor_set[&collector].insert(prop, *p);
                } else {
                    let mut hm:HashMap<String, ProcessorEntity> = HashMap::new();
                    hm.insert(prop, *p);
                    processor_set.insert(collector, hm);
                }
            });
        });

        ProcessorHub{
            collectors: collectors.iter().map(|name| tasks[name]).collect(),
            processors: processor_set
        }
    }

    pub fn start(&self) {

    }
}