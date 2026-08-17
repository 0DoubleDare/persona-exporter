use crate::config::{AgentConfigFile, DataType};
use crate::metrics::*;
use crate::platforms::other::methods::{
    ToLineProtocolArgument, build_request_body, collect_metrics_as_line_protocol, send_request,
};
use persona_exporter_types::metrics::ServerMetrics;
use reqwest::Client;
use std::env;
use std::time::{Duration, SystemTime};
use sysinfo::{Components, Disks, Networks};
use tokio::time::sleep;
use tracing::info;

pub async fn collect_metrics_for_os() {
    let debug_mode: bool = env::var("DEBUG")
        .unwrap_or_else(|_| "true".to_string())
        .parse()
        .unwrap_or(true);

    tracing_subscriber::fmt()
        .with_env_filter(if debug_mode { "debug" } else { "info" })
        .init();
    let config: AgentConfigFile = match AgentConfigFile::new() {
        Ok(value) => {
            info!("Config file parsed successfully",);
            value
        }
        Err(err) => {
            panic!(
                r#"
                Something went wrong while loading the configuration.
                Your environment variables or configuration file is invalid.
                Error: {err}
                "#
            );
        }
    };

    info!("Exporter initialized");
    let additional_headers = &config.server.additional_headers;
    let get_params = &config.server.additional_get_params;
    let target_url = &config.server.url;

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let mut sys = (config.metrics.cpu.settings.enabled || config.metrics.memory.settings.enabled)
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

    let await_sec = config.agent.send_metrics_interval;
    info!("Starting persona-exporter");

    loop {
        info!("Collect metrics...");
        let (mem_info, cpu_info, sys_info) = if let Some(ref mut s) = sys {
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
                config.metrics.cpu.settings.enabled.then(|| {
                    collect_system_metrics(
                        s,
                        &config.metrics.processes.sort_by,
                        config.metrics.processes.process_limit,
                    )
                }),
            )
        } else {
            (None, None, None)
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

        if config.agent.data_type == DataType::LineProtocol {
            let time = time as i64;
            let sys = sys_info.unwrap_or_default();
            let argument = ToLineProtocolArgument {
                time,
                system: &sys,
                memory: &mem_info.unwrap_or_default(),
                disk: &disk_info.unwrap_or_default(),
                network: &network_info.unwrap_or_default(),
                cpu: &cpu_info.unwrap_or_default(),
                components: &components_info.unwrap_or_default(),
                processes_info: &sys.processes,
            };
            let total_line = collect_metrics_as_line_protocol(argument);
            info!(
                "Sending data to a specified URL: {:#?}",
                String::from_utf8(total_line.to_vec())
            );

            let request = build_request_body(&client, target_url, get_params, additional_headers)
                .body(total_line);

            send_request(request).await;
        } else {
            let machine_metrics = ServerMetrics {
                system: sys_info,
                memory: mem_info,
                disk: disk_info,
                network: network_info,
                cpu: cpu_info,
                components: components_info,
                time: time as u64,
            };

            info!("Config: {:#?}", &config);
            info!("Machine metrics: {:#?}", machine_metrics);

            let request = build_request_body(&client, target_url, get_params, additional_headers)
                .json(&machine_metrics);
            info!("Sending data to a specified URL: {:?}", request);

            send_request(request).await;
        }

        info!("Next metrics before {} seconds", await_sec);
        sleep(Duration::from_secs(await_sec)).await;
    }
}
