//! Configuration management

use config::{Config, ConfigError, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub app: AppSettings,
    pub storage: StorageConfig,
    pub ocr: OcrConfig,
    pub llm: LlmConfig,
    pub sync: SyncConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub name: String,
    pub version: String,
    pub data_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    #[serde(rename = "type")]
    pub storage_type: String,
    pub database_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrConfig {
    pub default: String,
    pub providers: HashMap<String, ProviderConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub default: String,
    pub providers: HashMap<String, LlmProviderConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub enabled: bool,
    pub server_url: Option<String>,
    #[serde(default = "default_sync_interval")]
    pub sync_interval: u64,
}

fn default_sync_interval() -> u64 {
    300
}

/// Generic provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub enabled: bool,
    #[serde(default)]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub secret_key: Option<String>,
    #[serde(default = "default_timeout")]
    pub timeout: u64,
}

/// LLM-specific provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmProviderConfig {
    pub enabled: bool,
    #[serde(default)]
    pub endpoint: Option<String>,
    pub model: Option<String>,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default = "default_llm_timeout")]
    pub timeout: u64,
}

fn default_timeout() -> u64 {
    30
}

fn default_llm_timeout() -> u64 {
    120
}

impl AppConfig {
    /// Load configuration from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::new(path.as_ref().to_str().unwrap(), FileFormat::Yaml))
            .build()?;

        config.try_deserialize()
    }

    /// Load configuration from default locations
    pub fn load() -> Result<Self, ConfigError> {
        // Try to load from common locations
        let locations = [
            "./config.yaml",
            "./config/config.yaml",
            "~/.config/health-keeper/config.yaml",
        ];

        for location in locations {
            let path = shellexpand::tilde(location).to_string();
            if std::path::Path::new(&path).exists() {
                return Self::from_file(&path);
            }
        }

        // Return default configuration
        Ok(Self::default())
    }

    /// Save configuration to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error> {
        let content = serde_yaml::to_string(self).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
        })?;

        std::fs::write(path, content)
    }

    /// Get the data directory path
    pub fn data_dir(&self) -> std::path::PathBuf {
        shellexpand::tilde(&self.app.data_dir).into_owned().into()
    }

    /// Get the database URL
    pub fn database_url(&self) -> String {
        shellexpand::tilde(&self.storage.database_url).into_owned()
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut ocr_providers = HashMap::new();
        ocr_providers.insert(
            "paddle_local".to_string(),
            ProviderConfig {
                enabled: true,
                endpoint: Some("http://127.0.0.1:8868".to_string()),
                model: None,
                api_key: None,
                secret_key: None,
                timeout: 30,
            },
        );

        let mut llm_providers = HashMap::new();
        llm_providers.insert(
            "ollama_local".to_string(),
            LlmProviderConfig {
                enabled: true,
                endpoint: Some("http://127.0.0.1:11434".to_string()),
                model: Some("qwen2.5:7b".to_string()),
                api_key: None,
                timeout: 120,
            },
        );

        Self {
            app: AppSettings {
                name: "HealthKeeper".to_string(),
                version: "0.1.0".to_string(),
                data_dir: "./data".to_string(),
            },
            storage: StorageConfig {
                storage_type: "sqlite".to_string(),
                database_url: "sqlite:./data/healthkeeper.db?mode=rwc".to_string(),
            },
            ocr: OcrConfig {
                default: "paddle_local".to_string(),
                providers: ocr_providers,
            },
            llm: LlmConfig {
                default: "ollama_local".to_string(),
                providers: llm_providers,
            },
            sync: SyncConfig {
                enabled: false,
                server_url: None,
                sync_interval: 300,
            },
        }
    }
}