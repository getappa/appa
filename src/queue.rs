use rayon::prelude::*;
use config::AppaConfig;
use serde_json::{Value, from_str, to_string, Result};
// use std::collections::HashMap;

pub struct AppaQueue {
    config: AppaConfig
}

impl AppaQueue {
    pub fn new(ac: AppaConfig) -> AppaQueue {
        AppaQueue { config: ac }
    }

    pub fn exec(&self) {
        let tasks = &self.config.tasks;
        &self.config.entities.par_iter()
            .for_each(|e| {
                let data = tasks.get(&e.collector).unwrap().exec("".to_string());
                let json:Result<Value> = from_str(&data);
                // let map_json:Vec<Value> = Vec::new();

                json.ok().unwrap().as_array().unwrap().par_iter().for_each(|d| {
                    let mut output_d = &d.as_object().unwrap();

                    e.tasks.par_iter().for_each(|(k, v)| {
                        let str_data = to_string(d).unwrap().as_str().to_string();
                        output_d.insert(v.to_string(), Value::String(tasks.get(k).unwrap().exec(str_data)));
                    });
                    // scoped_data.par_iter_mut().for_each(|(k, v)| {
                    //     output_d[v] = tasks[k](d);
                    //     map_json.push(output_d);
                    // });
                });

                // e.reducers.iter().for_each(|k, v| {
                //     data[k] = tasks[v](data);
                //     save_on_puma(data);
                // });

            });
    }
}