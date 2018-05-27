use serde_yaml;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use itertools::Itertools;

use super::{
    Task,
    ProcessEntity,
    Collector
};

struct Entry {
    pub tag: String,
    pub entity: String
}

struct EntryIterator {
    pub collector: String,
    pub entry: Entry
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigurationFile {
    pub storage_uri: String,
    pub processors: Vec<ProcessEntity>,
    pub tasks: Vec<HashMap<String, String>>,
}

impl ConfigurationFile {
    pub fn new(path: &str) -> ConfigurationFile {
        let mut file = File::open(&path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let ac:ConfigurationFile =
            serde_yaml::from_str(&data).unwrap();

        ac
    }

    pub fn save(&self, path: &str) {
        let new_yaml = serde_yaml::to_string(&self).unwrap();
        let mut file = File::create(&path).unwrap();

        file.write_all(new_yaml.as_bytes());
    }

    pub fn tasks_as_map(&self) -> HashMap<String, Task> {
        let mut map = HashMap::new();

        self.tasks.iter().for_each(|t| {
            let task = Task::new(&t["command"], &t["path"]);
            map.insert(t["name"].clone(), task);
        });

        map
    }

    pub fn processors_as_map(self) -> HashMap<String, ProcessEntity> {
        let mut map = HashMap::new();

        for p in self.processors.into_iter() {
            let name = p.name.clone();
            map.insert(name, p);
        };

        map
    }

    pub fn collectors_as_vec(&self) -> Vec<Collector> {
        let mut c_entries = Vec::new();

        for p in &self.processors {
            for (collector_name, tag) in &p.collector_tasks {
                c_entries.push(EntryIterator{
                    collector: collector_name.clone(),
                    entry: Entry {
                        tag: tag.clone(),
                        entity: p.name.clone()
                    }
                });
            }
        }

        let mut collectors: Vec<Collector> = Vec::new();
        for (cname, eiters) in &c_entries.iter().group_by(|e| e.collector.clone()) {
            let mut entries = HashMap::new();

            for ei in eiters {
                entries.insert(
                    ei.entry.tag.clone(),
                    ei.entry.entity.clone()
                );
            }

            let collector = Collector {
                task: cname.clone(),
                entries: entries
            };

            collectors.push(collector);
        };

        collectors
    }
}