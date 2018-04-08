use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Error};
use std::collections::HashMap;
use std::vec::Vec;
use rayon::prelude::*;

use super::processor::Entry;
use super::Task;

pub fn exec<F1, F2>(mut c: Command, success: F1, fail: F2)
where F1: Fn(String), F2: Fn(Error) {
    let mut child = c
        .stdout(Stdio::piped())
        .spawn()
        .expect("Unable to spawn program");

    loop{
        match child.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) => {
                if let Some(ref mut stdout) = child.stdout {
                    let lines = BufReader::new(stdout).lines();
                    for line in lines {
                        match line {
                            Ok(x) => success(x),
                            Err(e) => fail(e)
                        };
                    }
                }
            }
            Err(e) => println!("error attempting to wait: {}", e),
        }
    }
}

pub fn entries(entries: Vec<Entry>, d: &str) {
    entries.par_iter().for_each(|e| {
        let ef = format!("!AppaTag({})", e.tag);
        if d.contains(&ef) {
            let new_d = d.replace(&ef, "");
            e.pentity.process(&new_d);
        }
    });
}