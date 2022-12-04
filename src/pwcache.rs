use std::ffi::CStr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn get_user(uid: uid_t) -> String {
    unsafe {
        let ret = pwcache_get_user(uid);
        CStr::from_ptr(ret).to_owned().into_string().unwrap()
    }
}

pub fn get_group(gid: gid_t) -> String {
    unsafe {
        let ret = pwcache_get_group(gid);
        CStr::from_ptr(ret).to_owned().into_string().unwrap()
    }
}
