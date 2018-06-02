use rocket::State;
use rocket_contrib::{Json, Value};

use super::super::super::api::Global;
use super::{
    Run, SetUri,
    run as _run,
    set_storage as ss
};

#[post("/run", format = "application/json")]
pub fn run(gs: State<Global>) -> Json<Value> {
    let opts = Run{};
    _run(&gs.file, &opts);

    Json(json!({
        "message": "done"
    }))
}

#[post("/set/storage", format = "application/json", data="<json>")]
pub fn set_storage(json: Json<SetUri>, gs: State<Global>) -> Json<Value> {
    match json {
        Json(value) => ss(&gs.file, &value)
    }

    Json(json!({
        "message": "done"
    }))
}
