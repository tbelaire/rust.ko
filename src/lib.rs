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
