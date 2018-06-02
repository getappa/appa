use rocket;
use super::commands::{
    task,
    processor,
    link
};

pub struct Global {
    pub file: String
}

pub fn init_server(file: String) {
    let router = routes![
        task::api::create,
        processor::api::create,
        link::api::exec
    ];

    rocket::ignite()
        .manage(Global{ file })
        .mount("/", router).launch();
}
