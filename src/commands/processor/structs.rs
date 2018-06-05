
use structopt::StructOpt;
use std::collections::HashMap;
use super::super::link::LinkFlags;
use super::super::super::ProcessEntity;

#[derive(Serialize, Deserialize, Debug, StructOpt)]
pub struct Processor {
    #[structopt(name = "NAME")]
    /// New processor name
    pub name: String,

    #[structopt(short = "i", long = "id-prop", default_value = "")]
    /// Insert an prop that will be used as id
    pub id_prop: String,

    #[structopt(flatten)]
    pub flags: LinkFlags
}

#[derive(Serialize, Deserialize, Debug, StructOpt)]
pub struct ProcessorSimple {
    #[structopt(name = "NAME")]
    /// Processor name
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, StructOpt)]
pub enum ProcessorCommands {
    #[structopt(name = "create")]
    Create(Processor),

    #[structopt(name = "update")]
    Update(Processor),

    #[structopt(name = "rm")]
    Remove(ProcessorSimple),

    #[structopt(name = "get")]
    Get(ProcessorSimple),

    #[structopt(name = "list")]
    List
}

impl Processor {
    pub fn to_entity(&self) -> ProcessEntity {
        let flags = self.flags.clone();
        let collectors =
            flags.collectors.into_iter().collect::<HashMap<_, _>>();
        let sync_tasks =
            flags.sync.into_iter().collect::<HashMap<_, _>>();
        let async_tasks =
            flags.async.into_iter().collect::<HashMap<_, _>>();

        ProcessEntity{
            name: self.name.clone(),
            id_prop: self.id_prop.clone(),
            collector_tasks: collectors,
            sync_tasks: sync_tasks,
            async_tasks: async_tasks
        }
    }
}