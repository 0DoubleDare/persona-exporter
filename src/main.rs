use persona_exporter::platforms::*;
use std::env;
#[cfg_attr(target_os = "none", no_std)]
#[cfg_attr(target_os = "none", no_main)]
#[cfg(target_os = "none")]
#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    microcontroller::collect_metrics::collect_metrics_for_microcontroller();
}

#[cfg(not(target_os = "none"))]
use mimalloc::MiMalloc;
use persona_exporter::config::SendModel;
use persona_exporter::platforms::other::methods::{initial_tracing, load_config_file};
use tracing::info;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    let debug_mode: bool = env::var("DEBUG")
        .unwrap_or_else(|_| "true".to_string())
        .parse()
        .unwrap_or(true);

    initial_tracing(debug_mode);

    let config = load_config_file();

    match config.agent.send_model {
        SendModel::Push => {
            info!("Exporter work send model: PUSH");
            smol::block_on(async {
                other::collector::collect_metrics_for_os().await;
            })
        }
        SendModel::Pull => {
            info!("Exporter work send model: PULL");
        }
    }
    info!("Exporter initialized");
}
