use std::vec::Vec;
use std::collections::HashMap;
use itertools::Itertools;

use super::processor::{Collector, Entry};
use super::{
    Task,
    RocksDbStorage,
    ConfigurationFile,
};

struct EntryIterator {
    on: String,
    entry: &Entry,
}

pub struct Hub {
    pub collectors: Vec<&Collector>,
}

impl Hub {
    pub fn new(config: ConfigurationFile) -> Hub {
        let storage = RocksDbStorage::new(config.storage_uri);
        let tasks = config.tasks_as_map();
        let processors = config.processors;
        let mut c_entries:Vec<EntryIterator>= Vec::new();

        for p in processors {
            let project = storage.project(p.name);
            let ctasks = p.collector_tasks.clone();
            for (collector_name, tag) in ctasks {
                c_entries.push(EntryIterator{
                    on: collector_name,
                    entry: Entry {
                        tag: tag,
                        pentity: p.convert_to_true_entity(tasks.clone(), project)
                    }
                });
            }
        }

        let mut collectors: Vec<Collector> = Vec::new();
        for (cname, eiters) in &c_entries.iter().group_by(|e| e.on.clone()) {
            let mut entries:Vec<&Entry> = Vec::new();
            for ei in eiters {
                entries.push(ei.entry)
            }

            collectors.push(Collector {
                task: tasks[&cname].clone(),
                entries: entries
            });
        };

        Hub { collectors: collectors }
    }
}
