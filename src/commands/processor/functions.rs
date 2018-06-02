use super::super::super::ConfigurationFile;
use super::Processor;

pub fn new(file: &str, processor: &Processor) {
    let mut config = ConfigurationFile::new(&file);

    config.processors.push(processor.to_entity());
    config.save(&file);
}
