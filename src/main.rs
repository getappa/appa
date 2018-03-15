extern crate appa;

use appa::config::AppaConfig;

fn main() {
    let set = AppaConfig::new("tests/mocks/config1.yml".to_string());
    for entitie in set.entities {
        println!("{}", entitie.name)
    }

    for (key, task) in set.tasks {
        println!("{} {}", key, task.command)
    }
}