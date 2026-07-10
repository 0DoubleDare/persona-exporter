use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::path::PathBuf;
use std::*;

#[derive(Deserialize, Debug)]
pub struct AgentConfigFile {
    pub agent: AgentSection,
    pub server: ServerSection,
    pub metrics: MetricsConfig,
}

#[derive(Deserialize, Debug, Default)]
pub struct MetricsConfig {
    pub cpu: CpuConfig,
    pub disks: DisksConfig,
    pub network: NetworkConfig,
    pub system: SystemConfig,
    pub components: ComponentsConfig,
    pub memory: MemoryConfig,
}

#[derive(Deserialize, Debug, Default)]
pub struct MemoryConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

#[derive(Deserialize, Debug, Default)]
pub struct ComponentsConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}
#[derive(Deserialize, Debug, Default)]
pub struct CpuConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

#[derive(Deserialize, Debug, Default)]
pub struct DisksConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

#[derive(Deserialize, Debug, Default)]
pub struct NetworkConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

#[derive(Deserialize, Debug, Default)]
pub struct SystemConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

#[derive(Deserialize, Debug, Default)]
pub struct AgentSection {
    pub send_metrics_interval: u64,
    #[serde(default)]
    pub debug_mode: bool,
}

#[derive(Deserialize, Debug, Default)]
pub struct ServerSection {
    pub server_url: String,
    pub server_key: String,
}

impl AgentConfigFile {
    pub fn new() -> Result<Self, ConfigError> {
        // const DEFAULT_CONFIG_CONTENT: &str = include_str!("config.example.toml");

        let config_path = env::var("PERSONA_EXPORTER_CONFIG_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/etc/persona-exporter/config.toml"));

        // match fs::metadata(&config_path) {
        //     Ok(metadata) => {
        //         tracing::info!("{:#?}", metadata);
        //         if metadata.len() <= 0 {
        //             fs::write(&config_path, DEFAULT_CONFIG_CONTENT)
        //                 .map_err(|e| {
        //                     ConfigError::Message(format!(
        //                         "Failed to write data in empty config file: {}",
        //                         e
        //                     ))
        //                 })
        //                 .err();
        //         }
        //     }
        //     Err(e) => {
        //         tracing::info!("Persona-exporter config error: {}", e);
        //     }
        // };
        // if !config_path.exists() {
        //     std::fs::write(&config_path, DEFAULT_CONFIG_CONTENT).map_err(|e| {
        //         ConfigError::Message(format!(
        //             "Failed to populate the configuration with default data: {e}"
        //         ))
        //     })?;
        // }

        let s = Config::builder()
            .add_source(File::from(config_path).required(true))
            .add_source(config::Environment::with_prefix("PERSONA_EXPORTER").separator("__"))
            .build()?;

        s.try_deserialize()
    }
}
fn default_enabled() -> bool {
    true
}

impl Default for AgentConfigFile {
    fn default() -> Self {
        AgentConfigFile {
            agent: AgentSection {
                send_metrics_interval: 5,
                debug_mode: false,
            },
            server: ServerSection {
                server_url: "put-your-target-url".to_string(),
                server_key: "put-your-token".to_string(),
            },
            metrics: MetricsConfig {
                cpu: CpuConfig { enabled: true },
                disks: DisksConfig { enabled: true },
                network: NetworkConfig { enabled: true },
                system: SystemConfig { enabled: true },
                components: ComponentsConfig { enabled: true },
                memory: MemoryConfig { enabled: true },
            },
        }
    }
}
