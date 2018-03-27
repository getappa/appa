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

    pub fn create(&self, json: Value) -> [u8; 16] {
        let uid = Uuid::new_v4();
        &self.put(*uid.as_bytes(), json);

        uid.as_bytes().clone()
    }

    pub fn put(&self, uid: [u8; 16], json: Value) {
        &self.conn.put(
            &uid,
            to_string(&json).unwrap().as_str().as_bytes()
        );
    }
    pub fn get(&self, key: [u8; 16]) -> Value {
        let resp_oks = self.conn.get(&key).ok().unwrap().unwrap();
        let resp_bts = resp_oks.to_utf8().unwrap();
        Value::from(resp_bts)
    }

    pub fn update_json(&self, key: [u8; 16], prop: String, value: String) {
        let json_map = self.get(key);
        let json_ref = json_map.as_object().unwrap();
        let mut json = json_ref.clone();

        json.insert(prop, Value::from(value));

        let new_value = Value::from(json);
        &self.put(key, new_value);
    }
}