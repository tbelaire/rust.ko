use types::c_uint;
use raw::delay;

pub fn msleep(msecs: c_uint) {
    unsafe {
        delay::msleep(msecs)
    }
}
