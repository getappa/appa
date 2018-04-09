use std::vec::Vec;
use std::collections::HashMap;
use rayon::prelude::*;

use super::Task;
use super::storage::RockDbProject;

#[derive(Clone)]
pub struct Entry {
    pub tag: String,
    pub pentity: ProcessorEntity
}

pub struct Collector {
    pub task: Task,
    pub entries: Vec<Entry>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProcessorEntityFromYaml {
    pub name: String,
    pub collector_tasks: HashMap<String, String>,
    sync_tasks: HashMap<String, String>,
    async_tasks: HashMap<String, String>
}

impl ProcessorEntityFromYaml {
    pub fn convert_to_true_entity(&self, tasks: HashMap<String, Task>, p: RockDbProject) -> ProcessorEntity {
        let mut sync_tasks:HashMap<String, Task> = HashMap::new();
        let mut async_tasks:HashMap<String, Task> = HashMap::new();

        for (tname, prop) in self.sync_tasks.clone() {
            sync_tasks.insert(prop, tasks[&tname].clone());
        }

        for (tname, prop) in self.async_tasks.clone() {
            async_tasks.insert(prop, tasks[&tname].clone());
        }

        ProcessorEntity {
            name: self.name.clone(),
            storage: p,
            sync_tasks: sync_tasks,
            async_tasks: async_tasks,
        }
    }
}

#[derive(Clone)]
pub struct ProcessorEntity {
    pub name: String,
    storage: RockDbProject,
    sync_tasks: HashMap<String, Task>,
    async_tasks: HashMap<String, Task>
}

impl ProcessorEntity {
    pub fn set_storage(&self) {

    }

    pub fn process(&self, data: &str) {
        self.sync_tasks.iter().for_each(|(prop, task)| {

        });

        self.async_tasks.par_iter().for_each(|(prop, task)| {

        });
    }

    // A method to reduce the hole database
    //self.reduce.iter().for_each(|task| {
        // @TODO: Script reduce ALL data in
    // })
}
