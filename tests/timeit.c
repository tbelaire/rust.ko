
#include<stdio.h>
#include<stdlib.h>
#include<errno.h>
#include<fcntl.h>
#include<string.h>

#include <sys/time.h>
#include <sys/resource.h>

double get_time() {
    struct timeval t;
    struct timezone tzp;
    gettimeofday(&t, &tzp);
    return t.tv_sec + t.tv_usec * 1e-6;
}

int send_and_receive(char* buffer, int len) {
    int ret, fd;
    fd = open("/dev/erchar", O_RDWR);
    if (fd < 0) {
        return errno;
    }
    ret = write(fd, buffer, len);
    if (ret < 0) {
        return errno;
    }
    ret = read(fd, buffer, len);
    if (ret < 0) {
        return errno;
    }
    return 0;
}

int main(int argc, char** argv) {
    char buffer[] = "abcdefghijklmnopqrstuvwxyz    abcdefghijklmnopqrstuvwxyz    abcdefghijklmnopqrstuvwxyz1234567890";
    double start_time, end_time;
    int ret;
    int i;
    int num_iterations = 1000000;

    start_time = get_time();
    for(i = 0; i < num_iterations; i++) {
        ret = send_and_receive(buffer, sizeof(buffer));
        if (ret < 0) {
            printf("Failed on iteration %x\n", i);
            break;
        }
    }
    end_time = get_time();
    printf("Finished in %f time units after %d iterations", end_time - start_time, num_iterations);
    return 0;
}

