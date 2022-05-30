
```
cargo build
LD_PRELOAD=$(readlink -f target/debug/liblibsmalloc.so) ls
$ gcc -Wall -g -O0 main.c -I. -Ltarget/debug/ -lsomelibname

## One command:
cargo build && LD_PRELOAD=$(readlink -f target/debug/libsmalloc.so) ls
```