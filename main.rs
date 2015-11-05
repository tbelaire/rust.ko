#![feature(core_str_ext)]
#![feature(libc)]
#![feature(no_std)]
#![feature(lang_items)]
#![no_std]

extern crate libc;

mod lang_items;
#[macro_use]
mod macros;
mod raw;
mod kapi;

#[no_mangle]
pub fn rust_main() {
    println!("Hello from Rust!");

    let class = kapi::device::Class::new("rust");

    let device = kapi::fs::CharacterDevice::new(0, 1, cstr!("chardev"));

    kapi::delay::msleep(4*1000);
}
