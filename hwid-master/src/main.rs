use colored::*;
use std::{ffi::CStr, io, mem::MaybeUninit};
use winapi::um::winbase::GetCurrentHwProfileA;

fn main() {
    let mut info = MaybeUninit::uninit();
    let hwid;

    unsafe {
        GetCurrentHwProfileA(info.as_mut_ptr());
        let safe_info = info.assume_init();
        let c_hwid = safe_info.szHwProfileGuid;
        hwid = CStr::from_ptr(c_hwid.as_ptr());
    }

    println!(
        "{}\n{}\n{}",
        "Your HWID is:",
        hwid.to_str().unwrap().black().on_white(),
        "Send this to whoever sent you this program so they can add your HWID to the ACB database."
            .italic(),
    );

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
}
