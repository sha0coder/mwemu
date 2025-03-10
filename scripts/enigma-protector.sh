#!/bin/bash

set -x

export RUST_BACKTRACE=1
export RUST_LOG=debug

# Check if mode parameter is provided
if [ -z "$1" ]; then
    echo "Error: Mode parameter required (dump/load)"
    exit 1
fi

MODE=$1

# Set target architecture based on OS
if [[ "$OSTYPE" == "msys"* ]] || [[ "$OSTYPE" == "cygwin"* ]]; then
    TARGET=x86_64-pc-windows-msvc
else
    TARGET=aarch64-apple-darwin
fi

# Execute based on mode
if [ "$MODE" == "dump" ]; then
    cargo run \
        -p mwemu \
        --release \
        --target $TARGET \
        -- \
        --filename ~/Desktop/enigma/pe_loader-20250122-v2.exe \
        --maps ./maps64/ \
        --64bits \
        -vv
elif [ "$MODE" == "dump_verbose" ]; then
    cargo run \
        -p mwemu \
        --release \
        --target $TARGET \
        -- \
        --filename ~/Desktop/enigma/surprise.dll \
        --trace /tmp/output.csv \
        --maps ./maps64/ \
        --64bits \
        --rcx 0x180000000 \
        --rdx 1 \
        --r8 0 \
        -vvv \
        --memory \
        --regs \
        --exit 1000000
elif [ "$MODE" == "load" ]; then
    cargo run \
        -p mwemu \
        --release \
        --target $TARGET \
        -- \
        --filename ~/Desktop/enigma/surprise.dll \
        --maps ./maps64/ \
        --64bits \
        --rcx 0x180000000 \
        --rdx 1 \
        --r8 0 \
        --dump ./dumps/emu-232321175.bin
elif [ "$MODE" == "load_verbose" ]; then
    cargo run \
        -p mwemu \
        --release \
        --target $TARGET \
        -- \
        --filename ~/Desktop/enigma/surprise.dll \
        --maps ./maps64/ \
        --64bits \
        --dump ./dumps/emu-232321175.bin \
        -vvv \
        --memory \
        --regs \
        -p \
        --banzai \
        --rcx 0x180000000 \
        --rdx 1 \
        --r8 0 \
        --trace /tmp/output.csv
else
    echo "Error: Invalid mode. Use 'dump' or 'load'"
    exit 1
fi
