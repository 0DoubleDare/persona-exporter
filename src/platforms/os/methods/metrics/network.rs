use persona_exporter_types::DEFAULT_UNKNOWN_MESSAGE;
use persona_exporter_types::metrics::structs::network::NetworkInfo;
use sysinfo::Networks;

pub fn collect_network_metrics(networks: &mut Networks, network_info_buffer: &mut NetworkInfo) {
    let main_interface = networks
        .iter()
        .filter(|(name, _)| {
            let n = name.as_str();
            n != "lo" && !n.starts_with("br-") && !n.starts_with("docker") && !n.starts_with("veth")
        })
        .max_by_key(|(_, data)| data.total_received() + data.total_transmitted())
        .map(|(name, _)| name.clone());

    if let Some(interface) = main_interface
        && let Some(data) = networks.get(&interface)
    {
        network_info_buffer.interface_name.push_str(&interface);
        network_info_buffer.total_rx_bytes = data.total_received();
        network_info_buffer.total_tx_bytes = data.total_transmitted();
        network_info_buffer.total_rx_packets = data.total_packets_transmitted();
        network_info_buffer.total_tx_packets = data.total_packets_transmitted();
        network_info_buffer.total_rx_errors = data.total_errors_on_received();
        network_info_buffer.total_tx_errors = data.total_errors_on_transmitted();
    } else {
        network_info_buffer
            .interface_name
            .push_str(DEFAULT_UNKNOWN_MESSAGE);
    }
}
