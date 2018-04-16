extern crate appa;

use appa::{ Hub, ConfigurationFile };

pub fn main() {
    let config = ConfigurationFile::new("tests/mocks/config1.yml");
    let hub = Hub::new(config);

    hub.start();
}