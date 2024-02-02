//#![windows_subsystem = "windows"]

use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::process;
use winapi::um::winbase::GetCurrentHwProfileA;

#[link_section = ".code"]
pub fn hwid_check() {
    let mut info = MaybeUninit::uninit();
    let hwid;

    let hwids = [""];

    unsafe {
        GetCurrentHwProfileA(info.as_mut_ptr());
        let safe_info = info.assume_init();
        let c_hwid = safe_info.szHwProfileGuid;
        hwid = CStr::from_ptr(c_hwid.as_ptr());
    }

    if !hwids.contains(&hwid.to_str().unwrap()) {
        println!("cracker detected! ratting your entire pc!");
        process::abort();
    }
}
