#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <binary_path>"
  exit 1
fi

"$1"
