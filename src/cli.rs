use super::{commands, api};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(name = "FILE")]
    /// Configuration file
    file: String,

    #[structopt(subcommand)]
    cmd: Option<CliSubcommands>
}

#[derive(StructOpt, Debug)]
pub enum CliSubcommands {
    #[structopt(name = "run")]
    /// Execute appa collector and processors
    Run {},

    #[structopt(name = "set_storage")]
    // Set storage uri link
    SetUri {
        #[structopt(name = "PATH")]
        /// Storage URI path
        path: String,
    },

    #[structopt(name = "link")]
    /// Link tasks to processors
    Link(commands::link::Link),

    #[structopt(name = "task")]
    /// Create a new task
    Task(commands::task::Task),

    #[structopt(name = "processor")]
    /// Create a new processor
    Processor(commands::processor::Processor),

    #[structopt(name = "prop")]
    /// Add a value to a prop inside database
    Prop {
        #[structopt(name = "ENTITY")]
        /// Processor Entity that you want to check
        entity: String,

        #[structopt(name = "KEY")]
        /// The key of the data that you want to insert a prop
        key: String,

        #[structopt(name = "PROP")]
        /// Property name
        prop: String,

        #[structopt(name = "VALUE")]
        /// Value that you want to insert
        value: String
    },
}

pub fn cli() {
    let opts = Cli::from_args();

    match opts.cmd {
        Some(value) => match value {
            CliSubcommands::Run{} => commands::run(opts.file),

            CliSubcommands::SetUri { path } =>
                commands::set_storage(opts.file, path),

            CliSubcommands::Prop{ entity, key, prop, value } =>
                commands::prop(opts.file, entity, key, prop, value),

            CliSubcommands::Task(task) =>
                commands::task::new(&opts.file, &task),

            CliSubcommands::Processor(processor) =>
                commands::processor::new(&opts.file, &processor),

            CliSubcommands::Link(link_info) =>
                commands::link::exec(&opts.file, &link_info)
        },
        None => {
            api::init_server(opts.file);
        }
    }
}

