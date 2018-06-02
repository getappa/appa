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
    Run(commands::execution::Run),

    #[structopt(name = "set_storage")]
    // Set storage uri link
    SetUri(commands::execution::SetUri),

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
    Prop(commands::prop::Prop)
}

pub fn cli() {
    let opts = Cli::from_args();

    match opts.cmd {
        Some(value) => match value {
            CliSubcommands::Run(run_opts) =>
                commands::execution::run(&opts.file, &run_opts),

            CliSubcommands::SetUri(uri_opts) =>
                commands::execution::set_storage(&opts.file, &uri_opts),

            CliSubcommands::Prop(prop_opts) =>
                commands::prop::attach(&opts.file, &prop_opts),

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

