# RVM-RISCV

Let's write an riscv hypervisor in Rust from scratch!

Adapted from: [RVM-Tutorial](https://github.com/equation314/RVM-Tutorial)

## Install Build Dependencies

Install [cargo-binutils](https://github.com/rust-embedded/cargo-binutils) to use `rust-objcopy` and `rust-objdump` tools:

```console
$ cargo install cargo-binutils
```

## Build & Run Hypervisor

```console
$ cd hypervisor
$ make run [LOG=warn|info|debug|trace]
......
 ______     ____  __       ____  ___ ____   ______     __
|  _ \ \   / /  \/  |     |  _ \|_ _/ ___| / ___\ \   / /
| |_) \ \ / /| |\/| |_____| |_) || |\___ \| |    \ \ / /
|  _ < \ V / | |  | |_____|  _ < | | ___) | |___  \ V /
|_| \_\ \_/  |_|  |_|     |_| \_\___|____/ \____|  \_/


arch = riscv64
build_mode = release
log_level = warn
......
```
