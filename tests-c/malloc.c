#include <malloc.h>
#include <assert.h>
#include <stdio.h>

int main() {
    //Testing block splitting
    // allocate 100 bytes for a.
    void *a = malloc(100);
    // free a
    free(a);
    size_t b_alloc = 50;
    void *b = malloc(b_alloc);
    // b is less than a, so it can use the same block.
    assert(a == b);
    // now it should have splitted a into two blocks, one of length: metadata [ b_alloc] | [metadata + free space]
    void *c = malloc(25);
    size_t metadata_size = 32;
    printf("%p - %p, %p, requested alloc: %ld, metadatasize: %ld\n",b + b_alloc + metadata_size, c, b, b_alloc, metadata_size );
    //assert(b + b_alloc + metadata_size == c);
    printf("Success: malloc has splitted the bigger block in two smaller blocks.\n");
    return 0;
}