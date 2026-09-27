use compact_str::ToCompactString;
use persona_exporter_types::DEFAULT_UNKNOWN_MESSAGE;
use persona_exporter_types::metrics::structs::components::{ComponentInfo, ComponentListInfo};
use sysinfo::Components;

pub fn collect_components_metrics(
    components: &mut Components,
    components_list_buffer: &mut ComponentListInfo,
) {
    components_list_buffer.count = components.len();
    components_list_buffer.is_empty = components.is_empty();

    components.iter().for_each(|c| {
        components_list_buffer.components.push(ComponentInfo {
            id: c
                .id()
                .unwrap_or(DEFAULT_UNKNOWN_MESSAGE)
                .to_compact_string(),
            name: c.label().to_compact_string(),
            temp: c.temperature().unwrap_or(0.0),
            critical_temp: c.critical().unwrap_or(0.0),
            max_temp: c.max().unwrap_or(0.0),
        });
    });
}
