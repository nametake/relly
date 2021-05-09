use crate::disk_manager::DiskManager;

pub struct BufferPoolManager {
    disk: DiskManager,
    pool: BufferP,
}
