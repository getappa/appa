use structopt::StructOpt;
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

#[derive(Serialize, Deserialize, StructOpt, Debug, Clone)]
pub struct LinkFlags {
    #[structopt(short = "c", long = "collector", parse(try_from_str = "parse_key_value"))]
    /// Link a collector task
    pub collectors: Vec<(String, String)>,

    #[structopt(short = "s", long = "sync", parse(try_from_str = "parse_key_value"))]
    /// Link a sync task
    pub sync: Vec<(String, String)>,

    #[structopt(short = "a", long = "async", parse(try_from_str = "parse_key_value"))]
    /// Link an async task
    pub async: Vec<(String, String)>,

    #[structopt(short = "p", long = "pos")]
    /// Link an pos tasks
    pub pos: Vec<String>
}

#[derive(Serialize, Deserialize, StructOpt, Debug)]
pub struct Link {
    #[structopt(name = "NAME")]
    /// Processor's name for link tasks
    pub name: String,

    #[structopt(flatten)]
    pub flags: LinkFlags
}