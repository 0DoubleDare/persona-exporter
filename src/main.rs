use persona_exporter::platforms::*;

#[cfg_attr(target_os = "none", no_std)]
#[cfg_attr(target_os = "none", no_main)]
#[cfg(target_os = "none")]
#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    microcontroller::collect_metrics::collect_metrics_for_microcontroller();
}

#[cfg(not(target_os = "none"))]
use mimalloc::MiMalloc;
use persona_exporter::config::{MainCliArguments, SendModel};
use persona_exporter::platforms::os::methods::{initial_tracing, load_config};
use tracing::info;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
fn main() {
    let args: MainCliArguments = argh::from_env();

    initial_tracing(args.verbose);

    let config = load_config(args.config_path);
    info!("Success initial configuration: {:#?}", config);

    smol::block_on(async {
        match config.agent.send_model {
            SendModel::Push => {
                info!("Exporter work send model: PUSH");
                os::collector::collect_metrics_for_os(config).await;
            }
            SendModel::Pull => {
                info!("Exporter work send model: PULL");
            }
        }
    });
    info!("Exporter initialized");
}
