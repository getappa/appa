use super::super::ConfigurationFile;

pub fn set_storage(file: String, path: String) {
    let mut config = ConfigurationFile::new(&file);

    config.storage_uri = path;
    config.save(&file);
}