use std::convert::From;
use std::error::Error;
use std::fs;
use std::{fmt::Display, str::FromStr};
use clap::{Parser, Subcommand};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Task::parse();

    let todos = fs::read_to_string("todos.txt")?;

    let mut t = todos.trim().parse::<Tasks>()?;

    // NOTE: MATCH, destructuring
    match args.command {
        Command::Delete { id } => t.0.retain(|task| task.id != id),
        // NOTE: debug formatter
        Command::Add { text } => t.0.push(Todo {
            id: t.0.iter().map(|t| t.id).max().unwrap_or(0) + 1,
            task: text,
        }),
        Command::Display => println!("{}", t),
    }

    fs::write("todos.txt", t.to_string())?;
    Ok(())
}

// NOTE: CLI parsing
#[derive(Parser)]
struct Task {
    #[command(subcommand)]
    command: Command,
}

// NOTE: CLI parsing
#[derive(Subcommand)]
enum Command {
    Delete { id: usize },
    // NOTE: use of option
    Add { text: String },
    Display,
}

// NOTE: use of Vec<>
#[derive(Debug)]
struct Tasks(Vec<Todo>);

// NOTE: common debugs
#[derive(Parser, Clone, Debug)]
struct Todo {
    task: String,
    id: usize,
}


////////////
// Part 2 //
////////////

// NOTE: use of trait, fromstr for parsing
impl FromStr for Tasks {
    type Err = &'static str;

    // TODO: change to partition
    // https://doc.rust-lang.org/rust-by-example/error/iter_result.html
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // NOTE: iterators
        if s.trim().is_empty() {
            return Ok(Tasks(vec![]));
        }
        let (tasks, errors): (Vec<_>, Vec<_>) = s
            .split('\n')
            .into_iter()
            .map(str::parse::<Todo>)
            .partition(Result::is_ok);

        if !errors.is_empty() {
            for err in errors {
                println!("{}", err.unwrap_err());
            }
        }

        if tasks.is_empty() {
            Ok(Tasks(vec![]))
        } else {
            Ok(Tasks(tasks.into_iter().map(Result::unwrap).collect()))
        }
    }
}

impl Display for Tasks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for task in &self.0 {
            f.write_str(&format!("{task}\n"))?
        }
        Ok(())
    }
}

impl FromStr for Todo {
    type Err = &'static str;

    // format: id:task
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        // NOTE: question mark error handling
        let id = split.next().ok_or("no id provided")?.parse::<usize>();

        // NOTE: if let, not explicitly returning
        if let (Some(task), Ok(id)) = (split.next(), id) {
            Ok(Todo {
                task: task.trim().to_owned(),
                id,
            })
        } else {
            Err("got id but no task")
        }
    }
}

// NOTE: display
impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}: {}", self.id, self.task))
    }
}
