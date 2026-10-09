use crate::config::{
    AgentConfigFile, AgentSection, CommonMetricSetting, ComponentsConfig, CpuConfig, DataType,
    DisksConfig, ListType, MemoryConfig, MetricsConfig, NetworkConfig, ProcessListConfig,
    ProcessSortBy, SectionPullModel, SectionPushModel, SendModel, ServerSection, SystemConfig,
};
use compact_str::{CompactString, ToCompactString};
use config::{Config, ConfigError};
use config_shellexpand::TemplatedFile;
use std::collections::HashMap;
use std::env;
use std::fs::{create_dir_all, write};
use std::path::PathBuf;
use tracing::{info, warn};

const CONFIG_FILENAME: &str = "config.yaml";

impl AgentConfigFile {
    #[must_use]
    pub fn match_config_path_by_os() -> PathBuf {
        if cfg!(target_os = "linux") {
            PathBuf::from("/etc/persona-exporter")
        } else {
            PathBuf::from(env::var("ProgramData").unwrap_or_else(|_| r"C:\ProgramData".to_string()))
                .join("PersonaMetrics")
                .join("PersonaExporter")
        }
    }
    pub fn new_with(override_config_path: Option<CompactString>) -> Result<Self, ConfigError> {
        // let config_path: PathBuf = {
        //
        //     if let Some(override_path) = override_config_path {
        //         PathBuf::from(override_path)
        //     } else {
        //         env::var("PERSONA_EXPORTER_CONFIG_PATH")
        //             .map(PathBuf::from)
        //             .unwrap_or_else(|_| {
        //                 // Configuration path for Linux
        //                 if cfg!(target_os = "linux") {
        //                     PathBuf::from("/etc/persona-exporter")
        //                 } else {
        //                     // Configuration path for Windows
        //                     PathBuf::from(
        //                         env::var("ProgramData")
        //                             .unwrap_or_else(|_| r"C:\Program Data".to_string()),
        //                     )
        //                     .join("PersonaMetrics")
        //                     .join("PersonaExporter")
        //                 }
        //                 .join(CONFIG_FILENAME)
        //             })
        //     }
        // };

        let config_path = override_config_path.map_or_else(
            || {
                env::var("PERSONA_EXPORTER_CONFIG_PATH").map_or_else(
                    |_| Self::match_config_path_by_os().join(CONFIG_FILENAME),
                    PathBuf::from,
                )
            },
            PathBuf::from,
        );

        if !config_path.exists() {
            // Require SUDO for write in systems directories
            if let Some(parent) = &config_path.parent() {
                create_dir_all(parent).expect("Failed to create directories");
            }

            std::fs::File::create(&config_path).expect("Failed to create file");

            let write_result = write(&config_path, include_str!("../config.example.yaml"));

            match write_result {
                Ok(()) => {
                    info!("Successfully insert default config to {:?}", config_path);
                }
                Err(err) => {
                    warn!(
                        "Failed to insert default config to {:?}. File has been created, but still empty - {}",
                        config_path, err
                    );
                }
            }
        }

        info!("You might change config path through env var 'PERSONA_EXPORTER_CONFIG_PATH'");
        info!(
            "Example (Linux): export PERSONA_EXPORTER_CONFIG_PATH=/home/alice/.config/myconfig.yaml"
        );
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
        Self {
            agent: AgentSection {
                send_model: SendModel::default(),
                data_type: DataType::default(),
            },
            server: ServerSection {
                push: SectionPushModel {
                    url: "https://example.com".to_string(),
                    retries_connection: None,
                    send_interval: 5,
                    // url_params: Vec::new(),
                    // http_headers: Vec::new(),
                    http_headers: HashMap::default(),
                    url_params: HashMap::default(),
                },
                pull: SectionPullModel {
                    route: "metrics".to_string(),
                    server_hostname: "localhost".to_string(),
                    port: 3434,
                },
            },
            metrics: MetricsConfig {
                global_tags: HashMap::from([(
                    "hostname".to_compact_string(),
                    "name-your-server".to_compact_string(),
                )]),
                processes: ProcessListConfig {
                    settings: CommonMetricSetting::default(),
                    process_limit: 5,
                    include_exporter_metrics: true,
                    remove_dead_processes: true,
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
        Self {
            enabled: true,
            override_interval: None,
            override_retries_connection: None,
        }
    }
}
