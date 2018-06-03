use std::collections::HashMap;
use super::super::super::ConfigurationFile;
use super::{Task, TaskUpdate};

pub fn new(file: &str, task: &Task) {
    let mut config = ConfigurationFile::new(file);
    config.tasks.push(task.to_hashmap());
    config.save(&file);
}

pub fn update(file: &str, task: &TaskUpdate) {
    let mut config = ConfigurationFile::new(file);
    let key = String::from("name");

    config.tasks
        .iter_mut()
        .find(|ref mut t| t[&key] == task.name)
        .map(|t| {
            if task.command != "" {
                t.insert(String::from("command"), task.command.clone());
            }

            if task.path != "" {
                t.insert(String::from("path"), task.path.clone());
            }

            t
        });

    config.save(file)
}

pub fn remove(file: &str, task_name: &str) {
    let mut config = ConfigurationFile::new(file);

    config.tasks.retain(|t| {
        t[&String::from("name")] != task_name
    });

    config.save(file);
}

pub fn get(file: &str, task_name: &str) -> HashMap<String, String> {
    let config = ConfigurationFile::new(file);
    let task =
        config.tasks.iter()
            .find(move |t| t[&String::from("name")] == task_name);

    task.unwrap().clone()
}

pub fn list(file: &str) -> Vec<HashMap<String, String>> {
    let config = ConfigurationFile::new(file);
    config.tasks
}