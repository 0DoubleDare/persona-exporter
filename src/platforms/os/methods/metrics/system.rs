use compact_str::ToCompactString;
use persona_exporter_types::metrics::structs::system::{LoadAverage, SystemInfo};
use persona_exporter_types::DEFAULT_UNKNOWN_MESSAGE;
use sysinfo::System;

pub fn collect_system_metrics(system_metrics_buffer: &mut SystemInfo) {
    system_metrics_buffer
        .name
        .push_str(System::name().as_deref().unwrap_or(DEFAULT_UNKNOWN_MESSAGE));
    system_metrics_buffer.kernel_version.push_str(
        System::kernel_version()
            .as_deref()
            .unwrap_or(DEFAULT_UNKNOWN_MESSAGE),
    );
    system_metrics_buffer.os_version.push_str(
        System::os_version()
            .as_deref()
            .unwrap_or(DEFAULT_UNKNOWN_MESSAGE),
    );
    system_metrics_buffer.host_name.push_str(
        System::host_name()
            .as_deref()
            .unwrap_or(DEFAULT_UNKNOWN_MESSAGE),
    );

    system_metrics_buffer
        .kernel_long_version
        .push_str(&System::kernel_long_version());
    system_metrics_buffer.cpu_arch.push_str(&System::cpu_arch());
    system_metrics_buffer
        .distribution_id
        .push_str(&System::distribution_id());
    system_metrics_buffer
        .distribution_id_like
        .push(System::distribution_id_like().join(",").to_compact_string());
    system_metrics_buffer.boot_time = System::boot_time();
    system_metrics_buffer.uptime = System::uptime();
    system_metrics_buffer.load_average = LoadAverage::from(System::load_average());

    // *system_metrics_buffer = SystemInfo {
    //     name: System::name().unwrap_or(DEFAULT_UNKNOWN_MESSAGE.to_string()),
    //     kernel_long_version: System::kernel_long_version(),
    //     kernel_version: System::kernel_version().unwrap_or(DEFAULT_UNKNOWN_MESSAGE.to_string()),
    //     distribution_id: System::distribution_id(),
    //     distribution_id_like: System::distribution_id_like(),
    //     cpu_arch: System::cpu_arch(),
    //     boot_time: System::boot_time(),
    //     uptime: System::uptime(),
    //     os_version: System::os_version().unwrap_or(DEFAULT_UNKNOWN_MESSAGE.to_string()),
    //     host_name: System::host_name().unwrap_or(DEFAULT_UNKNOWN_MESSAGE.to_string()),
    //     load_average: load_avg,
    // };
}
