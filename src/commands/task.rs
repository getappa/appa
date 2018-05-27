use super::super::ConfigurationFile;
use std::collections::HashMap;

pub fn new_task(file: String, name: String, command: String, path: String) {
    let mut config = ConfigurationFile::new(&file);
    let mut task = HashMap::new();

    task.insert("name".to_string(), name);
    task.insert("command".to_string(), command);
    task.insert("path".to_string(), path);

    config.tasks.push(task);
    config.save(&file);
}
