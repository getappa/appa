use super::super::{ConfigurationFile, ProcessEntity};
use std::collections::HashMap;

pub fn new_processor(
    file: String,
    name: String,
    id_prop: String,
    collectors: HashMap<String, String>,
    sync_tasks: HashMap<String, String>,
    async_tasks: HashMap<String, String>
) {
    let mut config = ConfigurationFile::new(&file);

    config.processors.push(ProcessEntity{
        name: name,
        id_prop: id_prop,
        collector_tasks: collectors,
        sync_tasks: sync_tasks,
        async_tasks: async_tasks
    });

    config.save(&file);
}
