// MENTION: cargo fmt
// Talk about result/option

use std::convert::From;
use std::error::Error;
use std::fs;
use std::{fmt::Display, str::FromStr};

// NOTE: import
use clap::{Parser, Subcommand};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Task::parse();
    let tasks = fs::read_to_string("todos.txt")?;
    let mut t = tasks.trim().parse::<Tasks>()?;
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

// NOTE: use of trait, fromstr for parsing
impl FromStr for Tasks {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // NOTE: iterators
        if s.trim().is_empty() {
            return Ok(Tasks(vec![]));
        }
        let x: Result<Vec<Todo>, &str> =
            s.split('\n').into_iter().map(str::parse::<Todo>).collect();
        match x {
            Ok(v) => Ok(Tasks(v)),
            // Ignore errors, could just be there were no tasks
            Err(e) => {
                println!("{e}");
                Ok(Tasks(vec![]))
            }
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

// NOTE: common debugs
#[derive(Parser, Clone, Debug)]
struct Todo {
    task: String,
    id: usize,
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
