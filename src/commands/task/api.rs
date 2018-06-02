use rocket::State;
use rocket_contrib::{Json, Value};

use super::super::super::api::Global;
use super::{Task, new};

#[post("/task", format = "application/json", data = "<json>")]
pub fn create(json: Json<Task>, gs: State<Global>) -> Json<Value> {

    match json {
        Json(value) => new(&gs.file, &value)
    }

    Json(json!({
        "message": "done"
    }))
}