use std::vec::Vec;
use std::collections::HashMap;
use rayon::prelude::*;
// use rayon::par_iter::ParallelIterator;

use super::Task;

struct Collector {
    task: Task,
    entries: HashMap<String, ProcessorEntity>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProcessorEntity {
    pub name: String,
    pub collector_tasks: HashMap<String, String>,
    pub sync_tasks: HashMap<String, String>,
    pub async_tasks: HashMap<String, String>
}

pub struct ProcessorHub {
    collectors: Vec<Task>,
    processors: HashMap<String, Collector>
}

impl ProcessorHub {
    pub fn new() -> ProcessorHub {
        ProcessorHub {
            collectors: Vec::new(),
            processors: HashMap::new()
        }
    }

    pub fn setup(
        &mut self, processors: Vec<ProcessorEntity>, tasks: HashMap<String, Task>
    ) {
        self.collectors = tasks.iter().map(|(_, task)| task.clone()).collect();

        processors.into_iter().for_each(|p| {
            p.collector_tasks.par_iter().for_each(|(col_name, recept_prop)| {
                let rp = recept_prop.clone();
                let pro = p.clone();

                if self.processors.contains_key(col_name) {
                    self.processors[col_name].entries.insert(rp, pro);
                } else {
                    let mut entries:HashMap<String, ProcessorEntity> = HashMap::new();
                    entries.insert(rp, pro);

                    self.processors.insert(col_name.clone(), Collector{
                        task: tasks[col_name].clone(),
                        entries: entries
                    });
                }
            });
        });
    }

    pub fn start(&self) {

    }
}
