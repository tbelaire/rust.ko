use types::*;
#[allow(non_camel_case_types)]
pub enum file {}
#[allow(non_camel_case_types)]
pub enum seq_file {}
#[allow(non_camel_case_types)]
pub enum kiocb {}
#[allow(non_camel_case_types)]
pub enum dir_context {}
#[allow(non_camel_case_types)]
pub enum inode {}
#[allow(non_camel_case_types)]
pub type fl_owner_t = *mut c_void; // http://lxr.free-electrons.com/source/include/linux/fs.h#L922
#[allow(non_camel_case_types)]
pub enum file_lock {}

extern {
    pub fn alloc_chrdev_region(
        device: *mut dev_t,
        first_minor: c_uint,
        count: c_uint,
        name: *const c_char
    ) -> c_int;
    pub fn unregister_chrdev_region(device: dev_t, count: c_uint);
}
