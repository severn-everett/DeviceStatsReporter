use serde::Serialize as SerdeSerialize;

#[derive(Debug,SerdeSerialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemReport {
    pub disks: Box<[DiskReport]>,
    pub cpus: Box<[CPUReport]>,
    pub memory: MemoryReport,
}

#[derive(Debug,SerdeSerialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskReport {
    pub name: String,
    pub disk_used: u64,
    pub disk_capacity: u64,
}

#[derive(Debug,SerdeSerialize)]
#[serde(rename_all = "camelCase")]
pub struct CPUReport {
    pub name: String,
    pub brand: String,
    pub vendor_id: String,
    pub frequency: u64,
    pub usage: f32,
}

#[derive(Debug,SerdeSerialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryReport {
    pub memory_used: u64,
    pub memory_capacity: u64,
}
