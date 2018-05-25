use super::super::{
    Hub,
    ConfigurationFile
};

pub fn run(file: String) {
    let config = ConfigurationFile::new(&file);
    let hub = Hub::new(config);
    hub.start();
}