mod entity;
mod tasks;

use yaml_rust::{Yaml, YamlLoader};
use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;
use std::collections::HashMap;

pub use self::entity::AppaEntity;
pub use self::tasks::AppaTask;

pub struct AppaConfig {
    pub entities: Vec<AppaEntity>,
    pub tasks: HashMap<String, AppaTask>,
}

fn generate_entities(yaml: &Yaml) -> Vec<AppaEntity> {
    let mut arr:Vec<AppaEntity> = Vec::new();
    for item in yaml.as_vec() {
        arr.push(AppaEntity::new(&item[0]));
    }
    arr
}

fn generate_tasks(yaml: &Yaml) -> HashMap<String, AppaTask> {
    let mut m:HashMap<String, AppaTask> = HashMap::new();
    for item in yaml.as_vec() {
        m[&item[0]["name"]] = AppaTask::new(&item[0]);
    }
    m
}

impl AppaConfig {
    pub fn new(file_name: String) -> AppaConfig {
        let mut file = File::open(&file_name).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let docs = YamlLoader::load_from_str(&data).unwrap();
        let doc = &docs[0];

        AppaConfig{
            entities: generate_entities(&doc["entities"]),
            tasks: generate_tasks(&doc["tasks"])
        }
    }
}