# yo

Ask your terminal anything using AI (OpenAI or Ollama).

## Features
- Ask questions directly from your terminal using natural language
- Supports both OpenAI and Ollama as AI backends
- Simple CLI: use `yo <question>` or `yo ask <question>`
- Easily configurable

## Installation

Clone the repository and build with Cargo:

```sh
git clone https://github.com/montekkundan/yo.git
cd yo
cargo install --path .
```

Or build and run directly:

```sh
cargo run -- <your question>
```

## Usage

You can ask questions in two ways:

```sh
yo ask What is the capital of France?
# or simply
yo What is the capital of France?
```

### Example

```sh
yo Summarize the Rust ownership model.
```

## Configuration

The tool supports configuration for different AI backends (OpenAI, Ollama). See the documentation or run `yo help` for more details.

## License

MIT
