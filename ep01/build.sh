#! /bin/bash

docker run -it \
    --platform linux/x86_64 \
    --rm \
    -v $(pwd):/root/env \
    tonyos-buildenv \
    ./buildscript.sh
