#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(start)]

use nds_sys;
use nds_sys::{consoleDemoInit, printf};
use core::panic::PanicInfo;

macro_rules! c_str {
    ($s:expr) => {{
        concat!($s, "\0").as_ptr()
    }};
}

#[no_mangle]
#[start]
fn main(argc: isize, argv: * const* const u8) -> isize {
    unsafe {
        consoleDemoInit();
        printf(c_str!("Hello, from Rust!"));
        loop {
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
