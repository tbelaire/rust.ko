use types::c_uint;

extern {
    pub fn msleep(msecs: c_uint);
}
