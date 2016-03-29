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
use core::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

static numberOpens: AtomicUsize = ATOMIC_USIZE_INIT;
// This is unsafe, please make a re-entraint version.
static mut message: [u8; 256] = [0; 256];
static mut size_of_message: usize = 0;

#[no_mangle]
pub fn rust_main() {
    println!("Hello  %d from %d Rust!++", 42, 0);
    numberOpens.store(0, Ordering::SeqCst);
}


// I can't currently run bindgen to generate the struct
// definitions for inode, and it's allmost 100 lines long
// and full of ifdefs, so I'm just not passing those arguments ATM.
#[no_mangle]
pub fn rust_dev_open() -> c_int {
    let old_num_opens = numberOpens.fetch_add(1, Ordering::SeqCst);
    println!("ERChar Device has been opened %d times before.",
             old_num_opens);
    0
}

#[no_mangle]
pub fn rust_dev_release() -> c_int {
    println!("ERChar Device successfully closed");
    0
}

#[no_mangle]
pub fn rust_dev_read(buffer: *mut c_char,
                     len: size_t,
                     offset: *mut c_off) -> ssize_t {
    let mut error_count = 0;

    unsafe{
    error_count = raw::my_copy_to_user(buffer as *mut u8,
                                       &message as *const u8,
                                       size_of_message as c_ulong);
    }
    if error_count == 0 {
        println!("ERchar: Sent %d characters to the user", size_of_message);
        unsafe {
            size_of_message = 0;
        }
        return 0;
    } else {
        println!("ERchar: Failed to send %d characters to the user",
                 error_count);
        return -(raw::ERRORS::EFAULT as ssize_t);
    }
}

#[no_mangle]
pub fn rust_dev_write(buffer: *const c_char,
                      len: size_t,
                      offset: *mut c_off) -> ssize_t {
    unsafe {
        message[0] = 'A' as u8;
        size_of_message = 2;
    }
    println!("ERChar: Totally didn't ignore %d chars", len);
    return len as ssize_t;
}





