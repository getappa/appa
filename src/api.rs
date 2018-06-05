use rocket;
use super::commands::{
    task::api as task,
    processor::api as processor,
    link::api as link,
    execution::api as execution,
    prop::api as prop

};

pub struct Global {
    pub file: String
}

pub fn init_server(file: String) {
    let router = routes![
        task::create,
        task::update,
        task::remove,
        task::list,
        task::get,
        processor::create,
        processor::update,
        processor::remove,
        processor::list,
        processor::get,
        link::exec,
        execution::run,
        execution::set_storage,
        prop::attach
    ];

    rocket::ignite()
        .manage(Global{ file })
        .mount("/", router).launch();
}
