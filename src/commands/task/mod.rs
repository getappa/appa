mod structs;
mod functions;

pub mod api;
pub use self::structs::{
    TaskCommands, TaskSimple, TaskUpdate, Task
};

pub use self::functions::{
    new, update, list, get, remove
};

pub fn exec(file: &str, command: &TaskCommands) {
    match command {
        TaskCommands::Create(opts) => new(file, opts),
        TaskCommands::Update(opts) => update(file, opts),
        TaskCommands::Remove(opts) => remove(file, &opts.name),
        TaskCommands::Get(opts) => {
            let task = get(file, &opts.name);
            println!("{:?}", task);
        },
        TaskCommands::List => {
            let tasks = list(file);
            println!("{:?}", tasks);
        }
    }
}