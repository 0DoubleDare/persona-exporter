use compact_str::ToCompactString;
use persona_exporter_types::metrics::structs::cpu::{CpuListInfo, CpuThreadInfo};
use sysinfo::System;

pub fn collect_cpus_metrics(
    sys: &System,
    physical_core_count: usize,
    cpu_list_buffer: &mut CpuListInfo,
) {
    cpu_list_buffer.global_cpu_usage = sys.global_cpu_usage() as f64;
    cpu_list_buffer.threads = sys.cpus().len();
    cpu_list_buffer.physical_core_count = physical_core_count;

    sys.cpus().iter().for_each(|c| {
        cpu_list_buffer.cpu_cores.push(CpuThreadInfo {
            frequency: c.frequency(),
            os_name: c.name().to_compact_string(),
            cpu_usage: c.cpu_usage(),
        });
    });
}
