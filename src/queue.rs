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
                e.tasks.par_iter().for_each(|k, v| {
                    data[k] = tasks[v](data);
                });

                e.reducers.iter().for_each(|k, v| {
                    data[k] = tasks[v](data);
                });

                //save on db
            })
    }
}