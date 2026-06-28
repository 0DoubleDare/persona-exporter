use config::{Config, ConfigError, File};
use persona_exporter_types::ComponentInfo;
use serde::Deserialize;
use std::env;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct AgentConfigFile {
    pub agent: AgentSection,
    pub server: ServerSection,
    pub metrics: MetricsConfig,
}

#[derive(Deserialize, Debug)]
pub struct MetricsConfig {
    pub cpu: CpuConfig,
    pub disks: DisksConfig,
    pub network: NetworkConfig,
    pub system: SystemConfig,
    pub components: ComponentsConfig,
    pub memory: MemoryConfig,
}

#[derive(Deserialize, Debug)]
pub struct MemoryConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

#[derive(Deserialize, Debug)]
pub struct ComponentsConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}
#[derive(Deserialize, Debug)]
pub struct CpuConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

#[derive(Deserialize, Debug)]
pub struct DisksConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

#[derive(Deserialize, Debug)]
pub struct NetworkConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

#[derive(Deserialize, Debug)]
pub struct SystemConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

#[derive(Deserialize, Debug)]
pub struct AgentSection {
    pub send_metrics_interval: u64,
    #[serde(default)]
    pub debug_mode: bool,
}

#[derive(Deserialize, Debug)]
pub struct ServerSection {
    pub server_url: String,
    pub server_key: String,
}

impl AgentConfigFile {
    pub fn new() -> Result<Self, ConfigError> {
        const DEFAULT_CONFIG_CONTENT: &str = include_str!("./config.example.toml");

        let config_path = env::var("PERSONA_CONFIG_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/etc/persona-rs-agent/config.toml"));

        if !config_path.exists() {
            if let Some(parent) = config_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    ConfigError::Message(format!("Не удалось создать директорию: {e}"))
                })?;
            }

            std::fs::write(&config_path, DEFAULT_CONFIG_CONTENT).map_err(|e| {
                ConfigError::Message(format!(
                    "Не удалось заполнить конфиг данными по умолчанию: {e}"
                ))
            })?;
        }

        let s = Config::builder()
            .add_source(File::from(config_path).required(true))
            .add_source(config::Environment::with_prefix("PERSONA_AGENT").separator("__"))
            .build()?;

        s.try_deserialize()
    }
}
fn default_enabled() -> bool {
    true
}
