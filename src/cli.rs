use std::{io, path::PathBuf};

use clap::{Parser, Subcommand};

use crate::{tidy::TidyProgram, todo::TodoProgram};

const ABOUT: &str = "Flow is a good TooKit for manage workflow of developers";

#[derive(Parser, Debug)]
#[command(name = "flow")]
#[command(author = "Dreck Sallow")]
#[command(version = "1.0")]
#[command(about=ABOUT,long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    ///Tidy is a sub-tool for manage your projects
    Tidy {
        #[command(subcommand)]
        command: TidyCommands,
    },
    ///Todo is a sub-tool for manage your todos
    Todo {
        #[command(subcommand)]
        command: TodoCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum TidyCommands {
    ///Add the current path to projects
    Add { path: PathBuf },
    ///List all projects saved
    List,
    ///Open a project with an editor
    Open,
    ///Create new folder project and save
    New,
    ///Remove a project
    Remove,
}

#[derive(Subcommand, Debug)]
pub enum TodoCommands {
    ///List all todos
    List,
    ///Create new todo and save
    Create,
    ///Change the todo status
    Check,
}

pub struct App;

impl App {
    pub fn run() -> io::Result<()> {
        let cli = Cli::parse();
        match &cli.command {
            Commands::Tidy { command } => TidyProgram::run(command),
            Commands::Todo { command } => TodoProgram::run(command),
        }
    }
}
