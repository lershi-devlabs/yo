mod cli;
mod config;
mod commands;

use clap::Parser;
use cli::{Cli, Command};

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Command::Setup)            => commands::setup(),
        Some(Command::Config)           => commands::show_config_path(),
        Some(Command::Switch { model }) => commands::switch(&model).await,
        Some(Command::Gpt { model })    => commands::set_gpt(&model).await,
        Some(Command::List)             => commands::list_models().await,
        Some(Command::Ask { question }) => commands::ask(&question).await,
        Some(Command::Current)          => commands::show_current(),
        Some(Command::Other(o))         => commands::ask(&o).await,
        Some(Command::ClearHistory)     => commands::clear_history(),
        None                            => println!("yo what?"),
    }
}

