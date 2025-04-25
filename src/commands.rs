use crate::config::{append_history, get_config_path, load_or_create_config, save_config, Config, get_history_path};
use prettytable::{Table, Row, Cell};
use reqwest::Client;
use serde_json::Value;
use std::io::{self, Write};
use std::process::Command as ShellCommand;
use std::fs;

async fn fetch_openai_models(api_key: &str) -> Vec<String> {
    let client = Client::new();
    let res = client
        .get("https://api.openai.com/v1/models")
        .bearer_auth(api_key)
        .send()
        .await
        .expect("failed to fetch models");
    let j: Value = res.json().await.expect("invalid JSON");
    j["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| m["id"].as_str().unwrap().to_string())
        .collect()
}

fn fetch_ollama_local() -> Vec<String> {
    let out = ShellCommand::new("ollama").arg("list").output().expect("ollama list failed");
    let lines: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect();
        
    if lines.len() > 1 {
        lines.iter()
            .skip(1)
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if !parts.is_empty() {
                    Some(parts[0].to_string())
                } else {
                    None
                }
            })
            .collect()
    } else {
        Vec::new()
    }
}

/// Clear the conversation history
pub fn clear_history() {
    let path = get_history_path();
    if path.exists() {
        if let Err(e) = fs::write(&path, "") {
            eprintln!("Failed to clear history: {}", e);
        } else {
            println!("✅ History cleared");
        }
    } else {
        println!("No history file found to clear.");
    }
}

/// Setup initial source & API key
pub fn setup() {
    // TODO: add more options
    print!("Choose backend (1) ollama  (2) openai: ");
    io::stdout().flush().unwrap();
    let mut c = String::new(); io::stdin().read_line(&mut c).unwrap();
    let src = match c.trim() {
        "1" => "ollama",
        "2" => "openai",
        _ => { eprintln!("invalid"); return; }
    }.to_string();

    let mut key = None;
    if src == "openai" {
        print!("Enter OpenAI API key: ");
        io::stdout().flush().unwrap();
        let mut k2 = String::new(); io::stdin().read_line(&mut k2).unwrap();
        key = Some(k2.trim().to_string());
    }

    // pick default model
    let default_model = if src == "openai" { "gpt-4".to_string() } else { 
        let loc = fetch_ollama_local();
        loc.get(0).cloned().unwrap_or_else(|| {
            eprintln!("no local ollama model installed");
            std::process::exit(1);
        })
    };

    let cfg = Config { source: src.clone(), model: default_model, openai_api_key: key };
    save_config(&cfg);
    println!("✅ setup complete");
    println!("⚙️ config saved at {}", get_config_path().display());
}

/// Show config path
pub fn show_config_path() {
    println!("{}", get_config_path().display());
}

/// Set a specific GPT model
pub async fn set_gpt(gpt_model: &str) {
    let mut cfg = load_or_create_config();
    cfg.source = "openai".into();
    
    // Make sure we have an API key
    if cfg.openai_api_key.is_none() {
        print!("Enter OpenAI API key: ");
        io::stdout().flush().unwrap();
        let mut k = String::new(); io::stdin().read_line(&mut k).unwrap();
        cfg.openai_api_key = Some(k.trim().to_string());
    }
    
    // Verify the model exists
    let models = fetch_openai_models(cfg.openai_api_key.as_ref().unwrap()).await;
    if models.iter().any(|m| m == gpt_model) {
        cfg.model = gpt_model.to_string();
        save_config(&cfg);
        println!("Switched to OpenAI model: {}", gpt_model);
        println!("⚙️ config saved at {}", get_config_path().display());
    } else {
        println!("Model '{}' not found in available OpenAI models.", gpt_model);
        println!("Available models:");
        for model in models.iter().take(10) {
            println!("  {}", model);
        }
        if models.len() > 10 {
            println!("  ... and {} more", models.len() - 10);
            println!("Run 'yo list' to see all available models");
        }
    }
}

/// Switch source or model
pub async fn switch(model: &str) {
    let mut cfg = load_or_create_config();
    if model == "openai" {
        cfg.source = "openai".into();
        if cfg.openai_api_key.is_none() {
            print!("Enter OpenAI API key: ");
            io::stdout().flush().unwrap();
            let mut k = String::new(); io::stdin().read_line(&mut k).unwrap();
            cfg.openai_api_key = Some(k.trim().to_string());
        }
        
        // Always use the previously selected OpenAI model if available in config
        // or default to gpt-4o-realtime-preview if no OpenAI model was previously selected
        if !cfg.model.starts_with("gpt-") && 
           !["o1", "o3", "o4", "dall-e"].iter().any(|prefix| cfg.model.starts_with(prefix)) {
            // No OpenAI model found, set default
            cfg.model = "gpt-4o-realtime-preview".to_string();
        }
        
        println!("Switched to OpenAI model: {}", cfg.model);
        save_config(&cfg);
        println!("⚙️ config saved at {}", get_config_path().display());
        return;
    } else if model == "ollama" {
        cfg.source = "ollama".into();
        let loc = fetch_ollama_local();
        if loc.is_empty() {
            eprintln!("❌ No local Ollama models found. Please install one first with:");
            eprintln!(" ollama pull llama3");
            eprintln!("\nVisit https://ollama.com/search to discover available models.");
            return;
        }
        
        // Check if the current model is an Ollama model (not starting with "gpt-" or other OpenAI prefixes)
        if !cfg.model.starts_with("gpt-") && 
           !["o1", "o3", "o4", "dall-e"].iter().any(|prefix| cfg.model.starts_with(prefix)) {
            // Current model is likely an Ollama model
            // Check if it exists in available models
            if loc.iter().any(|m| m == &cfg.model) {
                println!("Using previously selected Ollama model: {}", cfg.model);
                save_config(&cfg);
                println!("⚙️ config saved at {}", get_config_path().display());
                return;
            }
        }
        
        // If we don't have a valid Ollama model in config, just use the first available one
        cfg.model = loc[0].clone();
        println!("Switched to Ollama model: {}", cfg.model);
    } else {
        eprintln!("usage: yo switch <ollama|openai>");
        return;
    }
    save_config(&cfg);
    println!("switched to {}:{}", cfg.source, cfg.model);
    println!("⚙️ config saved at {}", get_config_path().display());
}

/// List available across both backends
pub async fn list_models() {
    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new("Src"), Cell::new("Model"), Cell::new("You")]));

    let cfg = load_or_create_config();
    if let Some(key) = cfg.openai_api_key.as_deref() {
        for m in fetch_openai_models(key).await {
            let you = if cfg.source=="openai" && cfg.model==m { "✔" } else { "" };
            table.add_row(Row::new(vec![Cell::new("OpenAI"), Cell::new(&m), Cell::new(you)]));
        }
    }
    for m in fetch_ollama_local() {
        let you = if cfg.source=="ollama" && cfg.model==m { "✔" } else { "" };
        table.add_row(Row::new(vec![Cell::new("Ollama"), Cell::new(&m), Cell::new(you)]));
    }
    table.printstd();
}

/// Ask current model; for Ollama, use ollama run and exit with /bye
pub async fn ask(question: &[String]) {
    let cfg = load_or_create_config();
    let prompt = question.join(" ");

    if cfg.source == "openai" {
        let mut messages = Vec::new();
        messages.push(serde_json::json!({
            "role": "system",
            "content": "You are a helpful AI assistant."
        }));
        messages.push(serde_json::json!({"role": "user", "content": prompt.clone()}));

        let client = Client::new();
        let body = serde_json::json!({
            "model": cfg.model,
            "messages": messages,
            "stream": true
        });
        
        let mut res = client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(cfg.openai_api_key.as_ref().unwrap())
            .json(&body)
            .send()
            .await
            .expect("OpenAI request failed");
            
        let mut full_response = String::new();
        
        let mut buffer = Vec::new();
        
        while let Ok(Some(chunk)) = res.chunk().await {
            let chunk_str = String::from_utf8_lossy(&chunk);
            buffer.extend_from_slice(chunk_str.as_bytes());
            
            let buffer_str = String::from_utf8_lossy(&buffer);
            let mut processed_up_to = 0;
            
            for (_i, line) in buffer_str.split('\n').enumerate() {
                if line.starts_with("data: ") {
                    let data = &line[6..]; // Skip "data: "
                    
                    if data.trim() == "[DONE]" {
                        continue;
                    }
                    
                    if let Ok(json) = serde_json::from_str::<Value>(data) {
                        if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                            print!("{}", content);
                            io::stdout().flush().unwrap();
                            full_response.push_str(content);
                        }
                    }
                    
                    processed_up_to = buffer_str[..buffer_str.find(line).unwrap_or(0) + line.len() + 1].len();
                }
            }
            
            if processed_up_to > 0 {
                buffer = buffer.split_off(processed_up_to);
            }
        }
        
        println!(); 
        append_history(&format!("Q: {}\nA: {}", prompt, full_response));
    } else {
        let mut child = ShellCommand::new("ollama")
            .arg("run")
            .arg(&cfg.model)
            .arg(&prompt)
            .spawn()
            .expect("Failed to start ollama run");
        

        let status = child.wait().expect("Failed to wait for ollama");
        
        if !status.success() {
            println!("Ollama command failed with status: {}", status);
        }
    }
}

/// Show information about the current model in use
pub fn show_current() {
    let cfg = load_or_create_config();
    
    println!("📋 Current AI Configuration");
    println!("---------------------------");
    println!("Backend: {}", cfg.source);
    println!("Model:   {}", cfg.model);
    
    if cfg.source == "ollama" {
        let output = ShellCommand::new("ollama")
            .args(["show", &cfg.model])
            .output();
            
        if let Ok(out) = output {
            let info = String::from_utf8_lossy(&out.stdout);
            if !info.is_empty() {
                let lines: Vec<&str> = info.lines().take(5).collect();
                if !lines.is_empty() {
                    println!("\nModel Details:");
                    for line in lines {
                        println!("  {}", line);
                    }
                }
            }
        }
    } else if cfg.source == "openai" {
        if let Some(api_key) = cfg.openai_api_key.as_deref() {
            if api_key.len() > 7 {
                let visible_part = &api_key[..7];
                let masked_part = "*".repeat(api_key.len() / 4);
                println!("\nAPI Key: {}{}", visible_part, masked_part);
            } else {
                println!("\nAPI Key: {}", "*".repeat(api_key.len()));
            }
        } else {
            println!("\nAPI Key: [not set]");
        }
    }
    
    println!("\n💡 Use 'yo list' to see all available models");
}

