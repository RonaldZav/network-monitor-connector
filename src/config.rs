use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use rand::{distributions::Alphanumeric, Rng};

const CONFIG_FILE: &str = "config.yml";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WhitelistConfig {
    pub enabled: bool,
    pub list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub whitelist: WhitelistConfig,
    pub token: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            host: "0.0.0.0".to_string(),
            port: 2141,
            whitelist: WhitelistConfig {
                enabled: true,
                list: vec!["127.0.0.1/32".to_string()],
            },
            token: generate_token(),
        }
    }
}

fn generate_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

pub fn load_or_create_config() -> AppConfig {
    let mut config = if Path::new(CONFIG_FILE).exists() {
        let content = fs::read_to_string(CONFIG_FILE).expect("Failed to read config file");
        serde_yaml::from_str(&content).unwrap_or_else(|_| {
            eprintln!("Failed to parse config file, using defaults");
            AppConfig::default()
        })
    } else {
        AppConfig::default()
    };

    if config.token.is_empty() {
        config.token = generate_token();
        save_config(&config);
    }

    config
}

fn save_config(config: &AppConfig) {
    let content = serde_yaml::to_string(config).expect("Failed to serialize config");
    fs::write(CONFIG_FILE, content).expect("Failed to write config file");
    println!("Config saved to {}", CONFIG_FILE);
}
