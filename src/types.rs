#![allow(non_camel_case_types)]
#![allow(unused_attributes)]

// Detect 32-bit platforms
#![cfg_attr(
	any(
		target_arch = "x86",
		target_arch = "arm",
		target_arch = "mips",
		target_arch = "mipsel",
		target_arch = "powerpc",
		target_arch = "le32"
	),
	target_arch_std32
)]

// Detect 64-bit platforms
#![cfg_attr(
	any(
		target_arch = "x86_64",
		target_arch = "aarch64"
	),
	target_arch_std64
)]

// Special: Platforms with unsigned character types
#![cfg_attr(
	any(
		target_arch = "aarch64"
	),
	target_arch_char_unsigned
)]

enum c_void {
}
// Character types
#[cfg(not(target_arch_char_unsigned))]
pub type c_char  = i8;
#[cfg(target_arch_char_unsigned)]
pub type c_char  = u8;

pub type c_uchar = u8;

#[cfg(not(target_arch_char_unsigned))]
pub type c_wchar = i32;
#[cfg(target_arch_char_unsigned)]
pub type c_wchar = u32;

// Standard integers
pub type c_short = i16; // Check.
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = i32; // TODO check this?
pub type c_ulong = u32; // TODO check this?
pub type c_off = c_long;

// The special "size" type
#[cfg(target_arch_std32)]
pub type size_t = u32;
#[cfg(target_arch_std64)]
pub type size_t = u64;

// This depends on CONFIG_ARCH_DMA_ADDR_T_64BIT
pub type dma_addr_t = u64;
// TODO check.

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct hlist_node {
    next: *mut hlist_node,
    prev: *mut *mut hlist_node,
}

#[repr(C)]
pub struct lockdep_map {
    key: *mut c_void,
    class_cache: [*mut c_void; 2],
    name: *mut c_char,
    // IF CONFIG_LOCK_STAT
    cpu: c_int,
    ip: c_ulong,
    // END IF
}


#[repr(C)]
pub struct timer_list {
    entry: hlist_node,
    expires: c_ulong,
    function: *const fn(c_ulong) -> c_void,
    data: c_ulong,
    flags: u32,
    slack: c_int,
    // IFDEF CONFIG_TIMER_STATS
    start_pid: c_int,
    start_site: *mut c_void,
    start_comm: [c_char; 16],
    // END IF
    // IF CONFIG_LOCKDEP
    lockdep_map: lockdep_map,
    // END IF
}
#[repr(C)]
pub struct work_struct {
    data: /* atomic */ c_long, // TODO
    entry: list_head,
    func: *const fn(),
    // IF CONFIG_LOCKDEP
    lockdep_map: lockdep_map,
    // END IF
}


pub enum workqueue_struct {}

#[repr(C)]
pub struct delayed_work {
    work: work_struct,
    timer: timer_list,

    wq: *mut workqueue_struct,
    cpu: c_int,
}

pub enum request{}
