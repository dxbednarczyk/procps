include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CStr;

use libc::c_int;

pub fn lookup(pid: c_int) -> String {
    unsafe {
        let ret = lookup_wchan(pid);
        CStr::from_ptr(ret).to_owned().into_string().unwrap()
    }
}
