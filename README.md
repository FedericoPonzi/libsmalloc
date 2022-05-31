# libsmalloc
A malloc implementation in Rust, suitable for rust and replace libc malloc.

```
# Test it as:
cargo build && LD_PRELOAD=$(readlink -f target/debug/libsmalloc.so) ls
```