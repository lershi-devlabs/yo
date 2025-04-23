use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "yo", about = "ask your terminal anything", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    Ask {
        #[arg(required = true)]
        question: Vec<String>,
    },
    Setup,
    Config,
    Switch {
        // TODO: add more options
        /// either "openai" or "ollama"
        model: String,
    },
    Gpt {
        /// specific GPT model to use, e.g. "gpt-3.5-turbo" or "gpt-4"
        model: String,
    },
    List,
    #[command(external_subcommand)]
    Other(Vec<String>),
}

