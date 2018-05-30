use super::commands;
use structopt::StructOpt;
use std::vec::Vec;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug)]
pub struct KeyValueParseError;

impl ToString for KeyValueParseError {
    fn to_string(&self) -> String {
        "Invalid key:value pair".into()
    }
}

fn parse_key_value<K, V>(s: &str) -> Result<(K, V), KeyValueParseError>
where
    K: FromStr,
    V: FromStr,
{
    let mut parts = s.split(':');
    let key = parts.next().map(K::from_str);
    let value = parts.next().map(V::from_str);

    match (key, value) {
        (Some(Ok(k)), Some(Ok(v))) => Ok((k, v)),
        _ => Err(KeyValueParseError),
    }
}

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(name = "FILE")]
    /// Configuration file
    file: String,

    #[structopt(subcommand)]
    cmd: Option<CliSubcommands>
}

#[derive(StructOpt, Debug)]
pub struct LinkFlags {
    #[structopt(short = "c", long = "collector", parse(try_from_str = "parse_key_value"))]
    /// Link a collector task
    collectors: Vec<(String, String)>,

    #[structopt(short = "s", long = "sync", parse(try_from_str = "parse_key_value"))]
    /// Link a sync task
    sync: Vec<(String, String)>,

    #[structopt(short = "a", long = "async", parse(try_from_str = "parse_key_value"))]
    /// Link an async task
    async: Vec<(String, String)>
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
    Link {
        #[structopt(name = "NAME")]
        /// Processor's name for link tasks
        name: String,

        #[structopt(flatten)]
        flags: LinkFlags
    },

    #[structopt(name = "task")]
    /// Create a new task
    Task {
        #[structopt(name = "NAME")]
        // Task name
        name: String,

        #[structopt(name = "COMMAND")]
        // Task command
        command: String,

        #[structopt(name = "PATH")]
        // Task script path
        path: String,
    },

    #[structopt(name = "processor")]
    /// Create a new processor
    Processor {
        #[structopt(name = "NAME")]
        /// New processor name
        name: String,

        #[structopt(short = "i", long = "id-prop", default_value = "")]
        /// Insert an prop that will be used as id
        id_prop: String,

        #[structopt(flatten)]
        link_flags: LinkFlags
    },

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

            CliSubcommands::Task{ name, command, path } =>
                commands::new_task(opts.file, name, command, path),

            CliSubcommands::Processor{ name, id_prop, link_flags } =>
                commands::new_processor(
                    opts.file, name, id_prop,
                    link_flags.collectors.into_iter().collect::<HashMap<_, _>>(),
                    link_flags.sync.into_iter().collect::<HashMap<_, _>>(),
                    link_flags.async.into_iter().collect::<HashMap<_, _>>()
                ),

            CliSubcommands::Link{ name, flags } =>
                commands::link(
                    opts.file, name,
                    flags.collectors.into_iter().collect::<HashMap<_, _>>(),
                    flags.sync.into_iter().collect::<HashMap<_, _>>(),
                    flags.async.into_iter().collect::<HashMap<_, _>>()
                )
        },
        None => {
            commands::run(opts.file);
        }
    }
}

