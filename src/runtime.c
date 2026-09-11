#include <stdio.h>
#include <stdlib.h>

extern long start(void);

void print_int(long x)    { printf("%ld", x); }
void print_bool(long x)   { printf(x ? "true" : "false"); }
void print_float(double x){ printf("%f", x); }
void print_newline(void)  { printf("\n"); }

void *allocate(long bytes) { return malloc(bytes); }
int main(void) {
    long r = start();
    return 0;
}