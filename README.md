## A Rust implementation of the LC3 Virtual Machine

- Following this blog which implements the virtual machine in C - [Justin Meiners Blog](https://www.jmeiners.com/lc3-vm/), this implementation attempts to write the same in Rust as accurately as possible.

### Steps to run the program
- Build the repo 
1. Run `make build`
This will build and generate a binary in your `<local/path/to/repo>/target/debug/rusty_lc3` path.
Use this binary and provide one of the following games as input.
`./target/debug/rusty_lc3 <path/to/games/rogue.obj>`

- Download one of the following binaries, these are the games that run on top this virtual machine
1. [2048 Game](https://www.jmeiners.com/lc3-vm/supplies/2048.obj)
2. [Rogue Game](https://www.jmeiners.com/lc3-vm/supplies/rogue.obj)

