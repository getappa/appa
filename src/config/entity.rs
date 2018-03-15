use yaml_rust::Yaml;
use std::vec::Vec;

pub struct AppaEntity {
    pub name: String,
    pub collector: String,
    pub tasks: Vec<String>,
}

fn convert_str(y: &Yaml) -> String {
    y.as_str().unwrap().to_string()
}

impl AppaEntity {
    pub fn new(vals: &Yaml) -> AppaEntity {
        AppaEntity{
            name: convert_str(&vals["name"]),
            collector: convert_str(&vals["collector"]),
            tasks: vals["tasks"].as_vec().unwrap().iter().map(convert_str).collect()
        }
    }
}