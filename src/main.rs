mod cli;
use structopt::StructOpt;
mod tasks;
use cli::{Action::*, CommandLineArgs};
use tasks::Task;

/// Main function
/// varaible CommandLineArgs this var takes in user input from the commandline. the user is asked to
///                          input the journal(json file) name and the action(add, done, and list).
/// variabele
fn main() {
    let CommandLineArgs { action, json_file } = CommandLineArgs::from_args();

    let json_file = json_file.expect("Failed to find json file");

    match action {
        Add { text } => tasks::add_task(json_file, Task::new(text)),
        List => tasks::list_tasks(json_file),
        Done { position } => tasks::complete_task(json_file, position),
    }
    .expect("Failed to perform action")
}
