use persona_exporter_types::*;
use std::path::Path;
use sysinfo::{Components, Disks, Networks, System};

pub fn collect_cpus_metrics(sys: &mut System) -> CpuInfo {
    let physical_core_count = System::physical_core_count();

    CpuInfo {
        cpu_usage: sys.global_cpu_usage(),
        threads: sys.cpus().len(),
        physical_core_count: physical_core_count.unwrap_or(0),
    }
}

pub fn collect_system_metrics(sys: &mut System) -> SystemInfo {
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();
    let free_swap = sys.free_swap();
    let available_memory = sys.available_memory();

    let load_avg = System::load_average();
    let load_avg = LoadAverage {
        one: load_avg.one as f32,
        five: load_avg.five as f32,
        fifteen: load_avg.fifteen as f32,
    };

    SystemInfo {
        total_memory,
        used_memory,
        total_swap,
        used_swap,
        free_swap,
        available_memory,
        load_avg,
    }
}

pub fn collect_disk_metrics(disks: &mut Disks, mount_point: &str) -> DiskInfo {
    let disk = disks
        .iter_mut()
        .find(|disk| disk.mount_point() == Path::new(&mount_point));

    if let Some(disk) = disk {
        let total_space = disk.total_space();
        let available_space = disk.available_space();

        return DiskInfo {
            name: disk.name().to_string_lossy().to_string(),
            file_system: disk.file_system().to_string_lossy().to_string(),
            kind: disk.kind().to_string(),
            total_space,
            available_space,
        };
    }

    DiskInfo {
        name: String::from("unknown"),
        ..Default::default()
    }
}

pub fn collect_network_metrics(networks: &mut Networks) -> NetworkInfo {
    let main_interface = networks
        .iter()
        .filter(|(name, _)| {
            let n = name.as_str();
            n != "lo" && !n.starts_with("br-") && !n.starts_with("docker") && !n.starts_with("veth")
        })
        .max_by_key(|(_, data)| data.total_received() + data.total_transmitted())
        .map(|(name, _)| name.clone());

    if let Some(interface) = main_interface {
        if let Some(data) = networks.get(&interface) {
            // println!("{}", data.)
            return NetworkInfo {
                interface_name: interface,
                total_rx_bytes: data.total_received(),
                total_tx_bytes: data.total_transmitted(),
                total_rx_packets: data.total_packets_received(),
                total_tx_packets: data.total_packets_transmitted(),
                total_rx_errors: data.total_errors_on_received(),
                total_tx_errors: data.total_errors_on_transmitted(),
            };
        }
    }

    NetworkInfo {
        interface_name: "unknown".to_string(),
        ..Default::default()
    }
}

pub fn collect_components_metrics(components: &mut Components) -> ComponentsInfo {
    let components_info = components
        .iter()
        .map(|c| ComponentInfo {
            id: c.id().unwrap_or("unknown").to_string(),
            name: c.label().to_string(),
            temp: c.temperature().unwrap_or(0.0),
            critical_temp: c.critical().unwrap_or(0.0),
            max_temp: c.max().unwrap_or(0.0),
        })
        .collect();

    ComponentsInfo {
        count: components.len(),
        is_empty: components.is_empty(),
        components: components_info,
    }
}
