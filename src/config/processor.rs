use std::vec::Vec;
use std::collections::HashMap;
use itertools::Itertools;

use super::Task;

#[derive(Clone)]
struct Entry {
    tag: String,
    pentity: ProcessorEntity
}

struct EntryIterator {
    on: String,
    entry: Entry,
}

struct Collector {
    task: Task,
    entries: Vec<Entry>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProcessorEntity {
    pub name: String,
    pub collector_tasks: HashMap<String, String>,
    pub sync_tasks: HashMap<String, String>,
    pub async_tasks: HashMap<String, String>
}

pub struct ProcessorHub {
    collectors: Vec<Collector>,
}

impl ProcessorHub {
    pub fn new(processors: Vec<ProcessorEntity>, tasks: HashMap<String, Task>) -> ProcessorHub {
        let mut c_entries:Vec<EntryIterator>= Vec::new();

        for p in processors {
            let ctasks = p.collector_tasks.clone();
            for (collector_name, tag) in ctasks {
                c_entries.push(EntryIterator{
                    on: collector_name,
                    entry: Entry {
                        tag: tag,
                        pentity: p.clone()
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

        ProcessorHub { collectors: collectors }
    }

    pub fn consume(&self) {
        // self.collectors.par_iter(|task| {
        //     let cmd = task.get_cmd();
        //     consume::run(cmd, "", |d| {
        //         let collector = self.processors[task.name];
        //         collector.entries.par_iter().for_each(|(k, p)| {
        //             p.exec_lifecycle(d);
        //         })
        //     }, |e| {
        //         println!("{:?}", e);
        //     });
        // });
    }
}
