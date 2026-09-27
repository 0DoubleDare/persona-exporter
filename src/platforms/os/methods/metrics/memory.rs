use persona_exporter_types::metrics::structs::memory::MemoryInfo;
use sysinfo::System;

pub fn collect_memory_metrics(sys: &System, memory_buffer: &mut MemoryInfo) {
    memory_buffer.total_memory = sys.total_memory();
    memory_buffer.used_memory = sys.used_memory();
    memory_buffer.free_memory = sys.free_memory();
    memory_buffer.available_memory = sys.available_memory();
    memory_buffer.total_swap = sys.total_swap();
    memory_buffer.used_swap = sys.used_swap();
    memory_buffer.free_swap = sys.free_swap();
}

pub fn initial_memory_snapshot() {}
