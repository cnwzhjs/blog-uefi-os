#! /bin/bash

# build the kernel
cargo build --target x86_64-unknown-uefi || exit 1

# copy the built kernel to the dist directory
mkdir -p dist/x86_64/EFI/BOOT || exit 1

cp target/x86_64-unknown-uefi/debug/tonyos.efi \
   dist/x86_64/EFI/BOOT/BOOTX64.EFI || exit 1
