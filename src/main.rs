#![no_std]
#![no_main]
#![feature(start)]

use nds_sys;
use nds_sys::{consoleDemoInit, print, println};
extern crate alloc;
extern crate nds;

#[no_mangle]
#[start]
fn main(argc: isize, argv: *const *const u8) -> isize {
    unsafe {
        let tiny_vec = alloc::vec![1, 2, 3];

        consoleDemoInit();
        println!("Hello, from Rust!");
        println!("{:?}", tiny_vec);
        loop {}
    }
}
