use std::path::Path;
use sysinfo::{Components, Disks, Networks, System};
use persona_exporter_types::*;

pub fn collect_cpus_metrics(sys: &mut System, comps: &mut Components) -> CpuInfo {
    sys.refresh_all();
    comps.refresh(false);

    let physical_core_count = System::physical_core_count();
    let mut components: Vec<ComponentInfo> = vec![];

    for component in comps {
        components.push(
            ComponentInfo {
                name: component.label().to_string(),
                // TODO: Добавить обработку unwrap()
                temp: component.temperature().unwrap(),
                critical_temp: component.critical().unwrap_or(0.0),
            }
        );
    }
    CpuInfo {
        cpu_usage: sys.global_cpu_usage(),
        threads: sys.cpus().len(),
        // TODO: Добавить обработку unwrap()
        physical_core_count: physical_core_count.unwrap(),
        components_info: components,
    }
}

pub fn collect_system_metrics(sys: &mut System) -> SystemInfo {
        sys.refresh_all();

        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let total_swap = sys.total_swap();
        let used_swap = sys.used_swap();
        let free_swap = sys.free_swap();
        let available_memory = sys.available_memory();

        let load_avg = System::load_average();
        let load_avg = LoadAverage {
            one: load_avg.one,
            five: load_avg.five,
            fifteen: load_avg.fifteen,
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
        let disk = disks.iter_mut().find(|disk| disk.mount_point() == Path::new(&mount_point));

        if let Some(disk) = disk {
            disk.refresh();

            let total_space = disk.total_space();
            let available_space = disk.available_space();

            disk.name();
            disk.kind();
            return DiskInfo {
                name: disk.name().to_string_lossy().to_string(),
                file_system: disk.file_system().to_string_lossy().to_string(),
                kind: disk.kind().to_string(),
                total_space,
                available_space,
            }
        }

        DiskInfo {
            name: String::from("unknown"),
            ..Default::default()
        }
    }


    pub fn collect_network_metrics(networks: &mut Networks) -> NetworkInfo {
        networks.refresh(false);

        let main_interface = networks.iter()
            .filter(|(name, _)| {
                let n = name.as_str();
                n != "lo" && !n.starts_with("br-") && !n.starts_with("docker") && !n.starts_with("veth")
            })
            .max_by_key(|(_, data)| {
                data.total_received() + data.total_transmitted()
            })
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
                }
            }
        }

        NetworkInfo {
            interface_name: "unknown".to_string(),
            ..Default::default()
        }
    }

    pub fn collect_components_metrics(components: &mut Components) {
        todo!()
    }

