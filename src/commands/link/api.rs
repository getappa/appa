use rocket::State;
use rocket_contrib::{Json, Value};

use super::super::super::api::Global;
use super::{Link, exec as _exec};

#[post("/link", format = "application/json", data = "<json>")]
pub fn exec(json: Json<Link>, gs: State<Global>) -> Json<Value> {

    match json {
        Json(value) => _exec(&gs.file, &value)
    }

    Json(json!({
        "message": "done"
    }))
}