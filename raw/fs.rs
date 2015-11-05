use libc::*;

pub enum file {}
pub enum seq_file {}
pub enum kiocb {}
pub enum dir_context {}
pub enum inode {}
pub type fl_owner_t = *mut c_void; // http://lxr.free-electrons.com/source/include/linux/fs.h#L922
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
