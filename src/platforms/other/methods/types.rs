use crate::config::{HeaderField, ParamField};
use persona_exporter_types::metrics::{
    ComponentsInfo, CpuInfo, DiskInfo, MemoryInfo, NetworkInfo, ProcessListInfo, SystemInfo,
};
use deboa_smol::Client;

pub struct ToLineProtocolOptions<'a> {
    pub time: i64,
    pub system: &'a SystemInfo,
    pub memory: &'a MemoryInfo,
    pub disk: &'a DiskInfo,
    pub network: &'a NetworkInfo,
    pub cpu: &'a CpuInfo,
    pub components: &'a ComponentsInfo,
    pub processes_info: &'a ProcessListInfo,
}

pub struct RequestBodyOptions {
    pub client: Client,
    pub url: String,
    pub get_params: Vec<ParamField>,
    pub headers: Vec<HeaderField>,
}
