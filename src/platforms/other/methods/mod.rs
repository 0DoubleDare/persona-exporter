pub mod types;

use deboa::request::{post, DeboaRequestBuilder};
use crate::config::AgentConfigFile;
use crate::platforms::other::methods::types::RequestBodyOptions;
pub(crate) use crate::platforms::other::methods::types::ToLineProtocolOptions;
use influxdb_line_protocol::LineProtocolBuilder;
use influxdb_line_protocol::builder::AfterField;
use persona_exporter_types::metrics::ProcessInfo;
use persona_exporter_types::traits::line_protocol::{FromWithMeasurement, IntoWithMeasurement};
use tracing::{debug, error, info};
use url::Url;

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

pub fn build_request_body(options: &RequestBodyOptions) -> DeboaRequestBuilder {
    let mut total_url = options.url.clone();

    if !options.get_params.is_empty() {
        let mut query_string = String::new();
        let get_url_pairs = form_urlencoded::Serializer::new(&mut query_string);

    options.get_params.iter()
            .fold(get_url_pairs, |mut acc, get_param| {
                acc.append_pair(get_param.key.as_str(), get_param.value.as_str());
                acc
            }).finish();
        total_url = format!("{}?{}", total_url, query_string);
    }
    let headers: Vec<(&str, &str)> = options
        .headers
        .iter()
        .map(|h| (h.key.as_str(), h.value.as_str()))
        .collect();


    post(total_url).unwrap()
        .headers(headers)
        .header(http::header::HOST, &options.host)
        .header(http::header::CONNECTION, "keep_alive")

}

pub async fn send_request(request: DeboaRequestBuilder, client: &deboa_smol::Client) {
    let response = request.send_with(client).await;

    match response {
        Ok(success_response) => {
            let response_status = success_response.status();
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
            debug!("{:#?}", success_response);
            debug!(
                "{}",
                success_response.text().await.unwrap_or_default()
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

pub fn get_host_from_url(url: &str) -> String {
    if let Ok(parsed_url) = Url::parse(url) {
        return parsed_url.host_str().unwrap_or("localhost").to_string()
    };
    "incorrect_url".to_string()
}
