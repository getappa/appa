use rocksdb::{Options, DB};
use serde_json::{Value, to_string, from_str};
use std::result::Result;

pub struct RocksDbStorage {
    pub base: String,
}

pub struct RocksDbProject {
    path: String,
    pub conn: DB
}

impl RocksDbStorage {
    pub fn new(base: String) -> RocksDbStorage {
        RocksDbStorage{ base: base }
    }

    pub fn project(&self, project: String) -> RocksDbProject {
        let p = format!("{}/{}", &self.base, project);
        RocksDbProject::new(p)
    }
}

impl RocksDbProject {
    pub fn new(path: String) -> RocksDbProject {
        let conn = DB::open_default(&path).unwrap();
        RocksDbProject { conn: conn, path: path }
    }

    pub fn destroy(&self) -> Result<String, String> {
        match DB::destroy(&Options::default(), &self.path) {
            Ok() => Ok(self.path),
            Err() => Err("Not today")
        }
    }

    pub fn put(&self, uid: &[u8], data: &Value) {
        &self.conn.put(
            uid,
            to_string(data).unwrap().as_str().as_bytes()
        );
    }

    pub fn put_string(&self, uid: &[u8], json: &str) {
        &self.conn.put(
            uid,
            json.as_bytes()
        );
    }

    pub fn get_bytes(&self, key: &[u8]) -> String {
        let resp_oks = self.conn.get(&key).ok().unwrap().unwrap();
        String::from(resp_oks.to_utf8().unwrap())
    }

    pub fn get(&self, key: &[u8]) -> Value {
        let resp_oks = self.conn.get(&key).ok().unwrap().unwrap();
        let resp_bts = resp_oks.to_utf8().unwrap();
        from_str(resp_bts).ok().unwrap()
    }

    pub fn update_json(&self, key: &[u8], prop: &str, value: &str) {
        let mut json_map = self.get(key);
        let json_obj = json_map.as_object_mut();
        match json_obj {
            Some(json) => {
                json.insert(String::from(prop), Value::from(value));
                &self.put(&key, &Value::from(json.clone()));
            },
            None => {
                println!("Deu ruim");
            }
        }
    }
}