use std::convert::From;
use std::error::Error;
use std::fs;
use std::{fmt::Display, str::FromStr};

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

#[derive(Parser)]
struct Task {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Delete { id: usize },
    // NOTE: use of option
    Add { text: String },
    Display,
}

#[derive(Debug)]
struct Tasks(Vec<Todo>);

#[derive(Parser, Clone, Debug)]
struct Todo {
    task: String,
    id: usize,
}

impl FromStr for Tasks {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (tasks, errors): (Vec<_>, Vec<_>) = s
            .trim()
            .split('\n')
            .map(str::parse::<Todo>)
            .partition(Result::is_ok);

        for err in errors {
            println!("{}", err.unwrap_err());
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
        f.write_str(
            &self
                .0
                .iter()
                .map(Todo::to_string)
                .collect::<Vec<_>>()
                .join("\n"),
        )?;
        Ok(())
    }
}

impl FromStr for Todo {
    type Err = &'static str;

    // format: id:task
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // NOTE: question mark error handling
        let (id, task) = s.split_once(':').ok_or("failed to split on :")?;

        if let Ok(id) = id.parse::<usize>() {
            Ok(Todo {
                task: task.trim().to_owned(),
                id,
            })
        } else {
            Err("failed to parse id")
        }
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}: {}", self.id, self.task))
    }
}
