#![feature(core_str_ext)]
#![feature(custom_attribute, lang_items)]
#![no_std]

#[macro_use]
mod macros;

mod raw;
// Defines various symbols that need to be around.
mod lang;
// Needs to be publicly exported for some reason?
// Some mismatch, but we are never going to panic anywas, right?
// http://stackoverflow.com/questions/10419801/undefined-reference-to-unwind-resume-and-gxx-personality-v0
// If this is not exported and you use anything that might panic,
// then you get errors when you insmod.
pub use lang::_Unwind_Resume;

pub mod types;
use types :: *;

use core::str::StrExt;

#[no_mangle]
pub fn rust_main() {
    println!("Hello  %d from %d Rust!++", 42, 0);
}

#[no_mangle]
pub unsafe fn rust_hello_read_proc(buffer : *const *mut c_char,
                            start: *const *mut c_char,
                            offset: c_off,
                            size: c_int,
                            eof: *mut c_int,
                            data: *mut ()) -> c_int {
    let hello_str = "Hello world!\n\0";
    let len = raw::strlen(hello_str.as_ptr() as *const i8);
    if size < len {
        return - (raw::ERRORS::EINVAL as c_int);
    }
    if offset != 0 {
        return 0;
    }
    raw::strcpy(*buffer, hello_str.as_ptr() as *const i8);
    *eof = 1;

    return 0;
}

#[no_mangle]
pub fn rust_dev_release() -> c_int {
    println!("ERChar Device successfully closed");
    0
}
