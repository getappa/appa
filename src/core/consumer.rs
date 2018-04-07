use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Error};

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
