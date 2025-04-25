use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "yo", about = "ask your terminal anything", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about = "Ask your AI a question", 
        long_about = "Ask the currently configured AI model a question. The response will stream in real-time if using OpenAI.\n\nYou can use this command as either:\n  yo ask <question>\n  yo <question>\n\nBoth forms are equivalent.", 
        visible_alias = "a")]
    Ask {
        #[arg(required = true, help = "The question or prompt to send to the AI")]
        question: Vec<String>,
    },
    
    #[command(about = "Setup your AI configuration", long_about = "Interactive setup to configure your AI backend (Ollama or OpenAI) and select a default model.")]
    Setup,
    
    #[command(about = "Show config file path", long_about = "Print the path to the configuration file where your settings are stored.")]
    Config,
    
    #[command(about = "Switch AI backend", long_about = "Switch between Ollama and OpenAI backends, or select a different model for your current backend.")]
    Switch {
        /// either "openai" or "ollama"
        #[arg(help = "Backend to switch to: 'openai' or 'ollama'")]
        model: String,
    },
    
    #[command(about = "Set specific GPT model", long_about = "Directly set a specific OpenAI model without going through the selection menu.")]
    Gpt {
        /// specific GPT model to use, e.g. "gpt-3.5-turbo" or "gpt-4"
        #[arg(help = "OpenAI model name to use (e.g. 'gpt-4', 'gpt-3.5-turbo')")]
        model: String,
    },
    
    #[command(about = "List available AI models", long_about = "Show a list of all available models from both Ollama and OpenAI backends.")]
    List,
    
    #[command(about = "Show current AI model in use", long_about = "Display information about the currently selected AI backend and model.")]
    Current,
    
    #[command(about = "Clear the conversation history", long_about = "Clear the conversation history stored in history.txt.")]
    ClearHistory,
    
    #[command(external_subcommand)]
    Other(Vec<String>),
}

