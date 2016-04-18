#![feature(custom_attribute, lang_items, core_str_ext, const_fn, alloc)]
#![no_std]

extern crate alloc;
extern crate kmalloc_allocator;


// Defines various language items that need to be around
mod lang;
extern "C" {
    fn printk(fmt: *const i8, ...);
}
#[no_mangle]
pub unsafe extern "C" fn rust_main() -> i32 {
    // Comment this line out and it compiles.
    let x = alloc::boxed::Box::new(32);
    // Comment this line out and it compiles.
    printk("value of x: %d\n\0".as_ptr() as *const _, *x);
    *x
}

#[no_mangle]
pub unsafe extern "C" fn rust_exit() {}
