use yaml_rust::Yaml;
use std::process::Command;

#[derive(Clone)]
pub struct AppaTask {
    pub command: String,
    pub path: String,
}

fn convert_str(y: &Yaml) -> String {
    y.as_str().unwrap().to_string()
}

impl AppaTask {
    pub fn new(vals: &Yaml) -> AppaTask {
        AppaTask{
            command: convert_str(&vals["command"]),
            path: convert_str(&vals["path"]),
        }
    }

    pub fn exec(&self, data: String) -> String {
        let output = Command::new("sh")
                        .arg("-c")
                        .arg(&self.get_command(data))
                        .output()
                        .expect("failed");

        format!("{}", String::from_utf8_lossy(&output.stdout))
    }

    fn get_command(&self, data: String) -> String {
        format!(
            "{} {} {}",
            &self.command,
            &self.path,
            data
        )
    }
}