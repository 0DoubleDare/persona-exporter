use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentConfigFile {
    pub agent: AgentSection,
    pub server: ServerSection,
    pub metrics: MetricsConfig,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MetricsConfig {
    pub cpu: CpuConfig,
    pub disks: DisksConfig,
    pub network: NetworkConfig,
    pub system: SystemConfig,
    pub components: ComponentsConfig,
    pub memory: MemoryConfig,
    pub processes: ProcessListConfig,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProcessListConfig {
    pub settings: CommonMetricSetting,
    pub process_limit: usize,
    pub sort_by: ProcessSortBy,
    // pub sort_by: [Option<ProcessListSortConfig>; 5],
}
// #[derive(Serialize, Deserialize, Debug, Default, Clone)]
// pub struct ProcessListSortConfig {
//     pub sort_by: ProcessSortBy,
//     pub override_process_limit: Option<usize>,
// }
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
    pub send_interval: u64,
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
    pub host: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SectionPushModel {
    pub url: String,
    pub retries_connection: Option<u32>,
    #[serde(default)]
    pub get_params: Vec<ParamField>,
    #[serde(default)]
    pub http_headers: Vec<HeaderField>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct HeaderField {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ParamField {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommonMetricSetting {
    pub enabled: bool,
    pub override_interval: Option<u32>,
    pub override_retries_connection: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SendModel {
    Pull,
    #[default]
    Push,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DataType {
    #[default]
    Json,
    LineProtocol,
    // OpenMetrics,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ListType {
    WhiteList,
    #[default]
    IgnoreList,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ProcessSortBy {
    #[default]
    CpuUsage,
    Memory,
    VirtualMemory,
    RunTime,
    StartTime,
}
