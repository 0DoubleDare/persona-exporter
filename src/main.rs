use std::alloc::System;
use std::time::{Duration, SystemTime};
use sysinfo::{Disks, Networks, Components};
use tokio::time::sleep;
use paas_core::structures::agent_config::AgentConfigFile;
use paas_core::structures::server_metrics::ServerMetrics;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let mut sys = sysinfo::System::new();
    let mut disks = Disks::new_with_refreshed_list();
    let mut networks = Networks::new_with_refreshed_list();
    let mut components = Components::new_with_refreshed_list();

    tracing::info!("Starting persona-persona-exporter");

    let config = loop {
        match AgentConfigFile::new() {
            Ok(value) => {
                tracing::info!("Config file parsed successfully");
                break value
            },
            Err(err) => {
                tracing::error!("Failed to parse config file: {}", err);
                tracing::warn!("Wait 10sec to reload...");
                sleep(Duration::from_secs(10)).await;
            }
        };
    };

    loop {
        components.refresh(false);

        println!("{:?}", sysinfo::Product::stock_keeping_unit());
        // println!("{}", fdf.total_memory);
        for component in &components {
            println!("{:#?}", component);
        }
        let sys_info = ServerMetrics::collect_system_metrics(&mut sys);

        let disk_info = ServerMetrics::collect_disk_metrics(&mut disks,  "/");

        let network_info = ServerMetrics::collect_network_metrics(&mut networks);

        let machine_metrics = ServerMetrics {
            sys_info,
            disk_info,
            network_data: network_info,
            timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as u64,
        };

        tracing::info!("Machine metrics: {:#?}", machine_metrics);
        sleep(Duration::from_secs(config.agent.send_metrics_interval)).await;
    }
}