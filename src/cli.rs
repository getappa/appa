use structopt::StructOpt;
use structopt::clap::AppSettings;
use super::commands;

#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "AppSettings::InferSubcommands"))]
pub enum Cli {
    #[structopt(name = "run")]
    Run {
        #[structopt(short = "d", long = "debug")]
        debug: bool,
    },

    #[structopt(name = "link")]
    Link {},

    #[structopt(name = "new")]
    New {},

    #[structopt(name = "prop")]
    Prop {},
}

pub fn cli() {
    let opts = Cli::from_args();

    match opts {
        Cli::Run{ .. } => commands::run(opts),
        Cli::Prop{ .. } => commands::prop(opts),
        Cli::Link{ .. } => commands::link(opts),
        Cli::New{ .. } => commands::new(opts)
    }
}

