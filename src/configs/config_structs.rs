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
    pub enabled: bool,
}

#[derive(Deserialize, Debug, Default)]
pub struct ComponentsConfig {
    pub enabled: bool,
}
#[derive(Deserialize, Debug, Default)]
pub struct CpuConfig {
    pub enabled: bool,
}

#[derive(Deserialize, Debug, Default)]
pub struct DisksConfig {
    pub enabled: bool,
}

#[derive(Deserialize, Debug, Default)]
pub struct NetworkConfig {
    pub enabled: bool,
}

#[derive(Deserialize, Debug, Default)]
pub struct SystemConfig {
    pub enabled: bool,
}

#[derive(Deserialize, Debug, Default)]
pub struct AgentSection {
    pub send_metrics_interval: u64,
}

#[derive(Deserialize, Debug, Default)]
pub struct ServerSection {
    pub server_url: String,
    pub server_key: String,
    pub enable_server_key: bool,
}

impl AgentConfigFile {
    pub fn new() -> Result<Self, ConfigError> {
        let config_path = env::var("PERSONA_EXPORTER_CONFIG_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/etc/persona-exporter/config.toml"));

        let s = Config::builder()
            .add_source(File::from(config_path).required(true))
            .add_source(config::Environment::with_prefix("PERSONA_EXPORTER").separator("__"))
            .build()?;

        s.try_deserialize()
    }
}
impl Default for AgentConfigFile {
    fn default() -> Self {
        AgentConfigFile {
            agent: AgentSection {
                send_metrics_interval: 5,
            },
            server: ServerSection {
                server_url: "https://example.com".to_string(),
                server_key: "put-your-token".to_string(),
                enable_server_key: true,
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
