pub mod arguments;

use crate::config::AgentConfigFile;
use crate::platforms::other::methods::arguments::RequestBodyOptions;
pub(crate) use crate::platforms::other::methods::arguments::ToLineProtocolOptions;
use influxdb_line_protocol::LineProtocolBuilder;
use influxdb_line_protocol::builder::AfterField;
use persona_exporter_types::metrics::ProcessInfo;
use persona_exporter_types::traits::line_protocol::{FromWithMeasurement, IntoWithMeasurement};
use surf::post;
use tracing::{debug, error};
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

pub fn build_request_body(options: &RequestBodyOptions) -> surf::RequestBuilder {
    let total_url = options.url.clone();

    // if !options.get_params.is_empty() {
    //     let mut query_string = String::new();
    //     let get_url_pairs = form_urlencoded::Serializer::new(&mut query_string);
    //
    //     options
    //         .get_params
    //         .iter()
    //         .fold(get_url_pairs, |mut acc, get_param| {
    //             acc.append_pair(get_param.key.as_str(), get_param.value.as_str());
    //             acc
    //         })
    //         .finish();
    //     total_url = format!("{}?{}", total_url, query_string);
    // }
    // let headers: Vec<(&str, &str)> = options
    //     .headers
    //     .iter()
    //     .map(|h| (h.key.as_str(), h.value.as_str()))
    //     .collect();

    let mut request = post(total_url)
        .query(&options.get_params).unwrap()
        .header(http::header::HOST.as_str(), &options.host)
        .header(http::header::CONNECTION.as_str(), "close");

    for header in &options.headers {
        request = request.header(header.key.as_str(), header.value.as_str());
    }

    request

}

pub async fn send_request(request: surf::RequestBuilder, client: &surf::Client) {
    let response = request.send().await;

    match response {
        Ok(success_response) => {
            let response_status = success_response.status();
            debug!("{:#?}", success_response);
            debug!("Canonical reason: {}", response_status.canonical_reason())
        }
        Err(err) => {
            error!("Send error: {}", err)
        }
    }
}

pub fn load_config() -> AgentConfigFile {
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

pub fn get_host(url: &str) -> String {
    if let Ok(parsed_url) = Url::parse(url) {
        return parsed_url.host_str().unwrap_or("localhost").to_string();
    };
    "incorrect_url".to_string()
}

// pub fn parse_cli_arguments()
