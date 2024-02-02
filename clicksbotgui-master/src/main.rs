#![cfg_attr(not(debug_assertions), deny())] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

//hide console window on line 91
use std::arch::global_asm;

mod app;
mod clicksgen;
mod hwid;
mod macro_parser;

global_asm!(
    "
set_statics:
  call get_code_start
  mov rcx, rax
  call set_code_start
  call get_code_length
  mov rcx, rax
  call set_code_length
  ret
"
);

extern "C" {
    fn set_statics();
}

#[no_mangle]
#[link_section = ".stub"]
pub extern "C" fn get_code_start() -> usize {
    0x1818181818181818
}

#[no_mangle]
#[link_section = ".stub"]
pub extern "C" fn get_code_length() -> usize {
    0x1919191919191919
}

#[no_mangle]
#[link_section = ".stub"]
pub unsafe extern "C" fn set_code_start(x: usize) {
    CODE_START = x;
}

#[no_mangle]
#[link_section = ".stub"]
pub unsafe extern "C" fn set_code_length(x: usize) {
    CODE_LENGTH = x;
}

#[no_mangle]
#[link_section = ".stub"]
pub unsafe fn decrypt() {
    set_statics();
    let code_start = CODE_START as *mut u8;
    let code_length = CODE_LENGTH;
    let key = KEY;
    let mut key_index = 0;
    for code_index in 0..code_length {
        let ptr = code_start.add(code_index);
        let key_byte = key[key_index];
        *ptr ^= key_byte;
        key_index += 1;
        if key_index >= 16 {
            key_index = 0;
        }
    }
}

extern "stdcall" {
    fn mainCRTStartup() -> !;
}

#[no_mangle]
pub unsafe extern "C" fn startup() -> ! {
    decrypt();
    mainCRTStartup();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    const ICON_BYTES: &[u8] = include_bytes!("../icon.ico");

    let image = image::load_from_memory_with_format(ICON_BYTES, image::ImageFormat::Ico).unwrap();

    let icon_data = eframe::IconData {
        rgba: image.as_rgba8().unwrap().to_vec(),
        width: image.width(),
        height: image.height(),
    };

    let options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::vec2(650.0, 280.0)),
        always_on_top: true,
        icon_data: Some(icon_data),
        ..Default::default()
    };
    hide_console_window();
    eframe::run_native(
        "clicksbotgui",
        options,
        Box::new(|_| Box::new(clicksbotgui::ClicksbotGUI::default())),
    )
    .unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
fn hide_console_window() {
    use std::ptr;
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::{ShowWindow, SW_HIDE};

    let window = unsafe { GetConsoleWindow() };
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
    if window != ptr::null_mut() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }
}

#[no_mangle]
#[used]
#[link_section = ".stub"]
pub static mut CODE_LENGTH: usize = 0;

#[no_mangle]
#[used]
#[link_section = ".stub"]
pub static mut CODE_START: usize = 0;

#[used]
#[link_section = ".stub"]
pub static KEY: [u8; 16] = [0x20; 16];
