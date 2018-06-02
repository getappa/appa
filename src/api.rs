use rocket;
use super::commands::{task};

pub struct Global {
    pub file: String
}

pub fn init_server(file: String) {
    let router = routes![
        task::api::create,
    ];

    rocket::ignite()
        .manage(Global{ file })
        .mount("/", router).launch();
}
