use super::super::{ ConfigurationFile, RocksDbStorage };

pub fn prop(file: String, entity: String, key: String, prop: String, value: String) {
    let config = ConfigurationFile::new(&file);
    let storage = RocksDbStorage::new(config.storage_uri);
    let db = storage.project(entity);

    db.update_json(key.as_bytes(), &prop, &value);
}
