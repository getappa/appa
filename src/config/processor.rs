use std::vec::Vec;
use std::collections::HashMap;

use super::Task;

#[derive(Serialize, Deserialize)]
pub struct ProcessorEntity {
    pub name: String,
    pub collector_tasks: Vec<HashMap<String, String>>,
    pub sync_task: Vec<HashMap<String, String>>,
    pub async_task: Vec<HashMap<String, String>>
}

pub struct ProcessorHub {
    collectors: Vec<Task>,
    processors: HashMap<String,
        HashMap<String, ProcessorEntity>>
}

impl ProcessorHub {
    pub fn new(processors: Vec<ProcessorEntity>, tasks: HashMap<String, Task>) -> ProcessorHub {
        let mut collectors:Vec<String> = Vec::new();
        let mut processor_set = HashMap<String, HashMap<String, ProcessorEntity>>

        processors.for_each(|p| {
            p.collector_tasks.par_iter(|collector, prop| {
                if !collectors.contains_key(collector) {
                    collectors.push(collector);
                }

                if !processor_set.contains_key(collector) {
                    let mut hm:HashMap<String, ProcessorEntity> = HashMap::new();
                    hm.insert(prop, p);
                    processor_set.insert(collector, hm);
                } else {
                    processor_set[collector].insert(prop, p)
                }
            });
        });

        ProcessorHub{
            collectors: collectors.map(|name| tasks.get(name)),
            processors: *processor_set
        }
    }

    pub fn start(&self) {

    }
}