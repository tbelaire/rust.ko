#include <linux/bug.h>
#include <linux/device.h>
#include <linux/fs.h>
#include <linux/init.h>
#include <linux/kernel.h>
#include <linux/module.h>
#include <linux/proc_fs.h>
#include <linux/slab.h>
#include <asm/uaccess.h>

#define DEVICE_NAME "erchar"
#define CLASS_NAME "er"

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Theo Belaire");
MODULE_DESCRIPTION("A simple Linux char device for exploring Rust");
MODULE_VERSION("0.1");

static int majorNumber;
static struct class* ercharClass = NULL;
static struct device* ercharDevice = NULL;


char __morestack[1024];
char _GLOBAL_OFFSET_TABLE_;

void abort(void)
{
    BUG();
}

long my_copy_to_user(void __user *to,
        const void* from, unsigned long n)
{
    return copy_to_user(to, from, n);
}
int my_register_chrdev(unsigned int major, const char *name,
        const struct file_operations *fops)
{
    return __register_chrdev(major, 0, 256, name, fops);
}
extern int rust_main(void);

static int hello_init(void)
{
    printk(KERN_INFO "rot13-rust: init\n");
    // majorNumber = register_chrdev(0, DEVICE_NAME, &fops);
    majorNumber = rust_main();

    if (majorNumber < 0) {
        printk(KERN_ALERT "rot13-rust: failed to register a major number\n");
        return majorNumber;
    }
    printk(KERN_INFO "rot13-rust: registered correctly with major number %d\n", majorNumber);

    ercharClass = class_create(THIS_MODULE, CLASS_NAME);
    if (IS_ERR(ercharClass)){
        unregister_chrdev(majorNumber, DEVICE_NAME);
        printk(KERN_ALERT "rot13-rust: failed to register device class\n");
        return PTR_ERR(ercharClass);
    }
    printk(KERN_INFO "rot13-rust: created class correctly\n");

    ercharDevice = device_create(ercharClass, NULL, MKDEV(majorNumber, 0),
            NULL, DEVICE_NAME);

    if (ercharDevice < 0) {
        class_unregister(ercharClass); // ???
        class_destroy(ercharClass);
        unregister_chrdev(majorNumber, DEVICE_NAME);
        printk(KERN_ALERT "rot13-rust: failed to register device class\n");
        return PTR_ERR(ercharDevice);
    }

    printk(KERN_INFO "rot13-rust: Device created successfully\n");
    return 0;
}

static void hello_exit(void)
{
    printk(KERN_INFO "rot13-rust: exit\n");
    device_destroy(ercharClass, MKDEV(majorNumber, 0));
    class_unregister(ercharClass);
    class_destroy(ercharClass);
    unregister_chrdev(majorNumber, DEVICE_NAME);
}

module_init(hello_init);
module_exit(hello_exit);
