use rocket::State;
use rocket_contrib::{Json, Value};

use super::super::super::api::Global;
use super::{
    Processor, new,
    update as _update,
    list as _list,
    get as _get,
    remove as rm
};

#[post("/processor", format = "application/json", data = "<json>")]
pub fn create(json: Json<Processor>, gs: State<Global>) -> Json<Value> {

    match json {
        Json(value) => new(&gs.file, &value)
    }

    Json(json!({
        "message": "done"
    }))
}

#[put("/processor", format = "application/json", data = "<json>")]
pub fn update(json: Json<Processor>, gs: State<Global>) -> Json<Value> {
    match json {
        Json(value) => _update(&gs.file, &value)
    }

    Json(json!({
        "message": "done"
    }))
}

#[get("/processor", format = "application/json")]
pub fn list(gs: State<Global>) -> Json<Value> {
    Json(json!(_list(&gs.file)))
}

#[get("/processor/<id>", format = "application/json")]
pub fn get(id: String, gs: State<Global>) -> Json<Value> {
    Json(json!(_get(&gs.file, &id)))
}

#[delete("/processor/<id>", format = "application/json")]
pub fn remove(id: String, gs: State<Global>) -> Json<Value> {
    rm(&gs.file, &id);

    Json(json!({
        "message": "done"
    }))
}

