use types::{c_char};

#[allow(non_camel_case_types)]
pub enum module {}

extern {
    pub fn printk(fmt: *const c_char);
    pub fn rustko_this_module() -> *mut module;
}

pub mod fs;
pub mod cdev;
pub mod uio;
pub mod poll;
pub mod mm_types;
pub mod kernel;
pub mod pipe_fs_i;
pub mod device;
pub mod delay;
