use std::{env, fs, path::PathBuf};
use std::process::Command as ProcessCommand;
use clap::Parser;
use dotenv::{dotenv, from_filename};
use yo::cli::{Cli, Command as CliCommand};
use yo::commands;
use serial_test::serial;

// Load environment variables for testing
fn load_test_env() {
    let _ = from_filename(".env.test");
    let _ = dotenv();
}

// Manage a temporary config directory for isolated tests
struct TestEnv {
    original: Option<String>,
    temp_dir: PathBuf,
}

impl TestEnv {
    fn new() -> Self {
        load_test_env();
        let original = env::var("XDG_CONFIG_HOME").ok();
        let temp_dir = env::temp_dir().join("yo_test_config");
        let yo_dir = temp_dir.join("yo");
        fs::create_dir_all(&yo_dir).unwrap();
        unsafe { env::set_var("XDG_CONFIG_HOME", &temp_dir); }

        // Write default config
        let config_path = yo_dir.join("config.toml");
        let api_key = env::var("OPENAI_API_KEY").unwrap_or_default();
        let content = format!(
            "source = \"openai\"\nmodel = \"gpt-3.5-turbo\"\nopenai_api_key = \"{}\"",
            api_key
        );
        fs::write(&config_path, content).unwrap();
        TestEnv { original, temp_dir }
    }
}

impl Drop for TestEnv {
    fn drop(&mut self) {
        if let Some(val) = &self.original {
            unsafe { env::set_var("XDG_CONFIG_HOME", val); }
        } else {
            unsafe { env::remove_var("XDG_CONFIG_HOME"); }
        }
        let _ = fs::remove_dir_all(&self.temp_dir);
    }
}

// Ollama helpers
fn is_ollama_available() -> bool {
    ProcessCommand::new("which").arg("ollama").output().map_or(false, |o| o.status.success())
}

fn is_ollama_model_available(model: &str) -> bool {
    if !is_ollama_available() {
        return false;
    }
    ProcessCommand::new("ollama").arg("list").output()
        .map_or(false, |o| String::from_utf8_lossy(&o.stdout).contains(model))
}

// Decide if external tests should run
fn should_run_external_api_tests() -> bool {
    load_test_env();
    env::var("ENABLE_EXTERNAL_API_TESTS").map_or(false, |v| v == "true")
}

// --- CLI parsing tests ---
#[test]
fn test_ask_parsing() {
    let cli = Cli::try_parse_from(&["yo", "ask", "Hello"]).unwrap();
    match cli.command {
        Some(CliCommand::Ask { question }) => assert_eq!(question, vec!["Hello"]),
        _ => panic!("Expected Ask"),
    }
}

#[test]
fn test_setup_parsing() {
    let cli = Cli::try_parse_from(&["yo", "setup"]).unwrap();
    assert!(matches!(cli.command, Some(CliCommand::Setup)));
}

#[test]
fn test_config_parsing() {
    let cli = Cli::try_parse_from(&["yo", "config"]).unwrap();
    assert!(matches!(cli.command, Some(CliCommand::Config)));
}

#[test]
fn test_switch_parsing() {
    let cli = Cli::try_parse_from(&["yo", "switch", "openai"]).unwrap();
    match cli.command {
        Some(CliCommand::Switch { model }) => assert_eq!(model, "openai"),
        _ => panic!("Expected Switch openai"),
    }
}

#[test]
fn test_gpt_parsing() {
    let cli = Cli::try_parse_from(&["yo", "gpt", "gpt-4"]).unwrap();
    match cli.command {
        Some(CliCommand::Gpt { model }) => assert_eq!(model, "gpt-4"),
        _ => panic!("Expected Gpt gpt-4"),
    }
}

#[test]
fn test_list_parsing() {
    let cli = Cli::try_parse_from(&["yo", "list"]).unwrap();
    assert!(matches!(cli.command, Some(CliCommand::List)));
}

#[test]
fn test_current_parsing() {
    let cli = Cli::try_parse_from(&["yo", "current"]).unwrap();
    assert!(matches!(cli.command, Some(CliCommand::Current)));
}

#[test]
fn test_other_parsing() {
    let cli = Cli::try_parse_from(&["yo", "foo", "bar"]).unwrap();
    match cli.command {
        Some(CliCommand::Other(args)) => assert_eq!(args, vec!["foo", "bar"]),
        _ => panic!("Expected Other"),
    }
}

// --- Command functionality tests ---
#[test]
fn test_show_config_path() {
    let _env = TestEnv::new();
    commands::show_config_path();
}

#[test]
fn test_show_current() {
    let _env = TestEnv::new();
    commands::show_current();
}

#[test]
fn test_ollama_avail() {
    if !is_ollama_available() {
        eprintln!("Ollama is not installed or not available in PATH, skipping test_ollama_avail");
        return;
    }
    assert!(is_ollama_available(), "Ollama is not installed or not available in PATH");
}

#[tokio::test]
#[serial]
async fn test_set_gpt() {
    if !should_run_external_api_tests() { return; }
    let _env = TestEnv::new();
    // Debug: print config path and contents
    let config_path = std::env::temp_dir().join("yo_test_config/yo/config.toml");
    if let Ok(contents) = std::fs::read_to_string(&config_path) {
        println!("Config contents before set_gpt:\n{}", contents);
    }
    commands::set_gpt("gpt-3.5-turbo").await;
}

#[tokio::test]
#[serial]
async fn test_ask_openai() {
    if !should_run_external_api_tests() { return; }
    let _env = TestEnv::new();
    let config_path = std::env::temp_dir().join("yo_test_config/yo/config.toml");
    if let Ok(contents) = std::fs::read_to_string(&config_path) {
        println!("Config contents before ask_openai:\n{}", contents);
    }
    commands::ask(&["Ping".into()]).await;
}

#[tokio::test]
#[serial]
async fn test_ask_ollama() {
    if !should_run_external_api_tests() { return; }
    let model = env::var("OLLAMA_TEST_MODEL").unwrap_or_else(|_| "llama3".into());
    if !is_ollama_model_available(&model) { return; }
    let _env = TestEnv::new();
    commands::switch("ollama").await;
    commands::ask(&["Ping".into()]).await;
}

#[tokio::test]
#[serial]
async fn test_switch_cmd() {
    if !should_run_external_api_tests() { return; }
    let _env = TestEnv::new();
    commands::switch("openai").await;
}