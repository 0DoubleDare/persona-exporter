use std::env;
use std::time::{Duration, SystemTime};
use sysinfo::{Components, Disks, Networks};
use tokio::time::sleep;
// use paas_core::structures::server_metrics::ServerMetrics;
use persona_exporter::config::AgentConfigFile;
use persona_exporter::metrics::*;
use persona_exporter_types::{ConvertTo, DataUnit, ServerMetrics};

#[tokio::main]
async fn main() {
    let debug_mode: bool = env::var("DEBUG_MODE")
        .unwrap_or_else(|_| "false".to_string())
        .parse()
        .unwrap_or(false);
    tracing_subscriber::fmt()
        .with_env_filter(if debug_mode { "debug" } else { "info" })
        .init();
    println!("Open app");
    let config = loop {
        match AgentConfigFile::new() {
            Ok(value) => {
                tracing::info!("Config file parsed successfully");
                break value;
            }
            Err(err) => {
                tracing::error!("Failed to parse config file: {}", err);
                tracing::warn!("Wait 10sec to reload...");
                sleep(Duration::from_secs(10)).await;
            }
        };
    };

    tracing::info!("Exporter initialized");
    let auth_token = config.server.server_key;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();
    let mut sys = (config.metrics.cpu.enabled || config.metrics.memory.enabled)
        .then(|| sysinfo::System::new());
    let mut disks = config
        .metrics
        .disks
        .enabled
        .then(|| Disks::new_with_refreshed_list());
    let mut networks = config
        .metrics
        .network
        .enabled
        .then(|| Networks::new_with_refreshed_list());
    let mut components = config
        .metrics
        .components
        .enabled
        .then(|| Components::new_with_refreshed_list());

    let await_sec = config.agent.send_metrics_interval;
    tracing::info!("Starting persona-exporter");

    loop {
        tracing::info!("Collect metrics...");

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
        let sys_info = config
            .metrics
            .system
            .enabled
            .then(|| collect_system_metrics());

        let machine_metrics = ServerMetrics {
            system: sys_info,
            memory: mem_info,
            disk: disk_info,
            network: network_info,
            cpu: cpu_info,
            components: components_info,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };

        tracing::info!("Machine metrics: {:#?}", machine_metrics);
        let response = client
            .post(&config.server.server_url)
            .bearer_auth(&auth_token)
            .json(&machine_metrics)
            .send()
            .await;
        tracing::info!("Sending data to a specified URL");
        match response {
            Ok(response) => {
                tracing::info!("Response from the server: {}", response.status())
            }
            Err(err) => {
                tracing::error!("Send error: {}", err);
            }
        }

        for s in (1..=await_sec).rev() {
            tracing::info!("Await {} seconds", s);
            sleep(Duration::from_secs(1)).await;
        }
    }
}
