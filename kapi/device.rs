use raw::{self, device};

pub struct Class {
    raw: *mut device::class
}

impl Class {
    pub fn new(name: &str) -> Result<Class, ()> {
        let name = name.as_ptr() as *const i8;
        unsafe {
            let class= raw::device::rustko_class_create(raw::rustko_this_module(), name);
            match class.is_null() {
                true => Err(()),
                false => Ok(Class {
                    raw: class
                })
            }
        }
    }
}

impl Drop for Class {
    fn drop(&mut self) {
        unsafe {
            device::class_destroy(self.raw);
        }
    }
}
