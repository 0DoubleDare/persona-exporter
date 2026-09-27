pub mod arguments;
pub mod metrics;

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
use std::collections::BTreeMap;
use surf::post;
use tracing::{Level, debug, error, info};
use url::Url;

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
        // disk.disk_list.iter().for_each(|d| {
        //     line_buffer.extend_from_slice(
        //         LineProtocolBuilder::from_with_name(d, "metrics_disk_info")
        //             .finish(time)
        //             .as_slice()
        //     );
        // });
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

pub fn build_request_body(options: &RequestBodyOptions) -> surf::RequestBuilder {
    let total_url = options.url.clone();

    // if !options.get_params.is_empty() {
    //     let mut query_string = String::new();
    //     let get_url_pairs = form_urlencoded::Serializer::new(&mut query_string);
    //
    //     options
    //         .get_params
    //         .iter()
    //         .fold(get_url_pairs, |mut acc, get_param| {
    //             acc.append_pair(get_param.key.as_str(), get_param.value.as_str());
    //             acc
    //         })
    //         .finish();
    //     total_url = format!("{}?{}", total_url, query_string);
    // }
    // let headers: Vec<(&str, &str)> = options
    //     .headers
    //     .iter()
    //     .map(|h| (h.key.as_str(), h.value.as_str()))
    //     .collect();

    let mut query_params: BTreeMap<String, String> = BTreeMap::new();
    for params in &options.get_params {
        query_params.insert(params.key.clone(), params.value.clone());
    }
    let mut request = post(total_url)
        .query(&query_params)
        .unwrap()
        .header(http::header::HOST.as_str(), &options.host)
        .header(http::header::CONNECTION.as_str(), "close");

    info!("{:#?}", request);
    for header in &options.headers {
        request = request.header(header.key.as_str(), header.value.as_str());
    }

    request
}

pub async fn send_request(request: surf::RequestBuilder, _client: &surf::Client) {
    let response = request.send().await;

    match response {
        Ok(success_response) => {
            let response_status = success_response.status();
            debug!("{:#?}", success_response);
            // info!(
            //     "{}",
            //     success_response.body_string().await.unwrap_or_default()
            // );
            info!(
                "Response status: {} \"{}\"",
                response_status as u16,
                response_status.canonical_reason()
            );
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

pub fn get_host(url: &str) -> String {
    if let Ok(parsed_url) = Url::parse(url) {
        return parsed_url.host_str().unwrap_or("localhost").to_string();
    };
    "incorrect_url".to_string()
}

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
