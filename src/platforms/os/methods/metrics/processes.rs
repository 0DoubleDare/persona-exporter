use crate::config::ProcessSortBy;
use persona_exporter_types::metrics::structs::processes::ProcessInfo;
use persona_exporter_types::metrics::sysinfo::processes::FromWithNormalizeCpu;
use std::cmp::Ordering;
use sysinfo::{Pid, System};

pub fn get_process_by_id(sys: &System, pid: Pid) -> ProcessInfo {
    let system_process = sys.process(pid).unwrap();
    

    ProcessInfo::from_with_cpu(system_process, sys.cpus().len() as f32)
}

pub fn get_sort_closure(
    sort_by: &ProcessSortBy,
) -> impl Fn(&ProcessInfo, &ProcessInfo) -> Ordering {
    match sort_by {
        ProcessSortBy::Memory => {
            |a: &ProcessInfo, b: &ProcessInfo| b.memory_usage.cmp(&a.memory_usage)
        }
        ProcessSortBy::VirtualMemory => {
            |a: &ProcessInfo, b: &ProcessInfo| b.virtual_memory.cmp(&a.virtual_memory)
        }
        ProcessSortBy::RunTime => |a: &ProcessInfo, b: &ProcessInfo| b.run_time.cmp(&a.run_time),
        ProcessSortBy::StartTime => {
            |a: &ProcessInfo, b: &ProcessInfo| b.start_time.cmp(&a.start_time)
        }
        // default also contain ProcessSortBy::CpuUsage
        _ => |a: &ProcessInfo, b: &ProcessInfo| b.global_cpu_usage.total_cmp(&a.global_cpu_usage),
    }
}

pub fn write_process_info(process: ProcessInfo, buffer: &mut ProcessInfo) {
    buffer.status = process.status;
    buffer.disk_usage = process.disk_usage;

    buffer.global_cpu_usage = process.global_cpu_usage;
    buffer.cpu_usage_per_thread = process.cpu_usage_per_thread;
    buffer.memory_usage = process.memory_usage;
    buffer.virtual_memory = process.virtual_memory;
    buffer.run_time = process.run_time;
    buffer.start_time = process.start_time;

    buffer.name.push_str(&process.name);
    buffer.program_id.push_str(&process.program_id);
    buffer.user_id.push_str(&process.user_id);
    buffer.group_id.push_str(&process.group_id);
}

pub fn update_process_list_info(
    sys: &System,
    // process_list_buffer: &mut Vec<ProcessInfo>,
    process_list_buffer: &mut Vec<ProcessInfo>,
) {
    process_list_buffer.clear();
    // let mut local_process_list_buffer = take(&mut process_list_struct.process_list);

    sys.processes().values().for_each(|process| {
        process_list_buffer.push(ProcessInfo::from_with_cpu(process, sys.cpus().len() as f32));
    });

    // let self_process_metrics: Option<ProcessInfo> = get_current_pid()
    //     .ok()
    //     .and_then(|pid| sys.process(pid))
    //     .map(Into::into);

    // local_process_list_buffer.extend(
    //     sys.processes().values().map(|p| ProcessInfo::from_process(p))
    // );
    // sys
    //     .processes()
    //     .iter()
    //     .for_each(|process| process_list_buffer.push(ProcessInfo::from(process.1)));
    // .collect();

    // let sort_by_closure = get_sort_closure(sort_by);
    //
    // local_process_list_buffer.sort_unstable_by(sort_by_closure);
    // local_process_list_buffer.truncate(process_limit);
    // process_list_struct.process_list = local_process_list_buffer;

    // *process_list_struct = ProcessListInfo {
    //     exporter_metrics: self_process_metrics,
    //     process_list: local_process_list_buffer,
    // };
}
