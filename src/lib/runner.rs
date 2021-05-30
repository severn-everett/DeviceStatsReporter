use std::env::args;
use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use sysinfo::{DiskExt, ProcessorExt, System, SystemExt};

use crate::lib::common::{MINUTES_MULTIPLIER, RuntimeError, RuntimeMode};
use crate::lib::config::load_config;
use crate::lib::report::{CPUReport, DiskReport, MemoryReport, SystemReport};

pub fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();
    let runner_config = Arc::new(load_config(args.get(1))?);
    let mqtt_opts = paho_mqtt::CreateOptionsBuilder::new()
        .server_uri(runner_config.server_address.as_str())
        .client_id(runner_config.device_name.as_str())
        .finalize();
    let mqtt_client = match paho_mqtt::Client::new(mqtt_opts) {
        Ok(mqtt_client) => mqtt_client,
        Err(e) => {
            let error = Box::new(RuntimeError::new(e.to_string().as_str()));
            return Err(error);
        }
    };
    let conn_opts = paho_mqtt::ConnectOptionsBuilder::new()
        .user_name(runner_config.user_name.as_str())
        .password(runner_config.user_password.as_str())
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();
    if let Err(e) = mqtt_client.connect(conn_opts) {
        let error = Box::new(RuntimeError::new(e.to_string().as_str()));
        return Err(error);
    }
    let mut sys = System::new_all();
    match runner_config.runtime_mode {
        RuntimeMode::Single => {
            match execute_check(&mut sys) {
                Ok(()) => {},
                Err(e) => {
                    eprintln!("An error occurred during check: {}", e);
                }
            };
        },
        RuntimeMode::Continuous => {
            let running = Arc::new(AtomicBool::new(true));
            let r = running.clone();
            let run_thread = thread::spawn(move || {
                while running.load(Ordering::SeqCst) {
                    match execute_check(&mut sys) {
                        Ok(()) => {}
                        Err(e) => {
                            eprintln!("An error occurred during check runtime loop: {}", e);
                            break;
                        }
                    }
                    thread::park_timeout(Duration::from_secs(runner_config.check_interval * MINUTES_MULTIPLIER));
                }
            });
            let run_thread_shutdown = run_thread.thread().clone();
            match ctrlc::set_handler(move || {
                r.store(false, Ordering::SeqCst);
                run_thread_shutdown.unpark();
            }) {
                Ok(()) => {}
                Err(e) => {
                    let error = Box::new(RuntimeError::new(e.to_string().as_str()));
                    return Err(error);
                }
            };
            run_thread.join().unwrap();
        }
    }
    match mqtt_client.disconnect(None) {
        Ok(_) => Ok(()),
        Err(e) => {
            let error = Box::new(RuntimeError::new(e.to_string().as_str()));
            Err(error)
        }
    }
}

fn execute_check(sys: &mut System) -> Result<(), Box<dyn Error>> {
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
    Ok(())
}
