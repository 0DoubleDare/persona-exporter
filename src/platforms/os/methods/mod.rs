pub mod arguments;
pub mod metrics;
pub mod context;

use crate::config::AgentConfigFile;
use crate::platforms::os::methods::arguments::RequestBodyOptions;
use influxdb_line_protocol::LineProtocolBuilder;
// use persona_exporter_types::traits::line_protocol::{FromWithMeasurement, IntoWithMeasurement};
use compact_str::CompactString;
use config::ConfigError;
use persona_exporter_types::metrics::line_protocol::GlobalTags;
use persona_exporter_types::metrics::structs::components::ComponentListInfo;
use persona_exporter_types::metrics::structs::cpu::CpuListInfo;
use persona_exporter_types::metrics::structs::disk::StorageListInfo;
use persona_exporter_types::metrics::structs::memory::MemoryInfo;
use persona_exporter_types::metrics::structs::network::NetworkInfo;
use persona_exporter_types::metrics::structs::processes::{ProcessInfo, ProcessListInfo};
use persona_exporter_types::metrics::structs::server::ServerMetrics;
use persona_exporter_types::metrics::structs::system::SystemInfo;
use persona_exporter_types::traits::line_protocol::{FinishLineProtocol, FromWithMeasurement};
use tracing::{Level, debug, error, info, warn};
use ureq::AsSendBody;
use ureq::typestate::WithBody;

pub fn collect_metrics_as_line_protocol(
    metrics: &ServerMetrics,
    line_buffer: &mut Vec<u8>,
    global_tags: &GlobalTags,
) {
    let time = metrics.time;
    if let Some(ref system) = metrics.system {
        line_buffer.extend_from_slice(
            LineProtocolBuilder::from_with_name(system, "metrics_system", global_tags)
                .finish(time)
                .as_slice(),
        );
    }
    if let Some(ref disk) = metrics.disk {
        for mount_point in disk.storage_list.iter() {
            line_buffer.extend_from_slice(
                LineProtocolBuilder::from_with_name(
                    mount_point,
                    "metrics_storage_mount_point",
                    global_tags,
                )
                .finish(time)
                .as_slice(),
            );
        }
    }
    if let Some(ref network) = metrics.network {
        line_buffer.extend_from_slice(
            LineProtocolBuilder::from_with_name(network, "metrics_network", global_tags)
                .finish(time)
                .as_slice(),
        );
    }
    if let Some(ref cpu) = metrics.cpu {
        line_buffer.extend_from_slice(
            LineProtocolBuilder::from_with_name(cpu, "metrics_cpu", global_tags)
                .finish(time)
                .as_slice(),
        );
        cpu.cpu_cores.iter().for_each(|cpu_core| {
            line_buffer.extend_from_slice(
                LineProtocolBuilder::from_with_name(cpu_core, "metrics_cpu_cores", global_tags)
                    .finish(time)
                    .as_slice(),
            );
        });
    }
    if let Some(ref memory) = metrics.memory {
        line_buffer.extend_from_slice(
            LineProtocolBuilder::from_with_name(memory, "metrics_memory", global_tags)
                .finish(time)
                .as_slice(),
        );
    }

    if let Some(ref component_list) = metrics.components {
        component_list.components.iter().for_each(|c| {
            line_buffer.extend_from_slice(
                LineProtocolBuilder::from_with_name(c, "metrics_component_list", global_tags)
                    .finish(time)
                    .as_slice(),
            );
        });
    }
    if let Some(ref processes) = metrics.process_list {
        processes.process_list.iter().for_each(|p| {
            line_buffer.extend_from_slice(
                LineProtocolBuilder::from_with_name(p, "metrics_process_list", global_tags)
                    .finish(time)
                    .as_slice(),
            );
        });
        if let Some(ref self_metrics) = processes.exporter_metrics {
            line_buffer.extend_from_slice(
                LineProtocolBuilder::from_with_name(
                    self_metrics,
                    "metrics_exporter_monitoring",
                    global_tags,
                )
                .finish(time)
                .as_slice(),
            );
        }
        // processes.exporter_metrics.iter().for_each(|self_process| {
        //     processes.exporter_metrics.extend_from_slice(
        //         LineProtocolBuilder::from_with_name(self_process, "metrics_exporter_monitoring")
        //             .finish(time)
        //             .as_slice(),
        //     );
        // });
    }
}

pub fn build_request_body(options: &RequestBodyOptions) -> ureq::RequestBuilder<WithBody> {
    let mut request = options
        .config
        .new_agent()
        .post(options.url)
        .query_pairs(options.get_params);

    for (header, value) in options.headers {
        request = request.header(header, value);
    }

    debug!("Result of build request: {:#?}", request);

    request
}

pub async fn send_request<T: AsSendBody>(request: ureq::RequestBuilder<WithBody>, data: T) {
    debug!("{:#?}", request);
    let response = request.send(data);

    match response {
        Ok(response) => {
            let response_status = response.status();
            let reason = response_status
                .canonical_reason()
                .unwrap_or("unknown_reason");

            debug!("{:#?}", response);

            match response_status.as_u16() {
                100..400 => {
                    info!("Success response: {} {}", response_status, reason);
                }
                400..500 => {
                    error!("Client side error: {} {}", response_status, reason);
                }
                500..600 => {
                    error!("Server side error: {} {}", response_status, reason);
                }
                _ => {
                    warn!("Unknown status code: {} {}", response_status, reason);
                }
            }
        }
        Err(err) => {
            error!("Send error: {}", err)
        }
    }
}

pub fn load_config(
    override_config_path: Option<CompactString>,
) -> Result<AgentConfigFile, ConfigError> {
    AgentConfigFile::new_with(override_config_path)
}

pub fn initial_tracing(log_level: u8) {
    let level = match log_level {
        1 => Level::INFO,
        2 => Level::DEBUG,
        3 => Level::TRACE,
        _ => Level::WARN,
    };

    tracing_subscriber::fmt()
        .compact()
        .without_time()
        .with_target(false)
        .with_max_level(level)
        .init();
}

// pub fn get_host(url: &str) -> String {
//     if let Ok(parsed_url) = Url::parse(url) {
//         return parsed_url.host_str().unwrap_or("localhost").to_string();
//     };
//     "incorrect_url".to_string()
// }

pub fn create_metrics_struct_by_config(config: &AgentConfigFile) -> ServerMetrics {
    ServerMetrics {
        system: config
            .metrics
            .system
            .settings
            .enabled
            .then(SystemInfo::default),
        process_list: config
            .metrics
            .processes
            .settings
            .enabled
            .then(|| ProcessListInfo {
                exporter_metrics: config
                    .metrics
                    .processes
                    .include_exporter_metrics
                    .then(ProcessInfo::default),
                process_list: Vec::new(),
            }),
        memory: config
            .metrics
            .memory
            .settings
            .enabled
            .then(MemoryInfo::default),
        disk: config
            .metrics
            .disks
            .settings
            .enabled
            .then(StorageListInfo::default),
        network: config
            .metrics
            .network
            .settings
            .enabled
            .then(NetworkInfo::default),
        cpu: config
            .metrics
            .cpu
            .settings
            .enabled
            .then(CpuListInfo::default),
        components: config
            .metrics
            .components
            .settings
            .enabled
            .then(ComponentListInfo::default),
        time: 0,
    }
}
