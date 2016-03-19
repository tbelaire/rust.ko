use types;

// Add as needed.
#[repr(C)]
pub enum ERRORS {
    EINVAL = 22
}

extern {
    pub fn printk(fmt: *const types::c_char, ...);
    pub fn strlen(s: *const types::c_char) -> types::c_int;
    pub fn strcpy(dst: *mut types::c_char, src: *const types::c_char);
}
