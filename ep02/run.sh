#! /bin/bash

qemu-system-x86_64 \
    -smp 8,sockets=2,cores=2,threads=2,maxcpus=8 \
    -L target/x86_64 \
    -bios target/x86_64/bios.bin \
    -hda fat:rw:dist/x86_64
