#! /bin/bash

docker run -it \
    --platform linux/x86_64 \
    --rm \
    -v $(pwd)/../.cargo:/usr/local/cargo/registry \
    -v $(pwd):/root/env \
    tonyos-buildenv \
    ./buildscript.sh
