#include <malloc.h>
#include <assert.h>
#include <stdio.h>

int main(){
    void* a = malloc(10);
    void* b = malloc(10);
    free(b);
    // a will check if b (the following block) is free, and coaelsh.
    free(a);
    void* c = malloc(15);
    // therefore a == c.
    assert(a == c);
    //printf("Forward cooaleshing worked.\n");
    free(c);
    // now it should be back to a single 20 bytes long block.
    // Let's fill it to "reset" the memory view.
    malloc(20);
    a = malloc(5);
    b = malloc(5);
    // Now the free is inversed. So when we free a, b is still occupied.
    free(a);
    // b can't know if a is free or not because there is no link to previous block.
    free(b);
    // during the malloc, we traverse the blocks and coalesh them.
    // when we visit a, will check if b is free and coalesh blocks.
    c = malloc(10);
    // therefore, a should equal c.
    assert(a == c);
    printf("Backward coaleshing worked.\n");

    return 0;
}