use rocksdb::DB;
use uuid::Uuid;
use serde_json::{Value, to_string};

pub struct RocksDbStorage {
    pub base: String,
}

pub struct RockDbProject {
    pub conn: DB
}

impl RocksDbStorage {
    pub fn new(base: String) -> RocksDbStorage {
        RocksDbStorage{ base: base }
    }

    pub fn project(&self, project: String) -> RockDbProject {
        let p = format!("{}/{}", &self.base, project);
        RockDbProject::new(DB::open_default(p).unwrap())
    }
}

impl RockDbProject {
    pub fn new(conn: DB) -> RockDbProject {
        RockDbProject{conn: conn}
    }

    pub fn create(&self, json: Value) {
        &self.conn.put(
            Uuid::new_v4().as_bytes(),
            to_string(&json).unwrap().as_str().as_bytes()
        );
    }
}