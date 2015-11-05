use libc::*;
use super::{
    module,
    loff_t
};
use super::fs::{
    file,
    seq_file,
    kiocb,
    dir_context,
    inode,
    fl_owner_t,
    file_lock
};
use super::uio::iov_iter;
use super::poll::poll_table_struct;
use super::mm_types::vm_area_struct;
use super::kernel::page;
use super::pipe_fs_i::pipe_inode_info;

pub enum cdev {}
pub enum file_operations {}

extern {
    pub fn cdev_alloc() -> *mut cdev;
    pub fn cdev_init(cdev: *mut cdev, fops: *const file_operations);
    pub fn cdev_add(cdev: *mut cdev, dev: dev_t, count: c_uint) -> c_int;
    pub fn cdev_del(cdev: *mut cdev);
    pub fn rustko_extended_cdev_init(
        cdev: *mut cdev,
        owner: *mut module,
        llseek: Option<extern fn(*mut file, loff_t, c_int) -> loff_t>,
        read: Option<extern fn(*mut file, /* __user */ *mut char, size_t, *mut loff_t) -> ssize_t>,
        write: Option<extern fn(*mut file, /* __user */ *const char, size_t, *mut loff_t) -> ssize_t>,
        read_iter: Option<extern fn(*mut kiocb, *mut iov_iter) -> ssize_t>,
        write_iter: Option<extern fn(*mut kiocb, *mut iov_iter) -> ssize_t>,
        iterate: Option<extern fn(*mut file, *mut dir_context) -> c_int>,
        poll: Option<extern fn(*mut file, *mut poll_table_struct) -> c_uint>,
        unlocked_ioctl: Option<extern fn(*mut file, c_uint, c_ulong) -> c_long>,
        compat_ioctl: Option<extern fn(*mut file, c_uint, c_ulong) -> c_long>,
        mmap: Option<extern fn(*mut file, *mut vm_area_struct) -> c_int>,
        mremap: Option<extern fn(*mut file, *mut vm_area_struct) -> c_int>,
        open: Option<extern fn(*mut inode, *mut file) -> c_int>,
        flush: Option<extern fn(*mut file, id: fl_owner_t) -> c_int>,
        release: Option<extern fn(*mut inode, *mut file) -> c_int>,
        fsync: Option<extern fn(*mut file, loff_t, loff_t, datasync: c_int) -> c_int>,
        aio_fsync: Option<extern fn(*mut kiocb, datasync: c_int) -> c_int>,
        fasync: Option<extern fn(c_int, *mut file, c_int) -> c_int>,
        lock: Option<extern fn(*mut file, c_int, *mut file_lock) -> c_int>,
        sendpage: Option<extern fn(*mut file, *mut page, c_int, size_t, *mut loff_t, c_int) -> ssize_t>,
        get_unmapped_area: Option<extern fn(*mut file, c_ulong, c_ulong, c_ulong, c_ulong) -> c_ulong>,
        check_flags: Option<extern fn(c_int) -> c_int>,
        lock: Option<extern fn(*mut file, c_int, *mut file_lock) -> c_int>,
        splice_write: Option<extern fn(*mut pipe_inode_info, *mut file, *mut loff_t, size_t, c_uint) -> ssize_t>,
        splice_read: Option<extern fn(*mut file, *mut loff_t, *mut pipe_inode_info, size_t, c_uint) -> ssize_t>,
        setlease: Option<extern fn(*mut file, c_long, *mut *mut file_lock, *mut *mut c_void) -> c_int>,
        fallocate: Option<extern fn(file: *mut file, mode: c_int, offset: loff_t, len: loff_t) -> c_long>,
        show_fdinfo: Option<extern fn(m: *mut seq_file, f: *mut file)>
    );
}