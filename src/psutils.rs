use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use psutil::cpu::CpuTimesPercent;
use psutil::disk::DiskIoCounters;
use psutil::memory::{SwapMemory, VirtualMemory};
use psutil::sensors::TemperatureSensor;
use psutil::*;

pub struct CpuStat {
    percent: f32,
    times_percent_user: f32,
    times_percent_system: f32,
    times_percent_idle: f32,
}

pub struct SystemStat {
    cpus: Vec<CpuStat>,
    virtual_memory: VirtualMemory,
    swap_memory: SwapMemory,
    diskio: HashMap<String, DiskIoCounters>,
    uptime: Duration,
    temperatures: Vec<Result<TemperatureSensor>>,
}

pub fn list() -> Result<SystemStat> {
    let block_time = Duration::from_millis(1000);

    let mut cpu_percent_collector = cpu::CpuPercentCollector::new().unwrap();
    let mut cpu_times_percent_collector: cpu::CpuTimesPercentCollector =
        cpu::CpuTimesPercentCollector::new().unwrap();

    let mut disk_io_counters_collector = disk::DiskIoCountersCollector::default();

    let cpu_percents_percpu: Vec<f32> = cpu_percent_collector.cpu_percent_percpu().unwrap();
    let cpu_times_percent_percpu: Vec<CpuTimesPercent> = cpu_times_percent_collector
        .cpu_times_percent_percpu()
        .unwrap();

    let mut cpus: Vec<CpuStat> = Vec::new();

    for i in 0..cpu_percents_percpu.len() {
        cpus.push(CpuStat {
            percent: cpu_percents_percpu[i],
            times_percent_user: cpu_times_percent_percpu[i].user(),
            times_percent_system: cpu_times_percent_percpu[i].system(),
            times_percent_idle: cpu_times_percent_percpu[i].idle(),
        })
    }

    let systemStat = SystemStat {
        cpus: cpus,
        diskio: disk_io_counters_collector
            .disk_io_counters_per_partition()
            .unwrap(),
        uptime: host::uptime().unwrap(),
        swap_memory: memory::swap_memory().unwrap(),
        virtual_memory: memory::virtual_memory().unwrap(),
        temperatures: sensors::temperatures(),
    };

    Ok(systemStat)
}
