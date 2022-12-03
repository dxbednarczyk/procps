#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::{
    ffi::CStr,
    time::Duration, mem, 
};
use libc::{c_ulonglong, c_uint, c_ulong};

pub trait Name {
    fn name_as_string(&self) -> String;
    fn name_as_str(&self) -> &str;
}

type Disk = disk_stat;
type Partition = partition_stat;

#[derive(Debug, PartialEq, Eq)]
pub struct MemInfo {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub shared: u64,
    pub buffers: u64,
    pub cached: u64,
    pub available: u64,
    pub swap_total: u64,
    pub swap_used: u64,
    pub swap_free: u64,
}

#[derive(Debug, PartialEq)]
pub struct LoadAvg {
    pub av1: f64,
    pub av5: f64,
    pub av15: f64,
}

#[derive(Debug)]
pub struct KernelVersion {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

#[derive(Debug)]
pub struct Uptime {
    pub active: Duration,
    pub idle: Duration,
}

#[derive(Debug)]
pub struct DiskStat {
    pub disks: Vec<Disk>,
    pub partitions: Vec<Partition>,
}

#[derive(Debug)]
pub struct Cpu {
    pub user_processes: c_ulonglong,
    pub nice_processes: c_ulonglong,
    pub system_processes: c_ulonglong,
    pub idle: c_ulonglong,
    pub iowait: c_ulonglong,
    pub irq: c_ulonglong,
    pub soft_irq: c_ulonglong,
    pub steal: c_ulonglong,
}

#[derive(Debug)]
pub struct Page {
    pub pin: c_ulong,
    pub pout: c_ulong,
}

#[derive(Debug)]
pub struct Swap {
    pub sin: c_ulong,
    pub sout: c_ulong,
}

#[derive(Debug)]
pub struct Stat {
    pub cpu: Cpu,
    pub page: Page,
    pub swap: Swap,
    pub interrupts: c_uint,
    pub context_switches: c_uint,
    pub running_processes: c_uint,
    pub blocked_processes: c_uint,
    pub btime: c_uint,
    pub processes: c_uint,
}

impl Name for Disk {
    fn name_as_string(&self) -> String {
        unsafe {
            CStr::from_ptr(self.disk_name.as_ptr())
                .to_owned()
                .into_string()
                .unwrap()
        }
    }

    fn name_as_str(&self) -> &str {
        unsafe { CStr::from_ptr(self.disk_name.as_ptr()).to_str().unwrap() }
    }
}

impl Name for Partition {
    fn name_as_string(&self) -> String {
        unsafe {
            CStr::from_ptr(self.partition_name.as_ptr())
                .to_owned()
                .into_string()
                .unwrap()
        }
    }

    fn name_as_str(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.partition_name.as_ptr())
                .to_str()
                .unwrap()
        }
    }
}

pub fn get_meminfo() -> MemInfo {
    unsafe {
        meminfo();

        MemInfo {
            total: kb_main_total,
            used: kb_main_used,
            free: kb_main_free,
            shared: kb_main_shared,
            buffers: kb_main_buffers,
            cached: kb_main_cached,
            available: kb_main_available,
            swap_total: kb_swap_total,
            swap_used: kb_swap_used,
            swap_free: kb_swap_free,
        }
    }
}

pub fn get_loadavg() -> LoadAvg {
    let mut av1 = 0.;
    let mut av5 = 0.;
    let mut av15 = 0.;

    unsafe {
        loadavg(&mut av1, &mut av5, &mut av15);
    }

    LoadAvg { av1, av5, av15 }
}

pub fn get_kernel_info() -> KernelVersion {
    let version;

    unsafe {
        version = procps_linux_version();
    }

    KernelVersion {
        major: (version >> 16) & 0xff,
        minor: (version >> 8) & 0xff,
        patch: version & 0xff,
    }
}

pub fn get_uptime() -> Uptime {
    let mut active = 0.;
    let mut idle = 0.;

    unsafe {
        uptime(&mut active, &mut idle);
    }

    Uptime {
        active: Duration::from_secs_f64(active),
        idle: Duration::from_secs_f64(active),
    }
}

pub fn get_btime() -> u64 {
    unsafe { getbtime() }
}

pub fn get_diskstat() -> DiskStat {
    let diskstat;
    let partitionstat;

    unsafe {
        let mut disks = mem::zeroed();
        let mut partitions = mem::zeroed();

        let diskstat_len = getdiskstat(&mut disks, &mut partitions) as usize;
        diskstat = Vec::from_raw_parts(disks, diskstat_len, diskstat_len);

        let partitionstat_len = getpartitions_num(disks, diskstat_len as i32) as usize;
        partitionstat = Vec::from_raw_parts(partitions, partitionstat_len, partitionstat_len);
    }

    DiskStat {
        disks: diskstat,
        partitions: partitionstat,
    }
}

pub fn get_stat() -> Stat {
    // cpu
    let user_processes: *mut c_ulonglong = [0; 2].as_mut_ptr();
    let nice_processes: *mut c_ulonglong = [0; 2].as_mut_ptr();
    let system_processes: *mut c_ulonglong = [0; 2].as_mut_ptr();
    let idle: *mut c_ulonglong = [0; 2].as_mut_ptr();
    // not separated out until the 2.5.41 kernel
    let iowait: *mut c_ulonglong = [0; 2].as_mut_ptr(); 
    // not separated out until the 2.6.0-test4 kernel
    let irq: *mut c_ulonglong = [0; 2].as_mut_ptr(); 
    // not separated out until the 2.6.0-test4 kernel
    let soft_irq: *mut c_ulonglong = [0; 2].as_mut_ptr();
    // not separated out until the 2.6.11 kernel
    let steal: *mut c_ulonglong = [0; 2].as_mut_ptr();

    // page
    let pin: *mut c_ulong = [0; 2].as_mut_ptr();
    let pout: *mut c_ulong = [0; 2].as_mut_ptr();

    // swap
    let sin: *mut c_ulong = [0; 2].as_mut_ptr();
    let sout: *mut c_ulong = [0; 2].as_mut_ptr();

    // other
    let interrupts: *mut c_uint = [0; 2].as_mut_ptr();
    let context_switches: *mut c_uint = [0; 2].as_mut_ptr();
    let mut running_processes: c_uint = 0;
    let mut blocked_processes: c_uint = 0;
    let mut btime: c_uint = 0;
    let mut processes: c_uint = 0;

    unsafe {
        getstat(
            user_processes,
            nice_processes,
            system_processes,
            idle,
            iowait,
            irq,
            soft_irq,
            steal,
            pin,
            pout,
            sin,
            sout,
            interrupts,
            context_switches,
            &mut running_processes,
            &mut blocked_processes,
            &mut btime,
            &mut processes,
        );

        // deref of *mut T is essentially 
        // let something = [T; 3];
        // something[0]
        Stat {
            cpu: Cpu {
                user_processes: *user_processes,
                nice_processes: *nice_processes,
                system_processes: *system_processes,
                idle: *idle,
                iowait: *iowait,
                irq: *irq,
                soft_irq: *soft_irq,
                steal: *steal,
            },
            page: Page {
                pin: *pin,
                pout: *pout,
            },
            swap: Swap {
                sin: *sin,
                sout: *sout,
            },
            interrupts: *interrupts,
            context_switches: *context_switches,
            running_processes,
            blocked_processes,
            btime,
            processes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;
    use chrono::Local;
    use datetime::Instant;
    use std::process::Command;

    #[test]
    fn meminfo() {
        let free_output = Command::new("free")
            .arg("-w")
            .output()
            .expect("failed to run free");

        let values: Vec<Vec<u64>> = String::from_utf8(free_output.stdout)
            .unwrap()
            .lines()
            .skip(1)
            .map(|x| {
                x.split_whitespace()
                    .filter_map(|y| y.parse().ok())
                    .collect()
            })
            .collect();

        assert_eq!(values.len(), 2);
        assert_eq!(values[0].len(), 7);
        assert_eq!(values[1].len(), 3);

        let meminfo = get_meminfo();

        assert_eq!(meminfo.total, values[0][0]);
        assert_eq!(meminfo.used, values[0][1]);
        assert_eq!(meminfo.free, values[0][2]);
        assert_eq!(meminfo.shared, values[0][3]);
        assert_eq!(meminfo.buffers, values[0][4]);
        assert_eq!(meminfo.cached, values[0][5]);
        assert_eq!(meminfo.available, values[0][6]);
        assert_eq!(meminfo.swap_total, values[1][0]);
        assert_eq!(meminfo.swap_used, values[1][1]);
        assert_eq!(meminfo.swap_free, values[1][2]);
    }

    #[test]
    fn loadavg() {
        let uptime_output = Command::new("uptime")
            .output()
            .expect("failed to run uptime");

        let stdout = String::from_utf8(uptime_output.stdout).unwrap();

        let mut avgs: Vec<f64> = stdout
            .split_whitespace()
            .filter_map(|x| x.replace([',', '\n'], "").parse().ok())
            .collect();

        if stdout.contains("minutes") || stdout.contains("min") {
            avgs.remove(0);
            avgs.remove(0);
        } else {
            avgs.remove(0);
        }

        let loadavg = get_loadavg();

        assert_eq!(avgs.len(), 3);
        assert_eq!(avgs[0], loadavg.av1);
        assert_eq!(avgs[1], loadavg.av5);
        assert_eq!(avgs[2], loadavg.av15);
    }

    #[test]
    fn kernel() {
        let kernel_version = get_kernel_info();

        let uname_output = Command::new("uname")
            .args(["-r"])
            .output()
            .expect("failed to run uname");

        let version_nums: Vec<i32> = String::from_utf8(uname_output.stdout)
            .unwrap()
            .split('-')
            .next()
            .unwrap()
            .split('.')
            .filter_map(|x| x.parse().ok())
            .collect();

        // all kernel versions have a major, minor and patch version
        // but not all have a 4th field, e.g. WSL kernels
        assert!(version_nums.len() <= 4 && version_nums.len() > 2);
        assert_eq!(version_nums[0], kernel_version.major);
        assert_eq!(version_nums[1], kernel_version.minor);
        assert_eq!(version_nums[2], kernel_version.patch);
    }

    #[test]
    fn uptime() {
        let uptime = get_uptime();

        let uptime_output = Command::new("uptime")
            .arg("-s")
            .output()
            .expect("failed to run uptime");

        let stdout = String::from_utf8(uptime_output.stdout)
            .unwrap()
            .replace('\n', "");

        let parsed_time = Local
            .datetime_from_str(&stdout, "%Y-%m-%d %H:%M:%S")
            .expect("parsing error for `uptime -s`");

        let now = Local::now();

        // can sometimes be off by 1 second
        assert!(
            chrono::Duration::from_std(uptime.active)
                .unwrap()
                .num_seconds()
                - 1
                <= (now - parsed_time).num_seconds()
        );
    }

    #[test]
    fn btime() {
        let btime = get_btime();
        let now = Instant::now();
        let uptime = get_uptime().active;

        // boot time will be earlier than uptime by 1 second
        assert_eq!(now.seconds() as u64 - btime - 1, uptime.as_secs());
    }
}
