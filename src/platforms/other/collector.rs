use crate::configs::{AgentConfigFile, DataType, HeaderConfig};
use crate::metrics::*;
use influxdb_line_protocol::{LineProtocolBuilder, builder::AfterField};
use persona_exporter_types::default::ServerMetrics;
use reqwest::RequestBuilder;
use reqwest::header::{HeaderName, HeaderValue};
use std::env;
use std::time::{Duration, SystemTime};
use sysinfo::{Components, Disks, Networks};
use tokio::time::sleep;
use tracing::{debug, error, info};

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
            info!("Config file parsed successfully");
            value
        }
        Err(err) => {
            panic!(
                r#"
                Something went wrong while loading the configuration.
                Your environment variables and configuration file are invalid.
                Error: {err}
                "#
            );
        }
    };

    info!("Exporter initialized");
    let auth_token = config.server.bearer_token;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();
    let mut sys =
        (config.metrics.cpu.enabled || config.metrics.memory.enabled).then(sysinfo::System::new);
    let mut disks = config
        .metrics
        .disks
        .enabled
        .then(Disks::new_with_refreshed_list);
    let mut networks = config
        .metrics
        .network
        .enabled
        .then(Networks::new_with_refreshed_list);
    let mut components = config
        .metrics
        .components
        .enabled
        .then(Components::new_with_refreshed_list);

    let await_sec = config.agent.send_metrics_interval;
    info!("Starting persona-exporter");

    loop {
        info!("Collect metrics...");

        let (mem_info, cpu_info) = if let Some(ref mut s) = sys {
            s.refresh_all();
            (
                config
                    .metrics
                    .memory
                    .enabled
                    .then(|| collect_memory_metrics(s)),
                config.metrics.cpu.enabled.then(|| collect_cpus_metrics(s)),
            )
        } else {
            (None, None)
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
        let sys_info = config.metrics.system.enabled.then(collect_system_metrics);

        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        if config.agent.data_type == DataType::LineProtocol {
            let time = time as i64;
            let line_sys = LineProtocolBuilder::from(sys_info.unwrap_or_default())
                .timestamp(time)
                .close_line()
                .build();
            let line_mem = LineProtocolBuilder::from(mem_info.unwrap_or_default())
                .timestamp(time)
                .close_line()
                .build();
            let line_disk = LineProtocolBuilder::from(disk_info.unwrap_or_default())
                .timestamp(time)
                .close_line()
                .build();
            let line_network = LineProtocolBuilder::from(network_info.unwrap_or_default())
                .timestamp(time)
                .close_line()
                .build();
            let line_cpu = LineProtocolBuilder::from(cpu_info.unwrap_or_default())
                .timestamp(time)
                .close_line()
                .build();

            let mut line_components: Vec<u8> = vec![];

            for line in components_info.unwrap_or_default().components {
                let mut l = LineProtocolBuilder::from(line)
                    .timestamp(time)
                    .close_line()
                    .build();
                line_components.append(&mut l);
            }

            let total_line = vec![
                line_sys.as_slice(),
                line_mem.as_slice(),
                line_disk.as_slice(),
                line_network.as_slice(),
                line_cpu.as_slice(),
                line_components.as_slice(),
            ]
            .concat();
            info!("Sending data to a specified URL: {:?}", total_line);

            let mut response_builder = client
                .post(&config.server.url)
                .header("Authorization", "Token d2Iv9eGcgUhljFmRl7zk_3ryWNGD5VclKkXIY9UYcXbfUW98BdWsOQENN9sFxDN6zYEDQ9WKPrQa4Uhetdr5Nw==")
                .header("Content-Type", "text/plain; charset=utf-8")
                .bearer_auth(&auth_token)
                .body(total_line);

            response_builder = insert_headers_to_request_builder(
                response_builder,
                config.server.additional_headers.clone(),
            );

            send(response_builder).await;
        } else {
            let machine_metrics = ServerMetrics {
                system: sys_info,
                memory: mem_info,
                disk: disk_info,
                network: network_info,
                cpu: cpu_info,
                components: components_info,
                load_average: None,
                time: time as u64,
            };

            info!("Machine metrics: {:#?}", machine_metrics);

            let response_builder = client
                .post(&config.server.url)
                .header("Authorization", "Token")
                .bearer_auth(&auth_token)
                .json(&machine_metrics);
            info!("Sending data to a specified URL");

            send(response_builder).await;
        }

        info!("Next metrics before {} seconds", await_sec);
        sleep(Duration::from_secs(await_sec)).await;
    }
}

pub async fn send(request: RequestBuilder) {
    let response = request.send().await;

    match response {
        Ok(response) => {
            let response_status = response.status();
            let status_code_type = response_status.as_u16() / 100;
            match status_code_type {
                4 => {
                    error!("What is wrong on the client side: {}", response_status);
                }
                5 => {
                    error!("What is wrong on the server side: {}", response_status);
                }
                _ => {
                    info!("Positive server response: {}", response_status);
                }
            }
            debug!("{:#?}", response);
            debug!(
                "{}",
                response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Body JSON is empty".to_string())
            )
        }
        Err(err) => {
            error!("Send error: {}", err)
        }
    }
}

pub fn insert_headers_to_request_builder(
    mut request: RequestBuilder,
    headers: Vec<HeaderConfig>,
) -> RequestBuilder {
    for header in headers {
        request = request.header(header.header, header.value);
    }
    request
}
