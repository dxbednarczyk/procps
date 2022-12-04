use std::ffi::{c_int, CStr};

use super::*;

pub fn get_uptime_str<'a>(human_readable: bool) -> &'a str {
    unsafe {
        let sprinted = sprint_uptime(human_readable as c_int);
        CStr::from_ptr(sprinted).to_str().unwrap()
    }
}

pub fn get_uptime_string(human_readable: bool) -> String {
    unsafe {
        let sprinted = sprint_uptime(human_readable as c_int);
        CStr::from_ptr(sprinted).to_owned().into_string().unwrap()
    }
}
