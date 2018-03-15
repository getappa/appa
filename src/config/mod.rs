mod entity;

use yaml_rust::{Yaml, YamlLoader};
use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

pub use self::entity::AppaEntity;

pub struct AppaConfig {
    pub entities: Vec<AppaEntity>,
    pub tasks: String,
}

fn generate_entities(yaml: &Yaml) -> Vec<AppaEntity> {
    let mut arr:Vec<AppaEntity> = Vec::new();
    for item in yaml.as_vec() {
        arr.push(AppaEntity::new(&item[0]));
    }
    arr
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
            tasks: "2".to_string()
        }
    }
}