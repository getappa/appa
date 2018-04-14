use super::Task;
use super::processor::ProcessorEntityFromYaml;

use serde_yaml;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ConfigurationFile {
    pub storage_uri: String,
    pub processors: Vec<ProcessorEntityFromYaml>,
    tasks: Vec<HashMap<String, String>>,
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
        let mut map = HashMap::new();

        self.tasks.iter().for_each(|t| {
            let task = Task::new(&t["command"], &t["path"]);
            map.insert(t["name"].clone(), task);
        });

        map
    }
}