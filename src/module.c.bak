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
static char message[256] = {0};
static short size_of_message;
static int numberOpens = 0;
static struct class* ercharClass = NULL;
static struct device* ercharDevice = NULL;


char __morestack[1024];
char _GLOBAL_OFFSET_TABLE_;

void abort(void)
{
    BUG();
}

void *malloc(size_t s)
{
    return kmalloc(s, GFP_KERNEL);
}

void free(void *ptr)
{
    kfree(ptr);
}

static int dev_open(struct inode *, struct file *);
static int dev_release(struct inode *, struct file *);
static ssize_t dev_read(struct file *, char*, size_t, loff_t *);
static ssize_t dev_write(struct file *, const char*, size_t, loff_t *);

extern void rust_main(void);
extern int rust_dev_open(void);
extern int rust_dev_release(void);

static struct file_operations fops = 
{
    .open = dev_open,
    .read = dev_read,
    .write = dev_write,
    .release = dev_release,
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

static int dev_open(struct inode *inodep, struct file *filep){
    return rust_dev_open();
    return 0;
}

static ssize_t dev_read(struct file *filep, char *buffer, size_t len, loff_t * offset){
    int error_count = 0;

    error_count = copy_to_user(buffer, message, size_of_message);

    if(error_count == 0){
        printk(KERN_INFO "ERChar: Sent %d characters to the user\n", size_of_message);
        size_of_message = 0;
        return 0;
    } else {
        printk(KERN_INFO "ERChar: Failed to send %d characters to the user\n", error_count);
        return -EFAULT;
    }
}
static ssize_t dev_write(struct file *filep, const char* buffer, size_t len, loff_t *offset){
    sprintf(message, "%s(%zu letters)", buffer, len);
    size_of_message = strlen(message);
    printk(KERN_INFO "ERChar: Received %zu characters from the user\n", len);
    return len;
}
static int dev_release(struct inode* inodep, struct file *filep){
    return rust_dev_release();
}

module_init(hello_init);
module_exit(hello_exit);
