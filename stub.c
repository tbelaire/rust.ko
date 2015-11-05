#include <linux/init.h>
#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/slab.h>
#include <linux/bug.h>
#include <linux/fs.h>
#include <linux/cdev.h>

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

extern void rust_main(void);

static int hello_init(void)
{
    printk(KERN_INFO "hello: init\n");
    rust_main();
    return 0;
}

static void hello_exit(void)
{
    printk(KERN_INFO "hello: exit\n");
}

module_init(hello_init);
module_exit(hello_exit);

MODULE_LICENSE("Dual MIT/GPL");

void rustko_extended_cdev_init(
    struct cdev * cdev,
    struct module *owner,
    loff_t (*llseek) (struct file *, loff_t, int),
    ssize_t (*read) (struct file *, char __user *, size_t, loff_t *),
    ssize_t (*write) (struct file *, const char __user *, size_t, loff_t *),
    ssize_t (*read_iter) (struct kiocb *, struct iov_iter *),
    ssize_t (*write_iter) (struct kiocb *, struct iov_iter *),
    int (*iterate) (struct file *, struct dir_context *),
    unsigned int (*poll) (struct file *, struct poll_table_struct *),
    long (*unlocked_ioctl) (struct file *, unsigned int, unsigned long),
    long (*compat_ioctl) (struct file *, unsigned int, unsigned long),
    int (*mmap) (struct file *, struct vm_area_struct *),
    int (*mremap)(struct file *, struct vm_area_struct *),
    int (*open) (struct inode *, struct file *),
    int (*flush) (struct file *, fl_owner_t id),
    int (*release) (struct inode *, struct file *),
    int (*fsync) (struct file *, loff_t, loff_t, int datasync),
    int (*aio_fsync) (struct kiocb *, int datasync),
    int (*fasync) (int, struct file *, int),
    int (*lock) (struct file *, int, struct file_lock *),
    ssize_t (*sendpage) (struct file *, struct page *, int, size_t, loff_t *, int),
    unsigned long (*get_unmapped_area)(struct file *, unsigned long, unsigned long, unsigned long, unsigned long),
    int (*check_flags)(int),
    int (*flock) (struct file *, int, struct file_lock *),
    ssize_t (*splice_write)(struct pipe_inode_info *, struct file *, loff_t *, size_t, unsigned int),
    ssize_t (*splice_read)(struct file *, loff_t *, struct pipe_inode_info *, size_t, unsigned int),
    int (*setlease)(struct file *, long, struct file_lock **, void **),
    long (*fallocate)(struct file *file, int mode, loff_t offset, loff_t len),
    void (*show_fdinfo)(struct seq_file *m, struct file *f)
) {
    struct file_operations fops = {
        .owner = owner,
        .llseek = llseek,
        .read = read,
        .write = write,
        .read_iter = read_iter,
        .write_iter = write_iter,
        .iterate = iterate,
        .poll = poll,
        .unlocked_ioctl = unlocked_ioctl,
        .mmap = mmap,
        .mremap = mremap,
        .open = open,
        .flush = flush,
        .release = release,
        .fsync = fsync,
        .aio_fsync = aio_fsync,
        .fasync = fasync,
        .lock = lock,
        .sendpage = sendpage,
        .get_unmapped_area = get_unmapped_area,
        .check_flags = check_flags,
        .flock = flock,
        .splice_write = splice_write,
        .splice_read = splice_read,
        .setlease = setlease,
        .fallocate = fallocate,
        .show_fdinfo = show_fdinfo
    };

    cdev_init(cdev, &fops);
}

struct module* rustko_this_module(void) {
    return THIS_MODULE;
}
