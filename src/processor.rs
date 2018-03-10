use std::process;

pub struct Command {
    pub path: String,
    pub command: String,
}

pub trait Runner {
    fn run(&self, pe: Command) -> String {}
}

pub fn consume_task(pe: Command) -> String {
    let output = process::Command::new("sh")
                        .arg("-c")
                        .arg(pe.get_command())
                        .output()
                        .expect("failed");

    format!("{}", String::from_utf8_lossy(&output.stdout))
}

impl Command {
    fn get_command(&self) -> String {
        format!(
            "{} {} {}",
            &self.command,
            &self.path,
            &self.data
        )
    }
}