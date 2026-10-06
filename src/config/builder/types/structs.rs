use crate::config::{DataType, HttpHeaders, ListType, ProcessSortBy, SendModel, UrlParams};
use compact_str::CompactString;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentConfigFile {
    pub agent: AgentSection,
    pub server: ServerSection,
    pub metrics: MetricsConfig,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MetricsConfig {
    pub global_tags: HashMap<CompactString, CompactString>,
    // #[serde(rename = "global")]
    // pub global_config: GlobalMetricsConfig,
    pub cpu: CpuConfig,
    pub disks: DisksConfig,
    pub network: NetworkConfig,
    pub system: SystemConfig,
    pub components: ComponentsConfig,
    pub memory: MemoryConfig,
    pub processes: ProcessListConfig,
}

// #[derive(Serialize, Deserialize, Debug, Default, Clone)]
// pub struct GlobalMetricsConfig {
//     pub tags: HashMap<CompactString, CompactString>,
// }

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProcessListConfig {
    #[serde(flatten)]
    pub settings: CommonMetricSetting,
    pub process_limit: usize,
    pub include_exporter_metrics: bool,
    pub remove_dead_processes: bool,
    pub sort_by: ProcessSortBy,
    // pub sort_by: [Option<ProcessListSortConfig>; 5],
}

// Задел на будущее
pub struct EnabledSortByProcessList {
    pub cpu_usage: bool,
    pub memory: bool,
    pub virtual_memory: bool,
    pub run_time: bool,
    pub start_time: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MemoryConfig {
    #[serde(flatten)]
    pub settings: CommonMetricSetting,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ComponentsConfig {
    #[serde(flatten)]
    pub settings: CommonMetricSetting,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CpuConfig {
    #[serde(flatten)]
    pub settings: CommonMetricSetting,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DisksConfig {
    #[serde(flatten)]
    pub settings: CommonMetricSetting,
    pub ignore_fs_types: Vec<String>,
    pub ignore_mount_points: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NetworkConfig {
    #[serde(flatten)]
    pub settings: CommonMetricSetting,
    pub list_type: ListType,
    pub interfaces: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SystemConfig {
    #[serde(flatten)]
    pub settings: CommonMetricSetting,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AgentSection {
    pub send_model: SendModel,
    pub data_type: DataType,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ServerSection {
    pub push: SectionPushModel,
    pub pull: SectionPullModel,
    // pub url: String,
    // pub retries_connection: Option<u32>,
    // #[serde(default)]
    // pub get_params: Vec<ParamField>,
    // #[serde(default)]
    // pub http_headers: Vec<HeaderField>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SectionPullModel {
    pub route: String,
    pub server_hostname: String,
    pub port: u32,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SectionPushModel {
    pub url: String,
    pub retries_connection: Option<u32>,
    pub send_interval: u64,
    // #[serde(default)]
    // pub url_params: Vec<ParamField>,
    // #[serde(default)]
    // pub http_headers: Vec<HeaderField>,
    #[serde(default)]
    pub http_headers: HttpHeaders,
    #[serde(default)]
    pub url_params: UrlParams,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommonMetricSetting {
    pub enabled: bool,
    pub override_interval: Option<u32>,
    pub override_retries_connection: Option<u32>,
}
