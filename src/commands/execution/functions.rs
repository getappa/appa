use super::super::super::{ Hub, ConfigurationFile };
use super::{ Run, SetUri };

pub fn run(file: &str, _opts: &Run) {
    let config = ConfigurationFile::new(file);
    let hub = Hub::new(config);
    hub.start();
}

pub fn set_storage(file: &str, opts: &SetUri) {
    let mut config = ConfigurationFile::new(file);

    config.storage_uri = opts.path.clone();
    config.save(file);
}