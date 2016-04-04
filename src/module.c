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
MODULE_DESCRIPTION("A simple Linux char device for rotating characters");
MODULE_VERSION("0.1");

static int majorNumber;
static struct class* ercharClass = NULL;
static struct device* ercharDevice = NULL;

static int numberOpens;
static char message[256];
static int size_of_message;


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
static int dev_open(struct inode*, struct file*);
static int dev_release(struct inode*, struct file*);
static ssize_t dev_read(struct file*, char*, size_t, loff_t *);
static ssize_t dev_write(struct file*, const char*, size_t, loff_t*);

static struct file_operations fops =
{
    .open = dev_open,
    .read = dev_read,
    .write = dev_write,
    .release = dev_release,
};

static int hello_init(void)
{
    printk(KERN_INFO "rot13C: init\n");
    numberOpens = 0;

    majorNumber = register_chrdev(0, DEVICE_NAME, &fops);

    if (majorNumber < 0) {
        printk(KERN_ALERT "rot13C failed to register a major number\n");
        return majorNumber;
    }
    printk(KERN_INFO "rot13C: registered correctly with major number %d\n", majorNumber);

    ercharClass = class_create(THIS_MODULE, CLASS_NAME);
    if (IS_ERR(ercharClass)){
        unregister_chrdev(majorNumber, DEVICE_NAME);
        printk(KERN_ALERT "rot13C: failed to register device class\n");
        return PTR_ERR(ercharClass);
    }
    printk(KERN_INFO "rot13C: created class correctly\n");

    ercharDevice = device_create(ercharClass, NULL, MKDEV(majorNumber, 0),
            NULL, DEVICE_NAME);

    if (ercharDevice < 0) {
        class_unregister(ercharClass); // ???
        class_destroy(ercharClass);
        unregister_chrdev(majorNumber, DEVICE_NAME);
        printk(KERN_ALERT "rot13C: failed to register device class\n");
        return PTR_ERR(ercharDevice);
    }

    printk(KERN_INFO "rot13C: Device created successfully\n");
    return 0;
}

static void hello_exit(void)
{
    printk(KERN_INFO "rot13C: exit\n");
    device_destroy(ercharClass, MKDEV(majorNumber, 0));
    class_unregister(ercharClass);
    class_destroy(ercharClass);
    unregister_chrdev(majorNumber, DEVICE_NAME);
}

static int dev_open(struct inode* inodep, struct file* filep) {
    numberOpens += 1;
    return 0;
}
static int dev_release(struct inode* inodep, struct file* filep) {
    return 0;
}

static ssize_t dev_read(struct file* filep, char* buffer, size_t len, loff_t * off) {
    int error_count;
    // Don't have enough room to store the message.
    if (len < size_of_message) {
        return -EFAULT;
    }
    error_count = _copy_to_user(buffer, &message, size_of_message);
    if (error_count == 0) {
        // It worked.
        size_of_message = 0;
        return 0;
    }
    // We failed.
    return -EFAULT;
}

unsigned char rot_13(unsigned char ch) {
    if (ch >= 'A' && ch <= 'Z') {
        ch += 13;
        if (ch > 'Z') {
            ch -= 26;
        }
    } else if (ch >= 'a' && ch <= 'z') {
        ch += 13;
        if (ch > 'z') {
            ch -= 26;
        }
    }
    return ch;
}

static ssize_t dev_write(struct file* filep, const char* buffer, size_t len, loff_t* off) {
    int i;
    size_of_message = 0;
    for (i = 0; i < len; i++) {
        message[i] = rot_13(buffer[i]);
    }
    size_of_message = len;
    return len;
}


module_init(hello_init);
module_exit(hello_exit);
