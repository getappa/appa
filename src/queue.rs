use rayon::prelude::*;
use serde_json::Value;
use self::config::AppaConfig;

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
                let data = tasks[e.collector]();
                let json:Vec<Value> = serde_json::from_str(data);
                let map_json:Vec<Value> = Vec::new();

                json.par_iter().for_each(|d| {
                    let output_d = clone(d);
                    e.tasks.par_iter().for_each(|k, v| {
                        output_d[v] = tasks[k](d)
                        map_json.push(output_d)
                    });
                });

                // e.reducers.iter().for_each(|k, v| {
                //     data[k] = tasks[v](data);
                //     save_on_puma(data);
                // });

            })
    }
}