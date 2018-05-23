use std::collections::HashMap;
use rayon::prelude::*;
use uuid::Uuid;
use std::option::Option;
use std::vec::Vec;
use serde_json::{Value, from_str, to_string};

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

                        let arr = match from_str::<Option<Value>>(&data) {
                            Ok(Some(done)) => match done.as_array() {
                                Some(arr) => arr.clone(),
                                None => Vec::new()
                            },
                            Ok(None) => {
                                println!("Parser return NONE");
                                Vec::new()
                            },
                            Err(e) => {
                                println!("Error on parse collector data{:?}", e);
                                Vec::new()
                            }
                        };

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


                            let str_item = match to_string(item) {
                                Ok(i) => i,
                                Err(_) => String::new()
                            };

                            project.put_string(key, &str_item);

                            processor.sync_tasks.iter().for_each(|(task, prop)| {
                                let str_data = project.get_bytes(key.clone());
                                let cmd = self.tasks[task].get_cmd(&str_data);

                                consumer::exec(cmd, |d| {
                                    project.update_json(key, prop, &d);
                                }, |e| {
                                    println!("Err on task: {:?}", e);
                                });
                            });

                            let str_data = project.get_bytes(key.clone());
                            processor.async_tasks.par_iter().for_each(|(task, prop)| {
                                consumer::exec(self.tasks[task].get_cmd(&str_data), |d| {
                                    project.update_json(key, prop, &d);
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