use types::{
    dev_t,
    c_uint,
    c_char,
};
use raw;
use raw::{fs,cdev};

pub struct CharacterDevice {
    dev: dev_t,
    cdev: *mut cdev::cdev,
    count: c_uint,
    first_minor: c_uint
}

impl CharacterDevice {
    pub fn new(first_minor: c_uint, count: c_uint, name: &str) -> CharacterDevice {
        let mut dev = 0;

        unsafe {
            fs::alloc_chrdev_region(&mut dev, first_minor, count, name.as_ptr() as *const c_char);
            let cdev = cdev::cdev_alloc();

            cdev::rustko_extended_cdev_init(
                cdev,
                raw::rustko_this_module(),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
            );

            cdev::cdev_add(cdev, dev, count);

            CharacterDevice {
                dev: dev,
                cdev: cdev,
                count: count,
                first_minor: first_minor,
            }
        }
    }
}

impl Drop for CharacterDevice {
    fn drop(&mut self) {
        unsafe {
            cdev::cdev_del(self.cdev);
            fs::unregister_chrdev_region(self.dev, self.count)
        }
    }
}
