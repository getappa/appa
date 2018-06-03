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

#[derive(Serialize, Deserialize, Debug, StructOpt)]
pub struct TaskUpdate {
    #[structopt(name = "NAME")]
    // Task name
    pub name: String,

    #[structopt(short = "c", long= "command", default_value="")]
    // Command to be updated
    pub command: String,

    #[structopt(short = "p", long= "path", default_value="")]
    // Path to be updated
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, StructOpt)]
pub struct TaskSimple {
    #[structopt(name = "NAME")]
    // Task name
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, StructOpt)]
pub enum TaskCommands {
    #[structopt(name = "name")]
    Create(Task),

    #[structopt(name = "update")]
    Update(TaskUpdate),

    #[structopt(name = "rm")]
    Remove(TaskSimple),

    #[structopt(name = "get")]
    Get(TaskSimple),

    #[structopt(name = "list")]
    List
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
