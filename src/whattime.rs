include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{c_int, CStr};

pub fn get_uptime_string(human_readable: bool) -> String {
    unsafe {
        let sprinted = sprint_uptime(human_readable as c_int);
        CStr::from_ptr(sprinted).to_owned().into_string().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    use super::*;

    #[test]
    fn whattime() {
        let uptime_output = Command::new("uptime")
            .output()
            .expect("failed to run uptime");

        let uptime_stdout = String::from_utf8(uptime_output.stdout)
            .unwrap()
            .replace('\n', "");

        let pretty_uptime_output = Command::new("uptime")
            .arg("-p")
            .output()
            .expect("failed to run uptime");

        let pretty_uptime_stdout = String::from_utf8(pretty_uptime_output.stdout)
            .unwrap()
            .replace('\n', "");

        assert_eq!(get_uptime_string(true), pretty_uptime_stdout);
        assert_eq!(get_uptime_string(false), uptime_stdout);
    }
}
