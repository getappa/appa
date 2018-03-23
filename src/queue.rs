use rayon::prelude::*;
use config::AppaConfig;
use serde_json::{Value, from_str, Result};
use storage::RocksDbStorage;

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
                arr.par_iter().for_each(|d| db.create(d.clone()))

                    // db.create(d);
                    // let mut output_d = &d.as_object().unwrap();

                    // e.tasks.par_iter().for_each(|(k, v)| {
                    //     let str_data = to_string(d).unwrap().as_str().to_string();
                    //     output_d.insert(v.to_string(), Value::String(tasks.get(k).unwrap().exec(str_data)));
                    // });
                    // scoped_data.par_iter_mut().for_each(|(k, v)| {
                    //     output_d[v] = tasks[k](d);
                    //     map_json.push(output_d);
                    // });
                // });

                // e.reducers.iter().for_each(|k, v| {
                //     data[k] = tasks[v](data);
                //     save_on_puma(data);
                // });

            });
    }
}