use types::*;

// Add as needed.
#[repr(C)]
pub enum ERRORS {
    EFAULT = 14,
    EINVAL = 22,
}

extern {
    pub fn printk(fmt: *const c_char, ...);
    pub fn strlen(s: *const c_char) -> c_int;
    pub fn strcpy(dst: *mut c_char, src: *const c_char);

    pub fn my_copy_to_user(to: *mut u8, from: *const u8, n: c_ulong) -> c_long;
}
