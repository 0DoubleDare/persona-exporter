use crate::config::{AgentConfigFile, HttpHeaders, ProcessSortBy, UrlParams};
use persona_exporter_types::metrics::structs::components::ComponentListInfo;
use persona_exporter_types::metrics::structs::cpu::CpuListInfo;
use persona_exporter_types::metrics::structs::disk::StorageListInfo;
use persona_exporter_types::metrics::structs::memory::MemoryInfo;
use persona_exporter_types::metrics::structs::network::NetworkInfo;
use persona_exporter_types::metrics::structs::processes::ProcessListInfo;
use persona_exporter_types::metrics::structs::server::ServerMetrics;
use persona_exporter_types::metrics::structs::system::SystemInfo;
use sysinfo::{CpuRefreshKind, DiskRefreshKind, MemoryRefreshKind, ProcessRefreshKind, UpdateKind};
use ureq::config::Config;

#[derive(Default)]
pub struct ToLineProtocolOptions {
    // pub time: i64,
    pub system: SystemInfo,
    pub memory: MemoryInfo,
    pub disk: StorageListInfo,
    pub network: NetworkInfo,
    pub cpu: CpuListInfo,
    pub components: ComponentListInfo,
    pub processes_info: ProcessListInfo,
}

pub struct RequestBodyOptions<'a> {
    pub config: &'a Config,
    pub url: &'a String,
    pub get_params: &'a UrlParams,
    pub headers: &'a HttpHeaders,
}

pub struct CollectProcessListOptions {
    pub sort_by: ProcessSortBy,
    pub cpu_cores: usize,
    pub process_limit: usize,
}

#[derive(Default)]
pub struct LineProtocolBuffer {
    pub line_protocol_buffer: Vec<u8>,
    pub line_protocol_options: ToLineProtocolOptions,
}

#[derive(Default)]
pub struct Buffers {
    pub metrics: ServerMetrics,
    pub line_protocol: LineProtocolBuffer,
}

pub struct CommonMetricsInformation {
    pub components: Option<sysinfo::Components>,
    pub networks: Option<sysinfo::Networks>,
    pub disks: Option<sysinfo::Disks>,
    pub system: Option<sysinfo::System>,
}

pub struct CommonConfigurations {
    pub memory_is_enabled: bool,
    pub system_is_enabled: bool,
    pub components_is_enabled: bool,
    pub cpu_is_enabled: bool,
    pub processes_is_enabled: bool,
    pub networks_is_enabled: bool,
    pub disks_is_enabled: bool,

    pub cpu_physical_core_count: usize,
    pub process_list_limit: usize,
}

#[derive(Default)]
pub struct RefreshKindContext {
    pub process_refresh_kind: Option<ProcessRefreshKind>,
    pub disk_refresh_kind: Option<DiskRefreshKind>,
    pub memory_refresh_kind: Option<MemoryRefreshKind>,
    pub cpu_refresh_kind: Option<CpuRefreshKind>,
}

#[derive(Default)]
pub struct SystemContext {
    pub system_snapshot: sysinfo::System,
    pub refresh_kinds: RefreshKindContext,
}

impl RefreshKindContext {
    pub fn new(config: &AgentConfigFile) -> Self {
        let cfg = &config.metrics;

        let process_refresh_kind = cfg.processes.settings.enabled.then(|| {
            ProcessRefreshKind::nothing()
                .with_user(UpdateKind::OnlyIfNotSet)
                .with_memory()
                .with_cpu()
                .with_disk_usage()
        });

        RefreshKindContext {
            process_refresh_kind,
            disk_refresh_kind: None,
            memory_refresh_kind: None,
            cpu_refresh_kind: None,
        }
    }
}
