#![allow(non_camel_case_types)]
use types::*;
use raw::scsi;
// usb_stor_bulk_trasfer_xxx() return codes
#[repr(C)]
pub enum USB_STOR_XFER {
    GOOD = 0,
    SHORT = 1,
    STALLED = 2,
    LONG = 3,
    ERROR = 4,
}

// Transport return codes.
#[repr(C)]
pub enum USB_STOR_TRANSPORT {
    GOOD = 0,
    FAILED = 1,
    NO_SENSE = 2,
    ERROR = 3,
}

// Void
pub enum us_data {}

extern "C" {
    pub fn usb_stor_bulk_transfer_sglist(
        us: *mut us_data,
        pipe: c_uint,
        sg: *mut scsi::scatterlist,
        num_sg: c_int,
        length: c_uint,
        partial: *mut c_uint) -> c_int;
}
