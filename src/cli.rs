use structopt::StructOpt;
use super::commands;

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(name = "FILE")]
    file: String,

    #[structopt(subcommand)]
    cmd: Option<CliSubcommands>
}

#[derive(StructOpt, Debug)]
pub enum CliSubcommands {
    #[structopt(name = "run")]
    Run {},

    // #[structopt(name = "link")]
    // Link {},

    #[structopt(name = "task")]
    Task {
        #[structopt(name = "NAME")]
        name: String,

        #[structopt(name = "COMMAND")]
        command: String,

        #[structopt(name = "PATH")]
        path: String,
    },

    // Processor {

    // },

    #[structopt(name = "prop")]
    Prop {
        #[structopt(name = "ENTITY")]
        entity: String,

        #[structopt(name = "KEY")]
        key: String,

        #[structopt(name = "PROP")]
        prop: String,

        #[structopt(name = "VALUE")]
        value: String
    },
}

pub fn cli() {
    let opts = Cli::from_args();

    match opts.cmd {
        Some(value) => match value {
            CliSubcommands::Run{} => commands::run(opts.file),
            CliSubcommands::Prop{ entity, key, prop, value } =>
                commands::prop(opts.file, entity, key, prop, value),
            CliSubcommands::Task{ name, command, path } =>
                commands::new_task(opts.file, name, command, path)
        },
        None => {
            commands::run(opts.file);
        }
    }
}

