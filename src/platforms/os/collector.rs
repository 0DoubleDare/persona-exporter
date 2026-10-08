use crate::config::{AgentConfigFile, DataType};
use crate::platforms::os::methods::metrics::processes::{get_sort_closure, update_process_list_info, get_process_by_id, write_process_info};
use core::time;

use crate::platforms::os::methods::arguments::{Buffers, GlobalContext, RefreshKindContext, RequestBodyOptions, SystemContext};
use crate::platforms::os::methods::{
    build_request_body, collect_metrics_as_line_protocol, create_metrics_struct_by_config,
    send_request,
};

use crate::platforms::os::methods::metrics::components::collect_components_metrics;
use crate::platforms::os::methods::metrics::cpu::collect_cpus_metrics;
use crate::platforms::os::methods::metrics::disk::collect_storage_list_metrics;
use crate::platforms::os::methods::metrics::memory::collect_memory_metrics;
use crate::platforms::os::methods::metrics::network::collect_network_metrics;
use crate::platforms::os::methods::metrics::system::collect_system_metrics;
use persona_exporter_types::metrics::traits::Clear;
use smol::stream::StreamExt;
use std::time::{Duration, SystemTime};
use sysinfo::{get_current_pid, Components, Disks, Networks, ProcessesToUpdate, System};
use tracing::{debug, info};

pub async fn collect_metrics_for_os(config: AgentConfigFile) {
    // TODO LIST:
    // Добавить массив url и send_interval - чтобы один экспортер мой слать метрики на разные url с разной переодичностью.
    // Как это будет выглядеть:
    // ...
    //  targets:
    //      - urls: ["https://example.com/" "https://example2.com"]
    //        send_interval: 10s
    //        timeout: 30s
    // ...
    let sort_by = get_sort_closure(&config.metrics.processes.sort_by);
    let mut global_context =
        GlobalContext::new(
            &config,
            sort_by,
            create_metrics_struct_by_config(&config),
        );

    // CPU, Memory, Processes & System
    // let mut system_context = (config.metrics.cpu.settings.enabled
    //     || config.metrics.memory.settings.enabled
    //     || config.metrics.processes.settings.enabled
    //     || config.metrics.processes.include_exporter_metrics
    //     || config.metrics.system.settings.enabled)
    //     .then(|| SystemContext {
    //         system_snapshot: System::new(),
    //         refresh_kinds: RefreshKindContext::new(&config.metrics),
    //     });

    let mut disks = config
        .metrics
        .disks
        .settings
        .enabled
        .then(Disks::new_with_refreshed_list);
    let mut networks = config
        .metrics
        .network
        .settings
        .enabled
        .then(Networks::new_with_refreshed_list);
    let mut components = config
        .metrics
        .components
        .settings
        .enabled
        .then(Components::new_with_refreshed_list);

    let additional_headers = &config.server.push.http_headers;
    let url_params = &config.server.push.url_params;
    let target_url = &config.server.push.url;
    let await_seconds = config.server.push.send_interval;
    let mut interval = smol::Timer::interval(Duration::from_secs(await_seconds));

    let http_client_config = ureq::Agent::config_builder()
        .timeout_global(Some(time::Duration::from_secs(64)))
        .build();

    let request_options = RequestBodyOptions {
        config: &http_client_config,
        url: target_url,
        get_params: url_params,
        headers: additional_headers,
    };

    // let mut line_protocol_buffer = &mut global_context.buffers.line_protocol_buffer;
    // let mut buffers = Buffers {
    //     metrics: create_metrics_struct_by_config(&config),
    //     ..Buffers::default()
    // };
    // let physical_core_count = System::physical_core_count().unwrap_or(0);
    // let sort_by = get_sort_closure(&config.metrics.processes.sort_by);
    // let process_limit = config.metrics.processes.process_limit;
    // let global_tags = config.metrics.global_tags;

    // Основной цикл сбора
    while interval.next().await.is_some() {
        info!("Next metrics created after {} seconds", await_seconds);

        // Метрики требующий sysinfo::System
        if let Some(ref mut sys_context) = global_context.system_context {
            let s = &mut sys_context.system_snapshot;
            // Здесь не нужна проверка, включена ли та или иная секция метрик в конфиге. Так как если выключена
            // То buffers.metrics.memory будет равнятся None. Такое поведение заложено ещё в инициализации структуры ServerMetrics
            if let Some(ref mut mem_buf) = global_context.buffers.metrics.memory {
                s.refresh_memory();
                // mem_buf.clear_dynamic();
                collect_memory_metrics(s, mem_buf);
            }
            if let Some(ref mut cpu_buf) = global_context.buffers.metrics.cpu {
                s.refresh_cpu_all();
                cpu_buf.clear_dynamic();
                collect_cpus_metrics(s, global_context.variables.physical_core_count, cpu_buf);
            }
            if let Some(ref mut system_buf) = global_context.buffers.metrics.system {
                system_buf.clear_dynamic();
                collect_system_metrics(system_buf);
            }
            if let (Some(process_list_buf), Some(refresh_kind)) = (
                &mut global_context.buffers.metrics.process_list,
                sys_context.refresh_kinds.process_refresh_kind,
            ) {
                s.refresh_processes_specifics(
                    ProcessesToUpdate::All,
                    config.metrics.processes.remove_dead_processes,
                    refresh_kind,
                );
                // s.refresh_processes(ProcessesToUpdate::All, false);
                process_list_buf.clear_dynamic();

                // Информация о процессах
                update_process_list_info(s, &mut process_list_buf.process_list);
                //// Сортируем по заданной функции aka замыканию
                process_list_buf.process_list.sort_unstable_by(&global_context.variables.sort_by);
                //// Обрезаем готовый массив
                process_list_buf.process_list.truncate(global_context.variables.process_limit);

                // Отдельная информация о самом экспортере
                if config.metrics.processes.include_exporter_metrics
                    && let (Ok(pid), Some(self_metrics)) =
                        (get_current_pid(), &mut process_list_buf.exporter_metrics)
                {
                    self_metrics.clear_dynamic();
                    let process = get_process_by_id(s, pid);
                    write_process_info(process, self_metrics);
                }
            }
        }

        if let (Some(disk_buffer), Some(d)) = (&mut global_context.buffers.metrics.disk, &mut disks) {
            d.refresh(false);
            disk_buffer.clear_dynamic();
            collect_storage_list_metrics(d, disk_buffer);
        }
        if let (Some(network_buffer), Some(n)) = (&mut global_context.buffers.metrics.network, &mut networks) {
            n.refresh(false);
            network_buffer.clear_dynamic();
            collect_network_metrics(n, network_buffer);
        }
        if let (Some(components_info), Some(c)) = (&mut global_context.buffers.metrics.components, &mut components)
        {
            c.refresh(false);
            components_info.clear_dynamic();
            collect_components_metrics(c, components_info);
        }

        global_context.buffers.metrics.time = i64::try_from( SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos()).unwrap_or_default();

        let request = build_request_body(&request_options);
        match config.agent.data_type {
            DataType::LineProtocol => {
                let mut line_buffer = std::mem::take(&mut global_context.buffers.line_protocol_buffer);

                line_buffer.clear();
                collect_metrics_as_line_protocol(
                    &global_context.buffers.metrics,
                    &mut line_buffer,
                    &global_context.variables.global_tags,
                );

                info!("Sending data to a specified URL",);
                debug!("{:#?}", String::from_utf8(line_buffer.clone()));

                send_request(request, &line_buffer).await;
            }
            DataType::Json => {
                info!("Machine metrics: {:#?}", global_context.buffers.metrics);

                let json_body = serde_json::json!(global_context.buffers.metrics);

                send_request(request, &json_body.to_string()).await;
            }
        }
    }
}
