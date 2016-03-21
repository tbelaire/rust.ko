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

// Character types
#[cfg(not(target_arch_char_unsigned))]
pub type c_char  = i8;
#[cfg(target_arch_char_unsigned)]
pub type c_char  = u8;
#[cfg(not(target_arch_char_unsigned))]
pub type c_wchar = i32;
#[cfg(target_arch_char_unsigned)]
pub type c_wchar = u32;

// Standard integers
pub type c_int = i32;
pub type c_uint = u32;

// The special "size" type
#[allow(non_camel_case_types)]
#[cfg(target_arch_std32)]
pub type size_t = u32;
#[allow(non_camel_case_types)]
#[cfg(target_arch_std64)]
pub type size_t = u64;

// This is a hack, should have been defined above.
#[allow(non_camel_case_types)]
pub type size_t = u64;

#[allow(non_camel_case_types)]
#[cfg(target_arch_std32)]
pub type ssize_t = i32;
#[allow(non_camel_case_types)]
#[cfg(target_arch_std64)]
pub type ssize_t = i64;

// This is a hack, should have been defined above.
#[allow(non_camel_case_types)]
pub type ssize_t = i64;

#[allow(non_camel_case_types)]
pub type c_long = c_int;
#[allow(non_camel_case_types)]
pub type c_ulong = c_uint;
#[allow(non_camel_case_types)]
pub type c_longlong = i64;
#[allow(non_camel_case_types)]
pub type c_ulonglong = u64;

#[repr(i8)]
#[allow(non_camel_case_types)]
pub enum c_void {
    __varient1,
    __varient2,
}

#[allow(non_camel_case_types)]
pub type dev_t = u64;

#[allow(non_camel_case_types)]
pub type loff_t = c_longlong; // http://lxr.free-electrons.com/source/include/uapi/asm-generic/posix_types.h#L87
