use rocksdb::{Error, DB};
use serde_json::{Value, to_string, from_str};
use std::result::Result;
use std::collections::HashMap;
// use std::fs::remove_dir_all;

pub struct RocksDbStorage {
    pub base: String,
}

pub struct RocksDbProject {
    pub conn: DB
}

impl RocksDbStorage {
    pub fn new(base: String) -> RocksDbStorage {
        RocksDbStorage{ base: base }
    }

    pub fn project(&self, project: String) -> RocksDbProject {
        RocksDbProject::new(&self.base, project)
    }
}

impl RocksDbProject {
    pub fn new(base: &str, project: String) -> RocksDbProject {
        let path = format!("{}/{}", base, project);
        let conn = DB::open_default(&path).unwrap();
        RocksDbProject { conn: conn }
    }

    pub fn scan(&self) -> HashMap<String, String> {
        let mut data = HashMap::new();
        let mut iter = self.conn.raw_iterator();

        iter.seek(b"%");

        while iter.valid() {
            let current = (iter.key(), iter.value());

            match current {
                (Some(key), Some(val)) => {
                    data.insert(
                        String::from_utf8(key).unwrap(),
                        String::from_utf8(val).unwrap()
                    );
                },
                (Some(_), None) => {
                    println!("@TODO err handling");
                },
                (None, Some(_)) => {
                    println!("@TODO err handling");
                },
                (None, None) => {
                    println!("@TODO err handling");
                }
            }

            iter.next();
        }

        data
    }

    pub fn bulk_insert_string(&self, data_string: &str) {
        let data = from_str::<Value>(data_string).ok().unwrap();

        for (k, v) in data.as_object().unwrap().iter() {
            self.put_string(
                k.as_bytes(),
                v.as_str().unwrap()
            );
        }
    }

    pub fn scan_bytes(&self) -> String {
        let data = json!(self.scan());
        String::from(to_string(&data).unwrap().as_str())
    }

    pub fn clean(&self) -> Result<() , Error> {
       self.conn.delete(b"%")
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