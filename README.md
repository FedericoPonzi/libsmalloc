# libsmalloc
A malloc implementation in Rust, suitable for rust and replace libc malloc.

```
# Test it as:
cargo build && LD_PRELOAD=$(readlink -f target/debug/libsmalloc.so) ls
```

## TODO:
* Fix thread-safety
* defer allocations larger than a page size to mmap
* set errno on failure.
* allocate more space (bigger sbrk) in advance.
* Dellocate memory on free if possible.
* 
----
# C tests
I've also included some small c programs to test the behaviours. Nothing fancy, I run them like this:
```
(cd ../ && cargo build ) && gcc malloc.c && LD_PRELOAD=$(readlink -f ../target/debug/libsmalloc.so) ./a.out 

# run all:
(cd ../ && cargo build )
for i in *.c ; do 
     gcc $i && LD_PRELOAD=$(readlink -f ../target/debug/libsmalloc.so) ./a.out 
     echo "----------"
done

```
