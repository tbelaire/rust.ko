#![feature(custom_attribute, lang_items, alloc)]
#![no_std]

extern crate alloc;
extern crate kmalloc_allocator;


// Defines various language items that need to be around
mod lang;
extern "C" {
    fn foo(x: u32);
}
pub fn main() {
    // Comment this line out and it compiles.
    let x = alloc::boxed::Box::new(32);
    unsafe {
        foo(*x);
    }
}
