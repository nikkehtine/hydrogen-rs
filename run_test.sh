#!/bin/bash

cargo run -- test.hy
echo
echo "OUTPUT (out.asm):"
cat out.asm
rm out.asm
