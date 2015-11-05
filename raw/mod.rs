use libc::{c_char,c_longlong};

pub enum module {}
pub type loff_t = c_longlong; // http://lxr.free-electrons.com/source/include/uapi/asm-generic/posix_types.h#L87

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
