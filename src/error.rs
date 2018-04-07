use std::io::Error;

pub fn error_handler(intro: &str, e: Error) {
    println!("{} -> {:?}", intro, e);
}