mod structs;
mod functions;

pub mod api;
pub use self::structs::{
    Processor, ProcessorSimple, ProcessorCommands
};
pub use self::functions::{
    new, update, remove, get, list
};

pub fn exec(file: &str, command: &ProcessorCommands) {
    match command {
        ProcessorCommands::Create(opts) => new(file, opts),
        ProcessorCommands::Update(opts) => update(file, opts),
        ProcessorCommands::Remove(opts) => remove(file, &opts.name),
        ProcessorCommands::Get(opts) => {
            let processor = get(file, &opts.name);
            println!("{:?}", processor);
        },
        ProcessorCommands::List => {
            let processor = list(file);
            println!("{:?}", processor);
        }
    }
}