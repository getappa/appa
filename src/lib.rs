#[macro_use]
extern crate serde_derive;

extern crate itertools;
extern crate serde_yaml;
extern crate serde_json;
extern crate rayon;

pub mod config;
pub mod processor;
pub mod task;
pub mod consumer;
// pub mod storage;

pub use self::task::Task;
pub use self::processor::ProcessorHub;
pub use self::config::ConfigurationFile;