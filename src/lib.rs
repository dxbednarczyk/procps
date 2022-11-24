#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::time::Duration;

#[derive(Debug, PartialEq, Eq)]
pub struct MemInfo {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub shared: u64,
    pub swap_free: u64,
    pub swap_total: u64,
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

pub fn get_meminfo() -> MemInfo {
    unsafe {
        meminfo();

        MemInfo {
            total: kb_main_total,
            used: kb_main_used,
            free: kb_main_free,
            shared: kb_main_shared,
            swap_free: kb_swap_free,
            swap_total: kb_swap_total,
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;
    use chrono::Local;
    use std::{process::Command, thread};

    #[test]
    fn meminfo() {
        let mi1 = get_meminfo();
        let mi2 = get_meminfo();
        assert_eq!(mi1, mi2);

        let one_second = Duration::from_secs(1);
        thread::sleep(one_second);

        let mi3 = get_meminfo();
        assert_ne!(mi3, mi2);
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

        assert_eq!(
            chrono::Duration::from_std(uptime.active)
                .unwrap()
                .num_seconds(),
            (now - parsed_time).num_seconds()
        );
    }
}
