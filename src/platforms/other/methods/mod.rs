pub mod types;

use crate::config::AgentConfigFile;
use crate::platforms::other::methods::types::RequestBodyOptions;
pub(crate) use crate::platforms::other::methods::types::ToLineProtocolOptions;
use influxdb_line_protocol::LineProtocolBuilder;
use influxdb_line_protocol::builder::AfterField;
use persona_exporter_types::metrics::ProcessInfo;
use persona_exporter_types::traits::line_protocol::{FromWithMeasurement, IntoWithMeasurement};
use reqwest::RequestBuilder;
use tracing::{debug, error, info};

pub fn collect_metrics_as_line_protocol(metrics: ToLineProtocolOptions) -> Vec<u8> {
    let lines: [LineProtocolBuilder<Vec<u8>, AfterField>; 5] = [
        metrics.system.into_with_name("metrics_system"),
        metrics.memory.into_with_name("metrics_memory"),
        metrics.disk.into_with_name("metrics_disk"),
        metrics.cpu.into_with_name("metrics_cpu"),
        metrics.network.into_with_name("metrics_network"),
    ];
    // Сбор метрик в одну строку
    // TODO: Операция очень прожорлива по памяти поэтому надо оптимизейшн
    let lines: Vec<u8> = lines
        .into_iter()
        .flat_map(|x| x.timestamp(metrics.time).close_line().build())
        .collect();

    let line_components: Vec<u8> = metrics
        .components
        .components
        .iter()
        .flat_map(|c| {
            LineProtocolBuilder::from_with_name(c, "metrics_component_list")
                .timestamp(metrics.time)
                .close_line()
                .build()
        })
        .collect();

    let line_processes: Vec<u8> = metrics
        .processes_info
        .process_list
        .iter()
        .flat_map(|p: &ProcessInfo| {
            LineProtocolBuilder::from_with_name(p, "metrics_process_list")
                .timestamp(metrics.time)
                .close_line()
                .build()
        })
        .collect();
    let line_self_process: Vec<u8> = metrics
        .processes_info
        .exporter_metrics
        .iter()
        .flat_map(|p| {
            LineProtocolBuilder::from_with_name(p, "metrics_process_list")
                .timestamp(metrics.time)
                .close_line()
                .build()
        })
        .collect();
    [
        lines.as_slice(),
        line_processes.as_slice(),
        line_components.as_slice(),
        line_self_process.as_slice(),
    ]
    .concat()
}

pub fn build_request_body(options: &RequestBodyOptions) -> RequestBuilder {
    let get_pairs: Vec<(&str, &str)> = options
        .get_params
        .iter()
        .map(|p| (p.key.as_str(), p.value.as_str()))
        .collect();
    let base = options.client.post(&options.url).query(&get_pairs);
    options
        .headers
        .iter()
        .fold(base, |acc, header| acc.header(&header.key, &header.value))
}

pub async fn send_request(request: RequestBuilder) {
    let response = request.send().await;

    match response {
        Ok(response) => {
            let response_status = response.status();
            let status_code_type = response_status.as_u16() / 100;
            match status_code_type {
                4 => {
                    error!("What is wrong on the client side: {}", response_status);
                }
                5 => {
                    error!("What is wrong on the server side: {}", response_status);
                }
                _ => {
                    info!("Positive server response: {}", response_status);
                }
            }
            debug!("{:#?}", response);
            debug!(
                "{}",
                response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Body JSON is empty".to_string())
            )
        }
        Err(err) => {
            error!("Send error: {}", err)
        }
    }
}

pub fn load_config_file() -> AgentConfigFile {
    AgentConfigFile::new().unwrap_or_else(|err| {
        error!("Something is wrong in your config file");
        panic!("{}", err);
    })
}

pub fn initial_tracing(debug: bool) {
    tracing_subscriber::fmt()
        .with_env_filter(if debug { "debug" } else { "info" })
        .init();
}
