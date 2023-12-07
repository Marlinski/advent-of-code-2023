#!/bin/bash

for dir in */; do
    binary_name=${dir%/}
    cargo build --release --bin "$binary_name"
done