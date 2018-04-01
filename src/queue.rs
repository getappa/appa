use rayon::prelude::*;
use config::AppaConfig;
use serde_json::{Value, from_str, to_string, Result};
use storage::RocksDbStorage;
use uuid::Uuid;

enum IdRef {
    RefStr(String),
    RefUid(Uuid)
}

pub struct AppaQueue {
    config: AppaConfig
}

impl AppaQueue {
    pub fn new(ac: AppaConfig) -> AppaQueue {
        AppaQueue { config: ac }
    }

    pub fn exec(&self) {
        let storage = RocksDbStorage::new(self.config.storage_uri.clone());
        let tasks = &self.config.tasks;

        &self.config.entities.par_iter()
            .for_each(|e| {
                let db = storage.project(e.name.clone());

                let data = tasks.get(&e.collector).unwrap().exec("".to_string());
                let json:Result<Value> = from_str(&data);
                let done = json.ok().unwrap();
                let arr = done.as_array().unwrap();

                arr.par_iter().for_each(|d| {
                    let core_id = if e.id_prop == "" {
                        IdRef::RefUid(Uuid::new_v4())
                    } else {
                        IdRef::RefStr(
                            d[e.id_prop.clone()].to_string()
                        )
                    };

                    let key = match core_id {
                        IdRef::RefUid(ref x) => x.as_bytes(),
                        IdRef::RefStr(ref y) => y.as_bytes()
                    };

                    db.put(key.clone(), d.clone());
                    let str_data = to_string(d).unwrap().as_str().to_string();

                    e.pre.iter().for_each(|v| {
                        db.put_string(
                            key.clone(),
                            tasks.get(v).unwrap().exec(str_data.clone())
                        );
                    });

                    e.tasks.par_iter().for_each(|(k, v)| {
                        db.update_json(
                            key.clone(), v.clone(),
                            tasks.get(k).unwrap().exec(str_data.clone())
                        );
                    });
                });
            });
    }
}