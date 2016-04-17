#![feature(custom_attribute, lang_items, core_str_ext, const_fn)]
#![no_std]

#[macro_use]
extern crate linux_std as std;
use std::os::raw::*;
use std::os::kernel;

// Defines various language items that need to be around
mod lang;

use core::str::StrExt;
use core::mem;
use core::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

static NUMBER_OPENS: AtomicUsize = ATOMIC_USIZE_INIT;
// This is unsafe, please make a re-entraint version.
static mut message: [u8; 256] = [0; 256];
static mut size_of_message: usize = 0;
static mut major_number: c_int = 0;
static mut fops: kernel::Struct_file_operations =
    kernel::Struct_file_operations {
        read: Some(rust_dev_read),
        write: Some(rust_dev_write),
        open: Some(rust_dev_open),
        release: Some(rust_dev_release),
        owner: core::ptr::null_mut(),
        llseek: None,
        read_iter: None,
        write_iter: None,
        iterate: None,
        poll: None,
        unlocked_ioctl: None,
        compat_ioctl: None,
        mmap: None,
        flush: None,
        fsync: None,
        aio_fsync: None,
        fasync: None,
        lock: None,
        sendpage: None,
        get_unmapped_area: None,
        check_flags: None,
        flock: None,
        splice_write: None,
        splice_read: None,
        setlease: None,
        fallocate: None,
        show_fdinfo: None,
    };

// No CStr, so manually null terminate these strings :/
const DEVICE_NAME: &'static str = "erchar\0";
const CLASS_NAME: &'static str = "er\0";

static mut erchar_class: *mut kernel::Struct_class = core::ptr::null_mut();
static mut erchar_device: *mut kernel::Struct_device = core::ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn rust_main() -> c_int {
    println!("rot13-rust: initializing");
    NUMBER_OPENS.store(0, Ordering::SeqCst);
    major_number = my_register_chrdev(0,
                                      DEVICE_NAME.as_ptr() as *const c_char,
                                      &fops);
    if major_number < 0 {
        // KERN_ALERT
        println!("rot13-rust: failed to register a major number");
        return major_number;
    }
    // KERN_INFO
    println!("rot13-rust: registered correctly with major number {}",
             major_number);

    erchar_class = my_class_create(CLASS_NAME.as_ptr() as *const c_char);
    if IS_ERR_VALUE(erchar_class as c_long) {
        my_unregister_chrdev(major_number as c_uint,
                             DEVICE_NAME.as_ptr() as *const c_char);
        println!("rot13-rust: failed to register device class");
        return PTR_ERR(erchar_class) as c_int;
    }
    println!("rot13-rust: created class correctly");
    erchar_device =
        kernel::device_create(erchar_class,
                              core::ptr::null_mut(),
                              my_MKDEV(major_number, 0),
                              core::ptr::null_mut(),
                              DEVICE_NAME.as_ptr() as *const c_char);

    if (erchar_device as usize) < 0 {
        kernel::class_unregister(erchar_class); // ???
        kernel::class_destroy(erchar_class);
        my_unregister_chrdev(major_number as c_uint,
                             DEVICE_NAME.as_ptr() as *const c_char);
        println!("rot13-rust: failed to register device class\n");
        return PTR_ERR(erchar_device) as c_int;
    }
    println!("rot13-rust: registered device correctly");
    0
}

#[no_mangle]
pub unsafe extern "C" fn rust_exit() {
    println!("rot13-rust: exit");
    kernel::device_destroy(erchar_class, my_MKDEV(major_number, 0));
    kernel::class_unregister(erchar_class); // ???
    kernel::class_destroy(erchar_class);
    my_unregister_chrdev(major_number as c_uint,
                         DEVICE_NAME.as_ptr() as *const c_char);
}


// I can't currently run bindgen to generate the struct
// definitions for inode, and it's allmost 100 lines long
// and full of ifdefs, so I'm just not passing those arguments ATM.
#[no_mangle]
pub unsafe extern "C" fn rust_dev_open(_inode: *mut kernel::Struct_inode,
                                       _file: *mut kernel::Struct_file)
                                       -> c_int {
    let old_num_opens = NUMBER_OPENS.fetch_add(1, Ordering::SeqCst);
    println!("rot13-rust: Device has been opened {} times before.",
             old_num_opens);
    0
}

#[no_mangle]
pub unsafe extern "C" fn rust_dev_release(_inode: *mut kernel::Struct_inode,
                                          _file: *mut kernel::Struct_file)
                                          -> c_int {
    println!("rot13-rust: Device successfully closed");
    0
}

#[no_mangle]
pub unsafe extern "C" fn rust_dev_read(_file: *mut kernel::Struct_file,
                                       buffer: *mut c_char,
                                       _len: kernel::size_t,
                                       _offset: *mut kernel::off_t)
                                       -> kernel::ssize_t {
    let error_count;

    error_count =
        kernel::_copy_to_user(buffer as *mut c_void,
                              (&message) as *const u8 as *const c_void,
                              size_of_message as c_uint);
    if error_count == 0 {
        println!("rot13-rust:: Sent {} characters to the user.",
                 size_of_message);
        size_of_message = 0;
        return 0;
    } else {
        println!("rot13-rust:: Failed to send {} characters to the user.",
                 error_count);
        return -(std::os::errors::ERROR::EFAULT as kernel::ssize_t);
    }
}

fn rot_13(mut ch: u8) -> u8 {
    if ch >= 'A' as u8 && ch <= 'Z' as u8 {
        ch += 13;
        if ch > 'Z' as u8 {
            ch -= 26;
        }
        ch
    } else if ch >= 'a' as u8 && ch <= 'z' as u8 {
        ch += 13;
        if ch > 'z' as u8 {
            ch -= 26;
        }
        ch
    } else {
        ch
    }
}

#[no_mangle]
pub unsafe extern "C" fn rust_dev_write(_file: *mut kernel::Struct_file,
                                        buffer: *const c_char,
                                        len: kernel::size_t,
                                        _offset: *mut kernel::off_t)
                                        -> kernel::ssize_t {
    for i in 0..len as usize {
        message[i] = rot_13(*buffer.offset(i as isize) as u8);
    }
    size_of_message = len as usize;
    println!("rot13-rust: Rotated {} characters.", len);
    return len as kernel::ssize_t;
}

// This was normally inlined
unsafe fn my_register_chrdev(major: c_uint,
                             name: *const c_char,
                             fops_: &kernel::Struct_file_operations)
                             -> c_int {
    kernel::__register_chrdev(major, 0, 256, name, fops_ as *const _)
}
unsafe fn my_unregister_chrdev(major: c_uint, name: *const c_char) {
    kernel::__unregister_chrdev(major, 0, 256, name);
}

extern "C" {
    fn my_class_create(name: *const c_char) -> *mut kernel::Struct_class;
    fn my_MKDEV(major: c_int, minor: c_int) -> c_uint;
}

fn IS_ERR_VALUE(x: c_long) -> bool {
    x >= (-4095 as c_long)
}
fn PTR_ERR<T>(ptr: *const T) -> c_long {
    ptr as c_long
}
