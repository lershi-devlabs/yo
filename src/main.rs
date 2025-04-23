use clap::Parser;

/// Read a question and send it to our llm model 
#[derive(Parser)]
struct Cli {
    /// User question
    question: String,
}

fn main() {
    let args = Cli::parse(); 

    println!("Asking: {:?}", args.question);
}
