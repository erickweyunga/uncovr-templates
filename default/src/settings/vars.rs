#![allow(unused)]

use serde::Deserialize;
use uncovr::config::Environment;

#[derive(Deserialize, Debug, Clone)]
#[serde(default)]
pub struct AppConfig {
    /// Application configuration
    pub app: App,

    /// Project environment configuration
    pub environment: Environment,
}

/// Application configuration
#[derive(Deserialize, Debug, Clone)]
#[serde(default)]
pub struct App {
    /// Application name
    pub name: String,

    /// Application description
    pub description: String,

    /// Application version
    pub version: String,

    /// App Public Address
    pub address: String,

    /// App Port
    pub port: u16,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app: App::default(),
            environment: Environment::default(),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            name: "{{project_name}}".to_string(),
            description: "App built with Uncovr".to_string(),
            version: "0.1.0".to_string(),
            address: "127.0.0.1".to_string(),
            port: 8000,
        }
    }
}
