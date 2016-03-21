use raw::module;
use types::{c_char};

#[allow(non_camel_case_types)]
pub enum class {}

extern {
    pub fn rustko_class_create(owner: *mut module, name: *const c_char) -> *mut class;
    pub fn class_destroy(cls: *mut class);
}
