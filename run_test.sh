#!/bin/bash

# Commands:
#    --no-cleanup: Don't delete the output directory

# Variables
BUILD_DIR=out
NAME=test
OUT=$BUILD_DIR/$NAME
print_red() {
    echo -e "\n\033[1m\033[31m$1\033[0m"
}
print_blue() {
    echo -e "\n\033[1m\033[34m$1\033[0m"
}

# Build and run the program
cargo run -- test.hy
if [ $? -ne 0 ]; then
    print_red "FAILED TO BUILD COMPILER"
    exit 1
fi
mkdir -p $BUILD_DIR
mv out.asm $OUT.asm

# Print generated assembly
print_blue "OUTPUT ($NAME.asm):"
cat $OUT.asm

# Compile the generated assembly
print_blue "COMPILING OUTPUT ($NAME.asm)"
nasm -f elf64 $OUT.asm -o $OUT.o
if [ $? -ne 0 ]; then
    print_red "FAILED TO COMPILE OUTPUT ($NAME.asm)"
    exit 1
else
    echo "DONE"
fi
print_blue "LINKING OUTPUT ($NAME.o)"
ld -o $OUT $OUT.o
if [ $? -ne 0 ]; then
    print_red "FAILED TO LINK OUTPUT ($NAME.o)"
    exit 1
else
    echo "DONE"
fi

# Run the program
print_blue "RUNNING OUTPUT ($NAME)"
./$OUT
print_blue "EXIT CODE: \033[33m$?"

# Cleanup
if [ "$1" == "--no-cleanup" ]; then
    echo -e "\nCleanup skipped"
    exit 0
else
    if [ -d $BUILD_DIR ]; then
        rm -rf $BUILD_DIR
    fi
fi
