macro_rules! cstr {
    ($arg:expr) => (concat!($arg, "\0"))
}

// These are wildly unsafe, as we use C style format strings,
// and varargs.
macro_rules! print {
    ($str:expr $(, $arg:expr )* ) => ({
        use core::str::StrExt;
        let str = cstr!($str);
        let ptr = str.as_ptr() as *const types::c_char;
        unsafe { raw::printk(ptr, $($arg),* ); }
    });
}

macro_rules! println {
    ($str:expr $(, $arg:expr )* ) => {(
        print!(concat!($str, "\n") $(,$arg)* ))};
}
