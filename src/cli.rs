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
    file: String,

    #[structopt(subcommand)]
    cmd: Option<CliSubcommands>
}

#[derive(StructOpt, Debug)]
pub enum CliSubcommands {
    #[structopt(name = "run")]
    Run {},

    #[structopt(name = "link")]
    Link {
        #[structopt(name = "NAME")]
        name: String,

        #[structopt(short = "c", long = "collector", parse(try_from_str = "parse_key_value"))]
        collectors: Vec<(String, String)>,

        #[structopt(short = "s", long = "sync", parse(try_from_str = "parse_key_value"))]
        sync: Vec<(String, String)>,

        #[structopt(short = "a", long = "async", parse(try_from_str = "parse_key_value"))]
        async: Vec<(String, String)>
    },

    #[structopt(name = "task")]
    Task {
        #[structopt(name = "NAME")]
        name: String,

        #[structopt(name = "COMMAND")]
        command: String,

        #[structopt(name = "PATH")]
        path: String,
    },

    #[structopt(name = "processor")]
    Processor {
        #[structopt(name = "NAME")]
        name: String,

        #[structopt(short = "i", long = "id-prop", default_value = "")]
        id_prop: String,

        #[structopt(short = "c", long = "collector", parse(try_from_str = "parse_key_value"))]
        collectors: Vec<(String, String)>,

        #[structopt(short = "s", long = "sync", parse(try_from_str = "parse_key_value"))]
        sync: Vec<(String, String)>,

        #[structopt(short = "a", long = "async", parse(try_from_str = "parse_key_value"))]
        async: Vec<(String, String)>
    },

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
                commands::new_task(opts.file, name, command, path),
            CliSubcommands::Processor{ name, id_prop, collectors, sync, async } =>
                commands::new_processor(
                    opts.file, name, id_prop,
                    collectors.into_iter().collect::<HashMap<_, _>>(),
                    sync.into_iter().collect::<HashMap<_, _>>(),
                    async.into_iter().collect::<HashMap<_, _>>()
                ),
            CliSubcommands::Link{ name, collectors, sync, async } =>
                commands::link(
                    opts.file, name,
                    collectors.into_iter().collect::<HashMap<_, _>>(),
                    sync.into_iter().collect::<HashMap<_, _>>(),
                    async.into_iter().collect::<HashMap<_, _>>()
                )
        },
        None => {
            commands::run(opts.file);
        }
    }
}

