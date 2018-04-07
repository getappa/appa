mod task;
mod processor;

pub use self::task::Task;
pub use self::processor::{ProcessorHub, ProcessorEntity};

use serde_yaml;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ConfigurationFile {
    storage_uri: String,
    tasks: Vec<HashMap<String, String>>,
    processors: Vec<ProcessorEntity>
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

    pub fn tasks_as_map(&self) -> HashMap<String, Task> {
        let mut map:HashMap<String, Task> = HashMap::new();
        self.tasks.for_each(|t| {
            map.insert(
                t["name"],
                Task::new(
                    t["command"],
                    t["path"]
                )
            )
        });

        map
    }
}