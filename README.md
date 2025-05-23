# yo

Ask your terminal anything using AI (OpenAI or Ollama).

## Features
- Ask questions directly from your terminal using natural language
- Supports both OpenAI and Ollama as AI backends
- Simple CLI: use `yo <question>` or `yo ask <question>`
- Easily configurable

## Installation

### Precompiled Binaries
Precompiled binaries for macOS, Linux, and Windows are available on the [Releases page](https://github.com/montekkundan/yo/releases). Download the appropriate binary for your platform, extract it, and move it to a directory in your `PATH`.

### Homebrew (macOS/Linux)
You can install `yo` using Homebrew:

```sh
brew tap lershi-devlabs/yo https://github.com/lershi-devlabs/yo.git
brew install lershi-devlabs/yo/yo-bin
```

### Cargo (Rust)
If you have Rust installed, you can install with Cargo:

```sh
cargo install yo
```

### Build from Source
Clone the repository and build with Cargo:

```sh
git clone https://github.com/lershi-devlabs/yo.git
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

## How to Push Changes to GitHub

To push your changes to GitHub, use the following steps:

```sh
git pull
git add .
git commit -m "chore: bump version to x.y.z"
git tag 1.0.0   # necessary when new version is released
# replace 1.0.0 with the version you are releasing
git push origin master --tags
```

## Changelog

See [CHANGELOG.md](./CHANGELOG.md) for a list of changes and version history.

## License

MIT
