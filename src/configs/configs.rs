use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentConfigFile {
    pub agent: AgentSection,
    pub server: ServerSection,
    pub metrics: MetricsConfig,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MetricsConfig {
    pub cpu: CpuConfig,
    pub disks: DisksConfig,
    pub network: NetworkConfig,
    pub system: SystemConfig,
    pub components: ComponentsConfig,
    pub memory: MemoryConfig,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MemoryConfig {
    pub settings: CommonMetricSetting,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ComponentsConfig {
    pub settings: CommonMetricSetting,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CpuConfig {
    pub settings: CommonMetricSetting,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DisksConfig {
    pub settings: CommonMetricSetting,
    pub ignore_fs_types: Vec<String>,
    pub ignore_mount_points: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NetworkConfig {
    pub settings: CommonMetricSetting,
    pub list_type: ListType,
    pub interfaces: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SystemConfig {
    pub settings: CommonMetricSetting,
    pub collect_processes: bool,
    pub process_limit: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AgentSection {
    pub send_metrics_interval: u64,
    pub send_model: SendModel,
    pub data_type: DataType,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ServerSection {
    pub url: String,
    pub bearer_token: String,
    pub enable_server_key: bool,
    pub additional_get_params: Vec<ParamField>,
    pub additional_headers: Vec<HeaderConfig>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct HeaderConfig {
    pub header: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ParamField {
    pub variable: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CommonMetricSetting {
    pub enabled: bool,
    pub override_interval: u32,
}
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum SendModel {
    #[default]
    Pull,
    Push,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DataType {
    #[default]
    Json,
    LineProtocol,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ListType {
    WhiteList,
    #[default]
    IgnoreList,
}

impl AgentConfigFile {
    pub fn new() -> Result<Self, ConfigError> {
        let config_path = env::var("PERSONA_EXPORTER_CONFIG_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/etc/persona-exporter/config.toml"));

        let s = Config::builder()
            .add_source(config::Config::try_from(&Self::default())?)
            .add_source(File::from(config_path).required(false))
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
                send_model: SendModel::default(),
                data_type: DataType::default(),
            },
            server: ServerSection {
                url: "https://example.com".to_string(),
                bearer_token: "".to_string(),
                enable_server_key: true,
                additional_get_params: Vec::new(),
                additional_headers: Vec::new(),
            },
            metrics: MetricsConfig {
                cpu: CpuConfig { enabled: true },
                disks: DisksConfig {
                    enabled: true,
                    ignore_fs_types: vec![String::from("tmpfs")],
                    ignore_mount_points: vec![String::from("/mnt/backup_test")],
                },
                network: NetworkConfig {
                    enabled: true,
                    list_type: ListType::default(),
                    interfaces: vec![String::from("lo"), String::from("docker0")],
                },
                system: SystemConfig {
                    enabled: true,
                    collect_processes: true,
                    process_limit: 5,
                },
                components: ComponentsConfig { enabled: true },
                memory: MemoryConfig { enabled: true },
            },
        }
    }
}
