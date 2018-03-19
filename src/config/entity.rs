use yaml_rust::Yaml;
use std::collections::HashMap;

pub struct AppaEntity {
    pub name: String,
    pub collector: String,
    pub tasks: HashMap<String, String>
}

fn convert_str(y: &Yaml) -> String {
    y.as_str().unwrap().to_string()
}

fn convert_to_hashmap(y: &Yaml) -> HashMap<String, String> {
    let mut map:HashMap<String, String> = HashMap::new();
    let v = y.as_hash().unwrap();

    for (key, value) in v {
        map.insert(
            key.as_str().unwrap().to_string(),
            value.as_str().unwrap().to_string()
        );
    }

    map
}

impl AppaEntity {
    pub fn new(vals: &Yaml) -> AppaEntity {
        AppaEntity{
            name: convert_str(&vals["name"]),
            collector: convert_str(&vals["collector"]),
            tasks: convert_to_hashmap(&vals["tasks"])
        }
    }
}
