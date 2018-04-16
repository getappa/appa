use std::process::Command;

pub struct Task {
    command: String,
    path: String
}

impl Task {
    pub fn new(c: &str, p: &str) -> Task {
        Task { command: c.to_string(), path: p.to_string() }
    }

    pub fn to_string(&self) -> String {
        format!("{} {}", self.command, self.path)
    }

    pub fn get_cmd(&self, d: &str) -> Command {
        println!("Will Call > {} {}", self.to_string(), format!("'{}'", d));
        let mut cmd = Command::new(&self.command);
        cmd.arg(&self.path).arg(format!("'{}'", d));
        cmd
    }
}