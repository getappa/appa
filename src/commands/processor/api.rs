use rocket::State;
use rocket_contrib::{Json, Value};

use super::super::super::api::Global;
use super::{Processor, new};

#[post("/processor", format = "application/json", data = "<json>")]
pub fn create(json: Json<Processor>, gs: State<Global>) -> Json<Value> {

    match json {
        Json(value) => new(&gs.file, &value)
    }

    Json(json!({
        "message": "done"
    }))
}