# rust-todo-cli

rust-todo is a Rust Command line To-Do list application 

## Download
The repository can be downloaded from the link below.

```bash
[https://github.com/vcxzzxcvn/rust-todo-cli](https://github.com/rajb16/rust-todo-cli).git
```

## Usage
This enumerator called Action has three fields or actions that it performs after the user specifies them in the command line.

* Add - to add the text input for the task by the user 
* Done - removes an existing task
* List - Lists all existing tasks
```rust
pub enum Action {
    Add {
        #[structopt()]
        text: String,
    },
    Done {
        #[structopt()]
        position: usize,
    },
    List,
}
```
A structure called Task that has a string value and the date & time of creation. When this structure is implemented a new Task is created.
```rust
pub struct Task {
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}
```
*add_task*: Opens the json file by the path given and adds the new task provided in the parameter

*complete_task*: Opens the file and removes a task by the given position(index) by the user.

*list_tasks*: Lists all tasks that are in the json file list.
```rust
pub fn add_task(joson_path: PathBuf, task: Task) 
pub fn complete_task(journal_path: PathBuf, task_position: usize)
pub fn list_tasks(journal_path: PathBuf)
```

## Sources
I made this application with the help of the guide below provided by Microsoft. Ive also included a YouTube link of the me explaining the project and a time-lapse of me writing the code.
```
Source :
* https://learn.microsoft.com/en-us/training/modules/rust-create-command-line-program/9-use-default-journal

YouTube Links:
* Time-lapse of me writing the code: https://youtu.be/xb1iGH9Q9WU
```

## License
MIT
