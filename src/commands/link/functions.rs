use std::collections::HashMap;
use super::super::super::ConfigurationFile;
use super::Link;

pub fn exec(file: &str, link_info: &Link) {
    let mut config = ConfigurationFile::new(&file);
    let flags = link_info.flags.clone();

    let collectors =
        flags.collectors.into_iter().collect::<HashMap<_, _>>();
    let sync_tasks =
        flags.sync.into_iter().collect::<HashMap<_, _>>();
    let async_tasks =
        flags.async.into_iter().collect::<HashMap<_, _>>();

    for v in config.processors.iter_mut() {
        if v.name == link_info.name {
            v.collector_tasks.extend(collectors);
            v.sync_tasks.extend(sync_tasks);
            v.async_tasks.extend(async_tasks);
            v.pos_tasks.extend(flags.pos);
            break;
        }
    }

    config.save(&file)
}
