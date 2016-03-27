#![allow(non_camel_case_types)]
use types::*;

pub mod transport;
pub mod scsi;

// Add as needed.
#[repr(C)]
pub enum ERRORS {
    EINVAL = 22
}

extern {
    pub fn printk(fmt: *const c_char, ...);
    pub fn strlen(s: *const c_char) -> c_int;
    pub fn strcpy(dst: *mut c_char, src: *const c_char);
}

