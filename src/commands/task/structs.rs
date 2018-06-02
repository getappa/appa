use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, StructOpt)]
pub struct Task {
    #[structopt(name = "NAME")]
    // Task name
    name: String,

    #[structopt(name = "COMMAND")]
    // Task command
    command: String,

    #[structopt(name = "PATH")]
    // Task script path
    path: String,
}

impl Task {
    pub fn to_hashmap(&self) -> HashMap<String, String> {
        let mut task = HashMap::new();

        task.insert("name".to_string(), self.name.clone());
        task.insert("command".to_string(), self.command.clone());
        task.insert("path".to_string(), self.path.clone());

        task
    }
}
