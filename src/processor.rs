use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessEntity {
    pub name: String,
    pub id_prop: String,
    pub collector_tasks: HashMap<String, String>,
    pub sync_tasks: HashMap<String, String>,
    pub async_tasks: HashMap<String, String>,
    pub pos_tasks: Vec<String>
}