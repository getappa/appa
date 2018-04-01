use yaml_rust::Yaml;
use std::collections::HashMap;
use std::vec::Vec;

#[derive(Clone)]
pub struct AppaEntity {
    pub name: String,
    pub collector: String,
    pub tasks: HashMap<String, String>,
    pub pre: Vec<String>,
    pub id_prop: String,
    pub is_stream: bool
}

fn convert_str(y: &Yaml) -> String {
    match y.as_str() {
        Some(x) => x.to_string(),
        None => "".to_string()
    }
}

fn convert_bool(y: &Yaml) -> bool {
    match y.as_bool() {
        Some(x) => x,
        None => false
    }
}

fn convert_to_vec(y: &Yaml) -> Vec<String> {
    let mut arr:Vec<String> = Vec::new();
    let empty:Vec<Yaml> = Vec::new();

    let v = match y.as_vec() {
        Some(val) => val,
        None => &empty
    };

    for val in v {
        arr.push(convert_str(val));
    };

    arr
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
            tasks: convert_to_hashmap(&vals["tasks"]),
            pre: convert_to_vec(&vals["pre"]),
            id_prop: convert_str(&vals["id_prop"]),
            is_stream: convert_bool(&vals["stream"]),
        }
    }
}
