use crate::config::{AgentConfigFile, HttpHeaders, MetricsConfig, ProcessSortBy, UrlParams};
use persona_exporter_types::metrics::line_protocol::GlobalTags;
use persona_exporter_types::metrics::structs::components::ComponentListInfo;
use persona_exporter_types::metrics::structs::cpu::CpuListInfo;
use persona_exporter_types::metrics::structs::disk::StorageListInfo;
use persona_exporter_types::metrics::structs::memory::MemoryInfo;
use persona_exporter_types::metrics::structs::network::NetworkInfo;
use persona_exporter_types::metrics::structs::processes::{ProcessInfo, ProcessListInfo};
use persona_exporter_types::metrics::structs::server::ServerMetrics;
use persona_exporter_types::metrics::structs::system::SystemInfo;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use sysinfo::{
    Components, CpuRefreshKind, DiskRefreshKind, Disks, MemoryRefreshKind, Networks,
    ProcessRefreshKind, System, UpdateKind,
};
use ureq::config::Config;

/// Since a trait signature cannot be used directly as a field type,
/// it is necessary to use a generic type parameter `F` that implements
/// that signature; consequently, any struct that includes a field of
/// this generic type `F` must also specify the type `F`.
pub struct GlobalContext<'a, F>
where
    F: Fn(&ProcessInfo, &ProcessInfo) -> Ordering + 'a,
{
    pub snapshots: MetricsSnapshots,
    pub buffers: Buffers,
    pub variables: ConfigContext<'a, F>,
}

pub struct ConfigContext<'a, F>
where
    F: Fn(&ProcessInfo, &ProcessInfo) -> Ordering + 'a,
{
    pub physical_core_count: usize,
    pub sort_by: F,
    pub process_limit: usize,
    pub global_tags: &'a GlobalTags,
}

impl<'a, F> GlobalContext<'a, F>
where
    F: Fn(&ProcessInfo, &ProcessInfo) -> Ordering + 'a,
{
    pub fn new(
        config: &'a AgentConfigFile,
        sort_closure: F,
        server_metrics: ServerMetrics,
    ) -> Self {
        let sys_context = (config.metrics.cpu.settings.enabled
            || config.metrics.memory.settings.enabled
            || config.metrics.processes.settings.enabled
            || config.metrics.system.settings.enabled)
            .then(|| SystemContext {
                snapshot: System::new(),
                refresh_kinds: RefreshKindContext::new(&config.metrics),
            });
        let disks = config
            .metrics
            .disks
            .settings
            .enabled
            .then(Disks::new_with_refreshed_list);
        let networks = config
            .metrics
            .network
            .settings
            .enabled
            .then(Networks::new_with_refreshed_list);
        let components = config
            .metrics
            .components
            .settings
            .enabled
            .then(Components::new_with_refreshed_list);

        let metrics_snapshots = MetricsSnapshots {
            components,
            networks,
            disks,
            system_context: sys_context,
        };

        GlobalContext {
            snapshots: metrics_snapshots,
            buffers: Buffers {
                metrics: server_metrics,
                ..Buffers::default()
            },
            variables: ConfigContext {
                physical_core_count: System::physical_core_count().unwrap_or(1),
                sort_by: sort_closure,
                process_limit: config.metrics.processes.process_limit,
                global_tags: &config.metrics.global_tags,
            },
        }
    }
}
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

#[derive(Default, Serialize, Deserialize)]
pub struct Buffers {
    pub metrics: ServerMetrics,
    pub line_protocol_buffer: Vec<u8>,
}

pub struct MetricsSnapshots {
    pub components: Option<sysinfo::Components>,
    pub networks: Option<sysinfo::Networks>,
    pub disks: Option<sysinfo::Disks>,
    pub system_context: Option<SystemContext>,
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
    pub snapshot: sysinfo::System,
    pub refresh_kinds: RefreshKindContext,
}

impl RefreshKindContext {
    #[must_use]
    pub fn new(metrics_config: &MetricsConfig) -> Self {
        let process_refresh_kind = metrics_config.processes.settings.enabled.then(|| {
            ProcessRefreshKind::nothing()
                .with_user(UpdateKind::OnlyIfNotSet)
                .with_memory()
                .with_cpu()
                .with_disk_usage()
        });

        Self {
            process_refresh_kind,
            disk_refresh_kind: None,
            memory_refresh_kind: None,
            cpu_refresh_kind: None,
        }
    }
}
