use rocket::State;
use rocket_contrib::{Json, Value};
use super::common::Global;
use super::super::commands;

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub name: String,
    pub command: String,
    pub path: String,
}

#[post("/task", format = "application/json", data = "<task>")]
pub fn create(task: Json<Task>, gs: State<Global>) -> Json<Value> {
    let t = task.clone();
    let file = gs.file.clone();

    commands::new_task(file, t.name, t.command, t.path);

    Json(json!({
        "message": "done"
    }))
}