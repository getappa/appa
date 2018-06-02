use super::super::super::{ ConfigurationFile, RocksDbStorage };
use super::Prop;

pub fn attach(file: &str, opts: &Prop) {
    let config = ConfigurationFile::new(file);
    let storage = RocksDbStorage::new(config.storage_uri);
    let db = storage.project(opts.entity.clone());

    db.update_json(opts.key.as_bytes(), &opts.prop, &opts.value);
}
