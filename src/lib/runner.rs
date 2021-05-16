use sysinfo::{System, SystemExt, DiskExt, ProcessorExt};
use crate::lib::report::{DiskReport, SystemReport, MemoryReport, CPUReport};

pub fn run() {
    let mut sys = System::new_all();
    sys.refresh_all();
    // Collect disk data
    let disk_reports: Vec<DiskReport> = sys.get_disks().iter().filter_map(|d| {
        let disk_name = match d.get_name().to_str() {
            Some(name) => name,
            None => return None
        };
        let disk_capacity = d.get_total_space();
        Some(DiskReport {
            name: String::from(disk_name),
            disk_used: disk_capacity - d.get_available_space(),
            disk_capacity,
        })
    }).collect();
    // Collect memory data
    let memory_capacity = sys.get_total_memory();
    let memory_report = MemoryReport {
        memory_used: memory_capacity - sys.get_available_memory(),
        memory_capacity,
    };
    let cpu_reports: Vec<CPUReport> = sys.get_processors().iter().map(|x| {
        CPUReport {
            name: String::from(x.get_name()),
            brand: String::from(x.get_brand()),
            vendor_id: String::from(x.get_vendor_id()),
            frequency: x.get_frequency(),
            usage: x.get_cpu_usage(),
        }
    }).collect();

    let sys_report = SystemReport {
        disks: disk_reports.into_boxed_slice(),
        cpus: cpu_reports.into_boxed_slice(),
        memory: memory_report,
    };
    println!("System Report: {:?}", sys_report);
}
