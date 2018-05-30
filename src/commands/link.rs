use super::super::ConfigurationFile;
use std::collections::HashMap;

pub fn link(
    file: String,
    name: String,
    collectors: HashMap<String, String>,
    sync_tasks: HashMap<String, String>,
    async_tasks: HashMap<String, String>
) {
    let mut config = ConfigurationFile::new(&file);

    for v in config.processors.iter_mut() {
        if v.name == name {
            v.collector_tasks.extend(collectors);
            v.sync_tasks.extend(sync_tasks);
            v.async_tasks.extend(async_tasks);
            break;
        }
    }

    config.save(&file)
}
