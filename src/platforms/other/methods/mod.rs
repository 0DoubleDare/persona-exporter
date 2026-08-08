use crate::config::{HeaderField, ParamField};
use influxdb_line_protocol::LineProtocolBuilder;
use influxdb_line_protocol::builder::AfterField;
use persona_exporter_types::metrics::{
    ComponentsInfo, CpuInfo, DiskInfo, MemoryInfo, NetworkInfo, ProcessesInfo, SystemInfo,
};
use reqwest::{Client, RequestBuilder};
use tracing::{debug, error, info};
pub struct ToLineProtocolArgument<'a> {
    pub time: i64,
    pub system: &'a SystemInfo,
    pub memory: &'a MemoryInfo,
    pub disk: &'a DiskInfo,
    pub network: &'a NetworkInfo,
    pub cpu: &'a CpuInfo,
    pub components: &'a ComponentsInfo,
    pub processes_info: &'a ProcessesInfo,
}
pub fn collect_metrics_as_line_protocol(metrics: ToLineProtocolArgument) -> Vec<u8> {
    let lines: [LineProtocolBuilder<Vec<u8>, AfterField>; 5] = [
        metrics.system.into(),
        metrics.memory.into(),
        metrics.disk.into(),
        metrics.cpu.into(),
        metrics.network.into(),
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
            LineProtocolBuilder::from(c)
                .timestamp(metrics.time)
                .close_line()
                .build()
        })
        .collect();

    let line_processes: Vec<u8> = metrics
        .processes_info
        .processes
        .iter()
        .flat_map(|p| {
            LineProtocolBuilder::from(p)
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
            LineProtocolBuilder::from(p)
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

pub fn build_request_body(
    client: &Client,
    url: &String,
    get_params: &[ParamField],
    headers: &[HeaderField],
) -> RequestBuilder {
    let get_pairs: Vec<(&str, &str)> = get_params
        .iter()
        .map(|p| (p.key.as_str(), p.value.as_str()))
        .collect();
    let base = client.post(url).query(&get_pairs);
    headers
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
