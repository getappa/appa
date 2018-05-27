use structopt::StructOpt;
use structopt::clap::AppSettings;
use super::commands;

#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "AppSettings::InferSubcommands"))]
pub enum Cli {
    #[structopt(name = "run")]
    Run {
        #[structopt(name = "FILE")]
        file: String
    },

    #[structopt(name = "link")]
    Link {},

    #[structopt(name = "new")]
    New {},

    #[structopt(name = "prop")]
    Prop {
        #[structopt(name = "FILE")]
        file: String,

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

    match opts {
        Cli::Run{ file } => commands::run(file),
        Cli::Prop{
            file, entity, key, prop, value
        } => commands::prop(file, entity, key, prop, value),
        Cli::Link{ .. } => commands::link(opts),
        Cli::New{ .. } => commands::new(opts)
    }
}

