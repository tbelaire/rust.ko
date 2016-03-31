#![feature(custom_attribute, lang_items, core_str_ext)]
#![no_std]

#[macro_use]
extern crate linux_std as std;
use std::os::raw::*;
use std::os::kernel;

// Defines various language items that need to be around
mod lang;

use core::str::StrExt;
use core::ptr;
use core::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

static numberOpens: AtomicUsize = ATOMIC_USIZE_INIT;
// This is unsafe, please make a re-entraint version.
static mut message: [u8; 256] = [0; 256];
static mut size_of_message: usize = 0;

#[no_mangle]
pub fn rust_main() {
    println!("Hello {} from {} Rust!++", 42, 0);
    numberOpens.store(0, Ordering::SeqCst);
}


// I can't currently run bindgen to generate the struct
// definitions for inode, and it's allmost 100 lines long
// and full of ifdefs, so I'm just not passing those arguments ATM.
#[no_mangle]
pub fn rust_dev_open(_inode: *mut kernel::Struct_inode, _file: *mut kernel::Struct_file) -> c_int {
    let old_num_opens = numberOpens.fetch_add(1, Ordering::SeqCst);
    println!("ERChar Device has been opened {} times before.",
             old_num_opens);
    0
}

#[no_mangle]
pub fn rust_dev_release(_inode: *mut kernel::Struct_inode, _file: *mut kernel::Struct_file) -> c_int {
    println!("ERChar Device successfully closed");
    0
}

#[no_mangle]
pub fn rust_dev_read(_file: *mut kernel::Struct_file,
                     buffer: *mut c_char,
                     len: kernel::size_t,
                     offset: *mut kernel::off_t) -> kernel::ssize_t {
    let error_count;

    unsafe {
    error_count = kernel::_copy_to_user(buffer as *mut c_void,
                                        (&message) as *const u8 as *const c_void,
                                        size_of_message as c_uint);
    }
    if error_count == 0 {
        unsafe{
            println!("ERchar: Sent {} characters to the user.", size_of_message);
        }
        unsafe {
            size_of_message = 0;
        }
        return 0;
    } else {
        println!("ERchar: Failed to send {} characters to the user.",
                 error_count);
        return -(std::os::errors::ERROR::EFAULT as kernel::ssize_t);
    }
}

fn rot_13(mut ch: u8) -> u8 {
    if ch >= 'A' as u8 && ch <= 'Z' as u8 {
        ch += 13;
        if ch > 'Z' as u8 {
            ch -= 26;
        }
        ch
    } else if ch >= 'a' as u8 && ch <= 'z' as u8 {
        ch += 13;
        if ch > 'z' as u8 {
            ch -= 26;
        }
        ch
    } else {
        ch
    }
}

#[no_mangle]
pub fn rust_dev_write(_file: *mut kernel::Struct_file,
                      buffer: *const c_char,
                      len: kernel::size_t,
                      offset: *mut kernel::off_t) -> kernel::ssize_t {
    unsafe {
        for i in 0..len as usize{
            message[i] = rot_13(*buffer.offset(i as isize) as u8);
        }
        size_of_message = len as usize;
    }
    println!("ERChar: Rotated {} characters.", len);
    return len as kernel::ssize_t;
}


