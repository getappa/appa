use super::super::super::ConfigurationFile;
use super::Task;

pub fn new(file: &str, task: &Task) {
    let mut config = ConfigurationFile::new(file);
    config.tasks.push(task.to_hashmap());
    config.save(&file);
}
