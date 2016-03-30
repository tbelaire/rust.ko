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
extern void rust_main(void);
extern int rust_dev_open(struct inode*, struct file*);
extern int rust_dev_release(struct inode*, struct file*);
extern ssize_t rust_dev_read(struct file*, char*, size_t, loff_t *);
extern ssize_t rust_dev_write(struct file*, const char*, size_t, loff_t*);

static struct file_operations fops =
{
    .open = rust_dev_open,
    .read = rust_dev_read,
    .write = rust_dev_write,
    .release = rust_dev_release,
};

static int hello_init(void)
{
    printk(KERN_INFO "hello: init\n");
    rust_main();
    majorNumber = register_chrdev(0, DEVICE_NAME, &fops);

    if (majorNumber < 0) {
        printk(KERN_ALERT "ERChar failed to register a major number\n");
        return majorNumber;
    }
    printk(KERN_INFO "ERChar: registered correctly with major number %d\n", majorNumber);

    ercharClass = class_create(THIS_MODULE, CLASS_NAME);
    if (IS_ERR(ercharClass)){
        unregister_chrdev(majorNumber, DEVICE_NAME);
        printk(KERN_ALERT "ERChar failed to register device class\n");
        return PTR_ERR(ercharClass);
    }
    printk(KERN_INFO "ERChar created class correctly\n");

    ercharDevice = device_create(ercharClass, NULL, MKDEV(majorNumber, 0),
            NULL, DEVICE_NAME);

    if (ercharDevice < 0) {
        class_unregister(ercharClass); // ???
        class_destroy(ercharClass);
        unregister_chrdev(majorNumber, DEVICE_NAME);
        printk(KERN_ALERT "ERChar failed to register device class\n");
        return PTR_ERR(ercharDevice);
    }

    printk(KERN_INFO "ERChar: Device created successfully\n");
    return 0;
}

static void hello_exit(void)
{
    printk(KERN_INFO "hello: exit\n");
    device_destroy(ercharClass, MKDEV(majorNumber, 0));
    class_unregister(ercharClass);
    class_destroy(ercharClass);
    unregister_chrdev(majorNumber, DEVICE_NAME);
}

module_init(hello_init);
module_exit(hello_exit);
