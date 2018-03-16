use yaml_rust::Yaml;
use std::vec::Vec;
use std::collections::HashMap;

pub struct AppaEntity {
    pub name: String,
    pub collector: String,
    pub tasks: Vec<HashMap<String, String>>,
}

fn convert_to_hashmap(y: &Yaml) -> HashMap<String, String> {
    let mut map:HashMap<String, String> = HashMap::new();
    let v = y.as_vec().unwrap();

    for val in 0 .. v.len() {
        map.insert(
            val["prop"].as_str().unwrap().to_string(),
            val["task"].as_str().unwrap().to_string()
        )
    }

    map
}

impl AppaEntity {
    pub fn new(vals: &Yaml) -> AppaEntity {
        AppaEntity{
            name: convert_str(&vals["name"]),
            collector: convert_str(&vals["collector"]),
            tasks: vals["tasks"].as_vec().unwrap().iter().map(convert_to_hashmap).collect()
        }
    }
}