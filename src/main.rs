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

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    other::collector::collect_metrics_for_os().await;
}
