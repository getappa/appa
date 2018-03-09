use std::process;

pub struct Command {
    pub path: String,
    pub command: String,
}

pub fn run(pe: Command, data: String) -> String {
    let output = process::Command::new("sh")
                        .arg("-c")
                        .arg(pe.get_command(data))
                        .output()
                        .expect("failed");

    format!("{}", String::from_utf8_lossy(&output.stdout))
}

impl Command {
    fn get_command(&self, data: String) -> String {
        format!(
            "{} {} {}",
            &self.command,
            &self.path,
            data
        )
    }
}