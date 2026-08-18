use crate::config::*;
use config::{Config, ConfigError};
use config_shellexpand::TemplatedFile;
use std::env;
use std::path::PathBuf;
use tracing::info;

impl AgentConfigFile {
    pub fn new() -> Result<Self, ConfigError> {
        let config_directory = env::var("PERSONA_EXPORTER_CONFIG_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                if cfg!(target_os = "linux") {
                    PathBuf::from("/etc/persona-exporter")
                } else {
                    dirs::config_dir().unwrap_or_default()
                }
            });
        let config_path = config_directory.join("config.toml");

        info!("Current config directory: {:?}", config_directory);
        info!("You might change config directory through env var 'PERSONA_EXPORTER_CONFIG_DIR'");
        info!("Current full config path: {:?}", config_path);

        Config::builder()
            .add_source(Config::try_from(&Self::default())?)
            .add_source(TemplatedFile::with_name(config_path).required(false))
            .add_source(config::Environment::with_prefix("PE").separator("__"))
            .build()?
            .try_deserialize()
    }
}
impl Default for AgentConfigFile {
    fn default() -> Self {
        AgentConfigFile {
            agent: AgentSection {
                send_interval: 10,
                send_model: SendModel::default(),
                data_type: DataType::default(),
            },
            server: ServerSection {
                url: "https://example.com".to_string(),
                retries_connection: None,
                get_params: Vec::new(),
                http_headers: Vec::new(),
            },
            metrics: MetricsConfig {
                processes: ProcessListConfig {
                    settings: CommonMetricSetting::default(),
                    process_limit: 5,
                    sort_by: ProcessSortBy::default(),
                },
                cpu: CpuConfig {
                    settings: CommonMetricSetting::default(),
                },
                disks: DisksConfig {
                    settings: CommonMetricSetting::default(),
                    ignore_fs_types: vec![String::from("tmpfs")],
                    ignore_mount_points: vec![String::from("/mnt/backup_test")],
                },
                network: NetworkConfig {
                    settings: CommonMetricSetting::default(),
                    list_type: ListType::default(),
                    interfaces: vec![String::from("lo"), String::from("docker0")],
                },
                system: SystemConfig {
                    settings: CommonMetricSetting::default(),
                },
                components: ComponentsConfig {
                    settings: CommonMetricSetting::default(),
                },
                memory: MemoryConfig {
                    settings: CommonMetricSetting::default(),
                },
            },
        }
    }
}

impl Default for CommonMetricSetting {
    fn default() -> Self {
        CommonMetricSetting {
            enabled: true,
            override_interval: None,
            override_retries_connection: None,
        }
    }
}
