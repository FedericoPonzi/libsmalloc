# libsmalloc
A malloc implementation in Rust, suitable for rust and replace libc malloc.

```
# Test it as:
cargo build && LD_PRELOAD=$(readlink -f target/debug/libsmalloc.so) ls
```

## TODO:
* Fix thread-safety
* Split blocks in sub-blocks
* join adjacent freed blocks
* defer allocations larger than a page size to mmap
* fix realloc to properly extend the current allocation chunk if possible.
* set errno on failure.
* allocate more space (bigger sbrk) in advance.
* Dellocate memory on free if possible.
* 