#![feature(custom_attribute, lang_items)]
#![no_std]

#[macro_use]
extern crate linux_std as std;

// Defines various language items that need to be around
mod lang;
use core::str::StrExt;
use core::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

static numberOpens: AtomicUsize = ATOMIC_USIZE_INIT;

#[no_mangle]
pub fn rust_main() {
    println!("scsi_cmnd is %d bytes in Rust",
             core::mem::size_of::<raw::scsi::scsi_cmnd>());
    numberOpens.store(0, Ordering::SeqCst);
}

/*
#[no_mangle]
pub fn rust_bulk_transfer_buf(
    us : *mut us_data,
    pipe: c_uint,
    buf: *mut c_void,
    length: c_uint,
    act_len: *mut c_uint) -> raw::transport::USB_STOR_XFER {
}
*/

#[no_mangle]
pub unsafe fn rust_bulk_srb(
    us: *mut ::us_data,
    pipe: c_uint,
    srb: *mut raw::scsi::scsi_cmnd) -> c_int{

    // let mut partial: c_uint = 0;
    // let result = raw::transport::usb_stor_bulk_transfer_sglist(
    //     us, pipe,
    //     raw::scsi::scsi_sglist(srb),
    //     raw::scsi::scsi_sg_count(srb) as c_int,
    //     raw::scsi::scsi_bufflen(srb),
    //     &mut partial as *mut c_uint);
   // raw::scsi::scsi_set_resid(
    //    srb, (raw::scsi::scsi_bufflen(srb) - partial) as c_int);
   // return result;
   return 0;
}


