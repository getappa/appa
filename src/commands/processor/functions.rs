use super::super::super::{ConfigurationFile, ProcessEntity};
use super::Processor;

pub fn new(file: &str, processor: &Processor) {
    let mut config = ConfigurationFile::new(&file);

    config.processors.push(processor.to_entity());
    config.save(&file);
}

pub fn update(file: &str, processor: &Processor) {
    let mut config = ConfigurationFile::new(file);

    config.processors
        .iter_mut()
        .find(|ref mut p| p.name == processor.name)
        .map(|p| {
            let pe = processor.to_entity();

            if pe.id_prop != "" {
                p.id_prop = pe.id_prop;
            }

            if pe.collector_tasks.len() > 0 {
                p.collector_tasks.extend(pe.collector_tasks)
            }

            if pe.sync_tasks.len()  > 0 {
                p.sync_tasks.extend(pe.sync_tasks)
            }

            if pe.async_tasks.len() > 0 {
                p.async_tasks.extend(pe.async_tasks)
            }

            p
        });

    config.save(file)
}

pub fn remove(file: &str, processor_name: &str) {
    let mut config = ConfigurationFile::new(file);

    config.processors.retain(|p| p.name != processor_name);
    config.save(file);
}

pub fn get(file: &str, processor_name: &str) -> ProcessEntity {
    let config = ConfigurationFile::new(file);
    let processor = config.processors.into_iter()
        .find(|p| p.name == processor_name);

    processor.unwrap()
}

pub fn list(file: &str) -> Vec<ProcessEntity> {
    let config = ConfigurationFile::new(file);
    config.processors
}