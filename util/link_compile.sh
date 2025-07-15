#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <source_file.s>"
  exit 1
fi

SOURCE_FILE="$1"
BASENAME=$(basename "$SOURCE_FILE" .s)
BUILD_DIR="../build"

arm-linux-gnueabihf-as -g -o "$BUILD_DIR/${BASENAME}.o" "$SOURCE_FILE"

arm-linux-gnueabihf-ld -o "$BUILD_DIR/$BASENAME" "$BUILD_DIR/${BASENAME}.o" -Ttext=0x10000 --no-dynamic-linker -nostdlib

