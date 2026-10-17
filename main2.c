#include <stdint.h>
extern int64_t foo(void);
extern int64_t use_pseudo(void);
extern int64_t *free_ptr;
int main(void){ return use_pseudo()==0 ? 0 : 1; }
