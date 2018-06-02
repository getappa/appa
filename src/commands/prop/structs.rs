#[derive(Serialize, Deserialize, Debug, StructOpt)]
pub struct Prop {
    #[structopt(name = "ENTITY")]
    /// Processor Entity that you want to check
    pub entity: String,

    #[structopt(name = "KEY")]
    /// The key of the data that you want to insert a prop
    pub key: String,

    #[structopt(name = "PROP")]
    /// Property name
    pub prop: String,

    #[structopt(name = "VALUE")]
    /// Value that you want to insert
    pub value: String
}