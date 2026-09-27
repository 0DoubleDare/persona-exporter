use persona_exporter_types::metrics::structs::disk::{StorageListInfo, StorageMountPointInfo};
use sysinfo::Disks;

pub fn collect_storage_list_metrics(disks: &mut Disks, disk_buffer: &mut StorageListInfo) {
    for disk in disks.list() {
        disk_buffer
            .storage_list
            .push(StorageMountPointInfo::from(disk));
    }
}

// pub fn collect_storage_info(mount_points: Vec<StorageMountPointInfo>) -> StorageInfo {
//   StorageInfo {
//       mount_points: mount_points,
//       name: Default::default(),
//       kind: Default::default(),
//   }
// }
