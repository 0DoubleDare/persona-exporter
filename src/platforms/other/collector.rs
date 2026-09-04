use crate::config::{AgentConfigFile, DataType};
use crate::metrics::*;
use crate::platforms::other::methods::arguments::RequestBodyOptions;
use crate::platforms::other::methods::{
    ToLineProtocolOptions, build_request_body, collect_metrics_as_line_protocol, get_host, send_request,
};
use persona_exporter_types::metrics::{ServerMetrics, SystemInfo};
use std::time::{Duration, SystemTime};
use surf::{Client, RequestBuilder};
use sysinfo::{Components, Disks, Networks};
use tracing::info;
use url::Url;

pub async fn collect_metrics_for_os(config: AgentConfigFile) {
    // let debug_mode: bool = env::var("DEBUG")
    //     .unwrap_or_else(|_| "true".to_string())
    //     .parse()
    //     .unwrap_or(true);
    //
    // initial_tracing(debug_mode);
    //
    // let config = load_config();
    //
    info!("Exporter initialized");
    let additional_headers = &config.server.push.http_headers;
    let get_params = &config.server.push.url_params;
    let target_url = &config.server.push.url;
    let await_sec = config.agent.send_interval;
    let system_info_container = SystemInfo::default();
    // let mut connection_pool = HttpConnectionPool::default();
    // connection_pool.set_keep_alive_duration(Duration::from_secs(1));
    // connection_pool.set_max_idle_connections(0);

    // let client = deboa_smol::Client::builder()
    //     .connection_pool(connection_pool)
    //     .build();

    let client: Client = surf::Config::new()
        .set_base_url(Url::parse(target_url).unwrap())
        .try_into().unwrap();


    let request_options = RequestBodyOptions {
        client,
        url: target_url.clone(),
        host: get_host(target_url),
        get_params: get_params.clone(),
        headers: additional_headers.clone(),
    };

    info!("Starting persona-exporter");
    let collect_metrics_handle = std::thread::spawn(move || {
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

        let mut line_protocol_buffer: String;
        let mut metrics: ServerMetrics;

        loop {
            info!("Collect metrics...");

            let (memory_info, cpu_info, system_info, process_list_info) = if let Some(ref mut s) = sys {
                s.refresh_all();
                (
                    config
                        .metrics
                        .memory
                        .settings
                        .enabled
                        .then(|| collect_memory_metrics(s)),
                    config
                        .metrics
                        .cpu
                        .settings
                        .enabled
                        .then(|| collect_cpus_metrics(s)),
                    config
                        .metrics
                        .cpu
                        .settings
                        .enabled
                        .then(collect_system_metrics),
                    config.metrics.processes.settings.enabled.then(|| {
                        collect_process_list_info(
                            s,
                            &config.metrics.processes.sort_by,
                            config.metrics.processes.process_limit,
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
                .as_nanos() as u64;

            let mut request: RequestBuilder = build_request_body(&request_options);

            match config.agent.data_type {
                DataType::LineProtocol => {
                    let time = time as i64;
                    let sys_info = system_info.unwrap_or_default();
                    let argument_for_line_builder = ToLineProtocolOptions {
                        time,
                        system: &sys_info,
                        memory: &memory_info.unwrap_or_default(),
                        disk: &disk_info.unwrap_or_default(),
                        network: &network_info.unwrap_or_default(),
                        cpu: &cpu_info.unwrap_or_default(),
                        components: &components_info.unwrap_or_default(),
                        processes_info: &process_list_info.unwrap_or_default(),
                    };
                    let total_line = collect_metrics_as_line_protocol(argument_for_line_builder);
                    line_protocol_buffer = String::from_utf8(total_line.to_vec()).unwrap_or_default();
                    info!("Sending data to a specified URL: {:#?}", line_protocol_buffer);

                    request = request
                        // .header(http::header::CONTENT_LENGTH.as_str(), &line_protocol_buffer.len().to_string())
                        .body_bytes(line_protocol_buffer);
                }
                DataType::Json => {
                    metrics = ServerMetrics {
                        system: system_info,
                        process_list: process_list_info,
                        memory: memory_info,
                        disk: disk_info,
                        network: network_info,
                        cpu: cpu_info,
                        components: components_info,
                        time,
                    };

                    info!("Machine metrics: {:#?}", metrics);

                    // let json_metrics = serde_json::to_string(&machine_metrics).expect("Failed to serialize to json");
                    let json_body = serde_json::json!(metrics);

                    request = request
                        .body_json(&json_body)
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

    collect_metrics_handle.join().unwrap();
}
