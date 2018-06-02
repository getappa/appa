use rocket::State;
use rocket_contrib::{Json, Value};

use super::super::super::api::Global;
use super::{Prop, attach as _attach};

#[post("/prop", format = "application/json", data = "<json>")]
pub fn attach(json: Json<Prop>, gs: State<Global>) -> Json<Value> {

    match json {
        Json(value) => _attach(&gs.file, &value)
    }

    Json(json!({
        "message": "done"
    }))
}
