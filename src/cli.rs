use std::{io, path::PathBuf};

use clap::{Parser, Subcommand};

use crate::tidy::TidyProgram;

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
    ///Tidy is a sub-tool for manage the proyects
    Tidy {
        #[command(subcommand)]
        command: TidyCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum TidyCommands {
    ///Open a proyect with an editor
    Open,
    ///Add the current path to proyects
    Add { path: PathBuf },
    ///Create new folder proyect and save
    New,
    ///Remove a proyect
    Remove,
}

pub struct App;

impl App {
    pub fn run() -> io::Result<()> {
        let cli = Cli::parse();
        match &cli.command {
            Commands::Tidy { command } => TidyProgram::run(command),
        }
    }
}
