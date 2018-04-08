use std::vec::Vec;
use std::collections::HashMap;
use itertools::Itertools;

use super::processor::{ProcessorEntityFromYaml, Collector, Entry};
use super::Task;

struct EntryIterator {
    on: String,
    entry: Entry,
}

pub struct Hub {
    pub collectors: Vec<Collector>,
}

impl Hub {
    pub fn new(processors: Vec<ProcessorEntityFromYaml>, tasks: HashMap<String, Task>) -> Hub {
        let mut c_entries:Vec<EntryIterator>= Vec::new();

        for p in processors {
            let ctasks = p.collector_tasks.clone();
            for (collector_name, tag) in ctasks {
                c_entries.push(EntryIterator{
                    on: collector_name,
                    entry: Entry {
                        tag: tag,
                        pentity: p.convert_to_true_entity(tasks.clone())
                    }
                });
            }
        }

        let mut collectors: Vec<Collector> = Vec::new();
        for (cname, eiters) in &c_entries.iter().group_by(|e| e.on.clone()) {
            let mut entries:Vec<Entry> = Vec::new();
            for ei in eiters {
                entries.push(ei.entry.clone())
            }

            collectors.push(Collector {
                task: tasks[&cname].clone(),
                entries: entries
            });
        };

        Hub { collectors: collectors }
    }
}
