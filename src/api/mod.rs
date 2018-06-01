mod task;
pub mod common;

use rocket;
use self::common::Global;

pub fn init_server(file: String) {

    let router = routes![
        task::create,
    ];

    rocket::ignite()
        .manage(Global{ file })
        .mount("/", router).launch();
}
