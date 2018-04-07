// use rocksdb::DB;
// use serde_json::{Value, to_string, from_str};

// pub struct RocksDbStorage {
//     pub base: String,
// }

// pub struct RockDbProject {
//     pub conn: DB
// }

// impl RocksDbStorage {
//     pub fn new(base: String) -> RocksDbStorage {
//         RocksDbStorage{ base: base }
//     }

//     pub fn project(&self, project: String) -> RockDbProject {
//         let p = format!("{}/{}", &self.base, project);
//         RockDbProject::new(DB::open_default(p).unwrap())
//     }
// }

// impl RockDbProject {
//     pub fn new(conn: DB) -> RockDbProject {
//         RockDbProject{conn: conn}
//     }


//     pub fn put(&self, uid: &[u8], json: Value) {
//         &self.conn.put(
//             uid,
//             to_string(&json).unwrap().as_str().as_bytes()
//         );
//     }

//     pub fn put_string(&self, uid: &[u8], json: String) {
//         &self.conn.put(
//             uid,
//             json.as_bytes()
//         );
//     }

//     pub fn get(&self, key: &[u8]) -> Value {
//         let resp_oks = self.conn.get(&key).ok().unwrap().unwrap();
//         let resp_bts = resp_oks.to_utf8().unwrap();
//         from_str(resp_bts).ok().unwrap()
//     }

//     pub fn update_json(&self, key: &[u8], prop: String, value: String) {
//         let json_map = self.get(key);
//         let json_obj = json_map.as_object();
//         let json_ref = json_obj.unwrap();

//         let mut json = json_ref.clone();

//         json.insert(prop, Value::from(value));

//         let new_value = Value::from(json);
//         &self.put(&key, new_value);
//     }
// }