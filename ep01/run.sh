#! /bin/bash

qemu-system-x86_64 \
    -L target/x86_64 \
    -bios target/x86_64/bios.bin \
    -hda fat:rw:dist/x86_64
