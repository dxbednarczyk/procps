use std::ffi::CStr;

use libc::c_int;

use super::*;

pub fn lookup_wchan_str<'a>(pid: c_int) -> &'a str {
    unsafe {
        let ret = lookup_wchan(pid);
        CStr::from_ptr(ret).to_str().unwrap()
    }
}

pub fn lookup_wchan_string(pid: c_int) -> String {
    unsafe {
        let ret = lookup_wchan(pid);
        CStr::from_ptr(ret).to_owned().into_string().unwrap()
    }
}
