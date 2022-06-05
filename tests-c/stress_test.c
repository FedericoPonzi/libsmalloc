#include <malloc.h>
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#define MAX 10000
#define MIN 0

int main(){
    int i;
    srand(time(NULL));
    int total_allocated = 0;
    for(i = 0; i< 10000; i++ ){
        int a_alloc = rand()%((MAX+1)-MIN) + MIN;
        int b_alloc = rand()%((MAX+1)-MIN) + MIN;
        void* a = malloc(a_alloc);
        void* b = malloc(b_alloc);
        free(a);
        free(b);
        total_allocated += a_alloc+b_alloc;
    }
    printf("Stress test completed, total_allocated: %d.\n", total_allocated);
    return 0;
}