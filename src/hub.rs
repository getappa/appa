use std::collections::HashMap;
use rayon::prelude::*;
use uuid::Uuid;
use serde_json::{Value, from_str, Result};

use super::{
    Task,
    ProcessEntity,
    ConfigurationFile,
    RocksDbStorage,
    consumer
};

enum IdRef {
    RefStr(String),
    RefUid(Uuid)
}

pub struct Collector {
    pub entries: HashMap<String, String>,
    pub task: String
}

pub struct Hub {
    tasks: HashMap<String, Task>,
    collectors: Vec<Collector>,
    processors: HashMap<String, ProcessEntity>,
    storage: RocksDbStorage
}

impl Hub {
    pub fn new(config: ConfigurationFile) -> Hub {
        let storage = RocksDbStorage::new(config.storage_uri.clone());

        Hub{
            tasks: config.tasks_as_map(),
            collectors: config.collectors_as_vec(),
            processors: config.processors_as_map(),
            storage: storage
        }
    }

    pub fn start(self) {
        self.collectors.par_iter().for_each(|collector| {
            let cmd = self.get_task(&collector.task).get_cmd("");
            consumer::exec(cmd, |d| {
                for (tag, entity) in &collector.entries {
                    let ef = format!("!AppaTag({})", tag);
                    if d.contains(&ef) {
                        let processor = self.get_processor(&entity);
                        let project = self.storage.project(processor.name.clone());
                        let data = d.replace(&ef, "");
                        let json:Result<Value> = from_str(&data);
                        let done = json.ok().unwrap();
                        let arr = done.as_array().unwrap();

                        arr.par_iter().for_each(|item| {
                            let core_id = if processor.id_prop == "" {
                                IdRef::RefUid(Uuid::new_v4())
                            } else {
                                IdRef::RefStr(
                                    item[processor.id_prop.clone()].to_string()
                                )
                            };

                            let key = match core_id {
                                IdRef::RefUid(ref x) => x.as_bytes(),
                                IdRef::RefStr(ref y) => y.as_bytes()
                            };

                            project.put_string(key, data.clone());

                            processor.sync_tasks.iter().for_each(|(task, prop)| {
                                let str_data = project.get_bytes(key.clone());
                                let cmd = self.tasks[task].get_cmd(&str_data);

                                consumer::exec(cmd, |d| {
                                    println!("{}", d);
                                    // project.update_json(key.clone(), prop.clone(), d);
                                }, |e| {
                                    println!("Err on task: {:?}", e);
                                });
                            });

                            let str_data = project.get_bytes(key.clone());
                            processor.async_tasks.par_iter().for_each(|(task, prop)| {
                                consumer::exec(self.tasks[task].get_cmd(&str_data), |d| {
                                    println!("{}", d);
                                    // project.update_json(key.clone(), prop.clone(), d);
                                }, |e| {
                                    println!("{:?}", e);
                                });
                            });
                        });
                    }
                }
            }, |e| {
                println!("{:?}", e);
            });
        });
    }

    fn get_processor(&self, entity: &str) -> &ProcessEntity {
        &self.processors[entity]
    }

    fn get_task(&self, name: &String) -> &Task {
        &self.tasks[name]
    }
}