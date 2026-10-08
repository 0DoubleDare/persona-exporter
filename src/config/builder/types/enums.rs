use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SendModel {
    Pull,
    #[default]
    Push,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub enum SortDirection {
    #[default]
    Desc,
    Asc,
}
