use crate::config::DataType;
use crate::metrics::*;
use crate::platforms::other::methods::types::RequestBodyOptions;
use crate::platforms::other::methods::{
    ToLineProtocolOptions, build_request_body, collect_metrics_as_line_protocol, get_host_from_url,
    initial_tracing, load_config_file, send_request,
};
use deboa::request::DeboaRequestBuilder;
use hyper_body_utils::HttpBody;
use persona_exporter_types::metrics::ServerMetrics;
use std::env;
use std::time::{Duration, SystemTime};
use sysinfo::{Components, Disks, Networks};
use tracing::info;

pub async fn collect_metrics_for_os() {
    let debug_mode: bool = env::var("DEBUG")
        .unwrap_or_else(|_| "true".to_string())
        .parse()
        .unwrap_or(true);

    initial_tracing(debug_mode);

    let config = load_config_file();

    info!("Exporter initialized");
    let additional_headers = &config.server.http_headers;
    let get_params = &config.server.get_params;
    let target_url = &config.server.url;
    let await_sec = config.agent.send_interval;

    let mut send_collection: String = String::from("");

    let client = deboa_smol::Client::default();

    let request_options = RequestBodyOptions {
        client,
        url: target_url.clone(),
        host: get_host_from_url(target_url),
        get_params: get_params.clone(),
        headers: additional_headers.clone(),
    };

    info!("Starting persona-exporter");
    let collect_metrics_thread = std::thread::spawn(move || {
        let mut sys = (config.metrics.cpu.settings.enabled
            || config.metrics.memory.settings.enabled)
            .then(sysinfo::System::new);
        let mut disks = config
            .metrics
            .disks
            .settings
            .enabled
            .then(Disks::new_with_refreshed_list);
        let mut networks = config
            .metrics
            .network
            .settings
            .enabled
            .then(Networks::new_with_refreshed_list);
        let mut components = config
            .metrics
            .components
            .settings
            .enabled
            .then(Components::new_with_refreshed_list);

        loop {
            info!("Collect metrics...");

            let config_clone = config.clone();
            let (mem_info, cpu_info, sys_info, process_list_info) = if let Some(ref mut s) = sys {
                s.refresh_all();
                (
                    config_clone
                        .metrics
                        .memory
                        .settings
                        .enabled
                        .then(|| collect_memory_metrics(s)),
                    config_clone
                        .metrics
                        .cpu
                        .settings
                        .enabled
                        .then(|| collect_cpus_metrics(s)),
                    config_clone
                        .metrics
                        .cpu
                        .settings
                        .enabled
                        .then(collect_system_metrics),
                    config_clone.metrics.processes.settings.enabled.then(|| {
                        collect_process_list_info(
                            s,
                            &config_clone.metrics.processes.sort_by,
                            config_clone.metrics.processes.process_limit,
                        )
                    }),
                )
            } else {
                (None, None, None, None)
            };

            let disk_info = disks.as_mut().map(|d| {
                d.refresh(false);
                collect_disk_metrics(d, "/")
            });

            let network_info = networks.as_mut().map(|n| {
                n.refresh(false);
                collect_network_metrics(n)
            });

            let components_info = components.as_mut().map(|c| {
                c.refresh(false);
                collect_components_metrics(c)
            });

            let time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos();

            let mut request: DeboaRequestBuilder = build_request_body(&request_options);

            match config.agent.data_type {
                DataType::LineProtocol => {
                    send_collection.clear();

                    let time = time as i64;
                    let sys_info = sys_info.unwrap_or_default();
                    let argument_for_line_builder = ToLineProtocolOptions {
                        time,
                        system: &sys_info,
                        memory: &mem_info.unwrap_or_default(),
                        disk: &disk_info.unwrap_or_default(),
                        network: &network_info.unwrap_or_default(),
                        cpu: &cpu_info.unwrap_or_default(),
                        components: &components_info.unwrap_or_default(),
                        processes_info: &process_list_info.unwrap_or_default(),
                    };
                    let total_line = collect_metrics_as_line_protocol(argument_for_line_builder);
                    let total_line = String::from_utf8(total_line.to_vec()).unwrap_or_default();
                    info!("Sending data to a specified URL: {:#?}", total_line);

                    request = request
                        .header(http::header::CONTENT_LENGTH, &total_line.len().to_string())
                        .body(HttpBody::from(total_line));
                }
                DataType::Json => {
                    let machine_metrics = ServerMetrics {
                        system: sys_info,
                        process_list: process_list_info,
                        memory: mem_info,
                        disk: disk_info,
                        network: network_info,
                        cpu: cpu_info,
                        components: components_info,
                        time: time as u64,
                    };

                    info!("Config: {:#?}", &config);
                    info!("Machine metrics: {:#?}", machine_metrics);

                    // let json_metrics = serde_json::to_string(&machine_metrics).expect("Failed to serialize to json");
                    let body = serde_json::json!(machine_metrics);

                    request = request
                        .body_as(deboa_extras::serde::json::JsonBody, body)
                        .expect("Failed to create request body");
                }
            }

            smol::block_on(async {
                send_request(request, &request_options.client).await;

                info!("Next metrics before {} seconds", await_sec);
                smol::Timer::after(Duration::from_secs(await_sec)).await;
            });
        }
    });

    collect_metrics_thread.join().unwrap();
}
