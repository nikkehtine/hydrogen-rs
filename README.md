# Hydrogen

Shamelessly stolen from [@orosmatthew](https://github.com/orosmatthew/hydrogen-cpp) - [Creating a compiler](https://www.youtube.com/playlist?list=PLUDlas_Zy_qC7c5tCgTMYq2idyyT241qs) (C++)

## Building, testing, running

⚠ Only tested on Linux

**My cool script**

```sh
./run_test.sh
```

- `--no-cleanup` - don't remove output files

**The boring way**

```sh
cargo run -- test.hy
nasm -f elf64 -o out.o out.asm
ld -o out out.o
./out
```

```sh
cargo build --release
./target/release/hydro test.hy
```

## Useful links

- [Reading a file (Rust Book)](https://doc.rust-lang.org/book/ch12-02-reading-a-file.html)
- [Rust String type (Rust Docs)](https://doc.rust-lang.org/std/string/struct.String.html)
- [Defining Structs (Rust Book)](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [Defining an Enum (Rust Book)](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
- [How to allow optional fields in struct? (comment by Alice Ryhl on Reddit)](https://www.reddit.com/r/rust/comments/pv38v0/comment/he79jlu/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button)
- [Rust Option type (Rust Docs)](https://doc.rust-lang.org/std/option/)
- [while let (Rust By Example)](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)
