include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::c_int;

#[derive(Debug)]
pub struct KernelVersion {
    pub major: c_int,
    pub minor: c_int,
    pub patch: c_int,
}

pub fn get_kernel_version() -> KernelVersion {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn version() {
        let kernel_version = get_kernel_version();

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
}
