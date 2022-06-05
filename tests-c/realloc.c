#include <malloc.h>
#include <assert.h>
#include <stdio.h>

int main(){
    void* a = malloc(10);
    void* b = malloc(10);
    free(b);
    void* c = realloc(a, 20);
    assert(a == c);
    printf("Success: realloc has extended the allocation instead of moved.\n");
    return 0;
}