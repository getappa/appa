#[derive(Serialize, Deserialize, Debug, StructOpt)]
pub struct Run {}

#[derive(Serialize, Deserialize, Debug, StructOpt)]
pub struct SetUri {
    #[structopt(name = "PATH")]
    /// Path to rocksdb storage the data
    pub path: String,
}