RUST_ROOT :=

-include ./config.mk

RC := rustc
RCFLAGS := -O -C code-model=kernel -C relocation-model=static


KER = $(shell uname -r)
OBJ = hello
RMODS = macros.rs lang_items.rs raw/*.rs

obj-m = ${OBJ}.o
hello-objs := stub.o main.o

all: ${OBJ}.ko

${OBJ}.ko: stub.c main.o
	make -C /lib/modules/$(KER)/build M=$(PWD) modules

%.o: %.rs ${RMODS}
	$(RC) $(RCFLAGS) --crate-type lib -o $@ --emit obj $<

insmod:
	sudo insmod ${OBJ}.ko
	dmesg | tail

rmmod:
	sudo rmmod ${OBJ}
	dmesg | tail

clean:
	make -C /lib/modules/$(KER)/build M=$(PWD) clean

test: ${OBJ}.ko
	sudo insmod ${OBJ}.ko
	sudo rmmod ${OBJ}
	dmesg | tail -3

.PHONY: all clean insmod rmmod test
