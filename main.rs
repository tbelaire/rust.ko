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

#[no_mangle]
pub fn rust_main() {
    println!("Hello from Rust!");
}
