#![feature(core_str_ext)]
#![feature(custom_attribute, lang_items)]
#![no_std]

#[macro_use]
mod macros;

// Defines various symbols that need to be around.
mod lang;
// Needs to be publicly exported for some reason?
// Some mismatch, but we are never going to panic anywas, right?
// http://stackoverflow.com/questions/10419801/undefined-reference-to-unwind-resume-and-gxx-personality-v0
// If this is not exported and you use anything that might panic,
// then you get errors when you insmod.
pub use lang::_Unwind_Resume;

mod types;
mod raw;
mod kapi;


#[no_mangle]
pub fn rust_main() {
    println!("Hello from Rust!++");

    let _class = kapi::device::Class::new("rust");

    let _device = kapi::fs::CharacterDevice::new(0, 1, cstr!("chardev"));


    kapi::delay::msleep(4*1000);
}
