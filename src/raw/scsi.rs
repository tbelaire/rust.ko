#![allow(non_camel_case_types)]
use types::*;

#[repr(C)]
pub enum dma_data_direction {
    DMA_BIDIRECTIONAL = 0,
    DMA_TO_DEVICE  = 1,
    DMA_FROM_DEVICE = 2,
    DMA_NONE = 3,
}
// Void
pub enum Scsi_Host{}
pub enum scsi_driver{}
pub enum scsi_device{}

// scatterlist.h
pub enum scatterlist {}
#[repr(C)]
pub struct sg_table {
    sgl: *mut scatterlist,
    nents: c_uint,
    orig_nents: c_uint,
}

#[repr(C)]
pub struct scsi_data_buffer {
    table: sg_table,
    length: c_uint,
    resid: c_int,
}

#[repr(C)]
pub struct scsi_pointer {
    ptr: *mut c_char,
    this_residual: c_int,
    buffer: *mut scatterlist,
    buffers_residual: c_int,

    dma_handle: dma_addr_t,

    // All of these are volatile.
    Status: c_int,
    Message: c_int,
    have_data_in: c_int,
    sent_command: c_int,
    phase: c_int,
}

#[repr(C)]
pub struct scsi_cmnd {
    device: *mut scsi_device,
    list: list_head,
    eh_entry: list_head,
    abort_work: delayed_work,
    eh_eflags: c_int,

    serial_number: c_ulong,

    jiffies_at_alloc: c_ulong,

    retries: c_int,
    allowed: c_int,

    prot_op: c_uchar,
    prot_type: c_uchar,
    prot_flags: c_uchar,

    cmd_len: c_ushort,
    sc_data_direction: dma_data_direction,

    cmnd: *mut c_uchar,

    sdb: scsi_data_buffer,
    prot_sdb: *mut scsi_data_buffer,

    underflow: c_uint,
    transfersize: c_uint,
    request: *mut request,
    sense_buffer: *mut c_uchar, /* 96 = SCSI_SENSE_BUFFERSIZE */

    scsi_done: *const fn(), /* fn pointer */
    SCp: scsi_pointer,
    host_scribble: *mut c_uchar,

    result: c_int,
    flags: c_int,
    tag: c_uchar,
}


pub unsafe fn scsi_sg_count(cmd: *const scsi_cmnd) -> c_uint {
    (*cmd).sdb.table.nents
}
pub unsafe fn scsi_bufflen(cmd: *const scsi_cmnd) -> c_uint {
    (*cmd).sdb.length
}
pub unsafe fn scsi_sglist(cmd: *mut scsi_cmnd) -> *mut scatterlist {
    (*cmd).sdb.table.sgl
}
pub unsafe fn scsi_set_resid(cmd: *mut scsi_cmnd, resid: c_int) {
    (*cmd).sdb.resid = resid;
}
pub unsafe fn scsi_get_resid(cmd: *mut scsi_cmnd) -> c_int {
    (*cmd).sdb.resid
}


    /*
extern "C" {
    pub fn scsi_bufflen(cmd: *const scsi_cmnd) -> c_uint;
    pub fn scsi_sg_count(cmd: *const scsi_cmnd) -> c_uint;
    pub fn scsi_sglist(cmd: *mut scsi_cmnd) -> *mut scatterlist;
    pub fn scsi_set_resid(cmd: *mut scsi_cmnd, resid: c_int);
    pub fn scsi_get_resid(cmd: *mut scsi_cmnd) -> c_int;
}
*/
