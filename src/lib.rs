#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate structopt;

#[macro_use]
extern crate serde_json;

extern crate rocket_contrib;
extern crate rocket;
extern crate itertools;
extern crate serde_yaml;
extern crate rayon;
extern crate rocksdb;
extern crate uuid;

mod cli;

pub mod commands;
pub mod config;
pub mod processor;
pub mod task;
pub mod consumer;
pub mod hub;
pub mod storage;
pub mod api;

pub use self::cli::{cli, Cli};
pub use self::storage::{RocksDbStorage, RocksDbProject};
pub use self::processor::ProcessEntity;
pub use self::task::Task;
pub use self::hub::{Hub, Collector};
pub use self::config::ConfigurationFile;