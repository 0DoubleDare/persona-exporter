use std::time::{Duration, SystemTime};
use sysinfo::{Components, Disks, Networks};
use tokio::time::sleep;
// use paas_core::structures::server_metrics::ServerMetrics;
use persona_exporter::config::AgentConfigFile;
use persona_exporter::metrics::*;
use persona_exporter_types::ServerMetrics;

#[tokio::main]
async fn main() {
    let config = loop {
        match AgentConfigFile::new() {
            Ok(value) => {
                tracing::debug!("Config file parsed successfully");
                break value;
            }
            Err(err) => {
                tracing::error!("Failed to parse config file: {}", err);
                tracing::warn!("Wait 10sec to reload...");
                sleep(Duration::from_secs(10)).await;
            }
        };
    };

    tracing_subscriber::fmt()
        .with_env_filter(if config.agent.debug_mode {
            "debug"
        } else {
            "info"
        })
        .init();

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let mut sys = (config.metrics.system.enabled || config.metrics.cpu.enabled)
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

    tracing::info!("Starting persona-exporter");

    // println!("{}", sys.);

    loop {
        // components.expect("REASON").refresh(false);
        let sys_info = sys.as_mut().map(|s| collect_system_metrics(s));
        let disk_info = disks.as_mut().map(|d| collect_disk_metrics(d, "/"));
        let network_info = networks.as_mut().map(|n| collect_network_metrics(n));
        let cpu_info = sys.as_mut().map(|s| collect_cpus_metrics(s));
        let components_info = components.as_mut().map(|c| collect_components_metrics(c));
        // let mut sys_info = if let Some(ref mut system) = sys {
        //     collect_system_metrics(system);
        // };
        // let sys_info = sys.is_some().then(|| collect_system_metrics(&mut sys));
        // if config.metrics.system.enabled {
        //     let sys_info = collect_system_metrics(&mut sys);
        // }
        // let disk_info = collect_disk_metrics(&mut disks, "/");
        // let network_info = collect_network_metrics(&mut networks);
        //
        // let components_info = collect_components_metrics(&mut components);
        // let cpu_info = collect_cpus_metrics(&mut sys, components_info);

        let machine_metrics = ServerMetrics {
            system: sys_info,
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
            .json(&machine_metrics)
            .send()
            .await;
        tracing::info!("Send data to server...");
        match response {
            Ok(response) => {
                tracing::info!("Get server response: {}", response.status())
            }
            Err(err) => {
                tracing::error!("Failed to send webhook: {}", err);
            }
        }

        sleep(Duration::from_secs(config.agent.send_metrics_interval)).await;
    }
}
