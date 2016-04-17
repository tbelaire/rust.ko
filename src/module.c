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

char __morestack[1024];
char _GLOBAL_OFFSET_TABLE_;

void abort(void)
{
    BUG();
}

unsigned int my_MKDEV(int major, int minor) {
    return MKDEV(major, minor);
}

struct class * my_class_create(char* name) {
    return class_create(THIS_MODULE, name);
}
extern int rust_main(void);
extern void rust_exit(void);

static int hello_init(void)
{
    return rust_main();
}

static void hello_exit(void)
{
    rust_exit();
}

module_init(hello_init);
module_exit(hello_exit);
