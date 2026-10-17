#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>

/*
 * Cheney's copying collection runtime.
 *
 * Object layout (contract with the compiler):
 *   Every heap object is a flat array of 8-byte words:
 *
 *       [ tag | field_0 | field_1 | ... | field_{n-1} ]
 *
 *   The leading `tag` word encodes the object's shape:
 *       bit 0    : 1 = real tag, 0 = forwarding pointer
 *       bits 1-6 : number of fields n
 *       bits 7+  : pointer bitfield; bit (7 + i) is 1 iff field_i is a
 *                  pointer to another heap object.
 *
 *   Values are untagged: Int/Bool/Float/Unit are 8-byte immediates and
 *   Tuple/closure values are raw pointers into the heap. The collector
 *   therefore never inspects values directly; it only follows the
 *   pointer bitfield of each object's tag and the roots on the root
 *   stack (which the compiler guarantees contain only heap pointers).
 *
 * Compiler responsibilities (not implemented here):
 *   * write the tag word (length + pointer bitfield) when allocating,
 *   * push every live pointer-typed variable onto the root stack before
 *     each `collect`, and
 *   * offset field loads/stores by one word to skip the tag.
 */

extern int64_t start(void);

/* --- heap ------------------------------------------------------------ */

#define HEAP_SIZE_BYTES (8 * 1024 * 1024)   /* per semispace */
#define ROOTSTACK_SIZE_BYTES (1024 * 1024)  /* total root stack */

static int64_t *fromspace_begin;
static int64_t *tospace_begin;
static int64_t *tospace_end;

/* The compiler reads `free_ptr` and `fromspace_end` directly, and pushes
 * roots onto the root stack region starting at `rootstack_begin`. */
int64_t *free_ptr;
int64_t *fromspace_end;
int64_t **rootstack_begin;
int64_t **rootstack_end;

static int initialized = 0;

/* --- tag encoding ----------------------------------------------------- */

/* bit 0: 0 => forwarding pointer, 1 => real tag */
#define TAG_FORWARD_MASK 1
/* bits 1-6: field count */
#define TAG_LENGTH_MASK 126 /* 0b1111110 */
#define TAG_LENGTH_RSHIFT 1
/* bits 7+: pointer bitfield */
#define TAG_PTR_BITFIELD_RSHIFT 7

static inline int is_forwarding(int64_t tag) { return !(tag & TAG_FORWARD_MASK); }

static inline int64_t get_length(int64_t tag) {
    return (tag & TAG_LENGTH_MASK) >> TAG_LENGTH_RSHIFT;
}

static inline int64_t get_ptr_bitfield(int64_t tag) {
    return tag >> TAG_PTR_BITFIELD_RSHIFT;
}

/* --- allocation ------------------------------------------------------- */

/* Round a byte count up to a whole number of 8-byte words. */
static inline uint64_t round_to_word(uint64_t bytes) {
    return (bytes + 7) & ~(uint64_t)7;
}

void initialize(void) {
    fromspace_begin = (int64_t *)malloc(HEAP_SIZE_BYTES);
    tospace_begin = (int64_t *)malloc(HEAP_SIZE_BYTES);
    rootstack_begin = (int64_t **)malloc(ROOTSTACK_SIZE_BYTES);

    if (!fromspace_begin || !tospace_begin || !rootstack_begin) {
        fprintf(stderr, "initialize: failed to allocate heap\n");
        exit(EXIT_FAILURE);
    }

    fromspace_end = fromspace_begin + (HEAP_SIZE_BYTES / sizeof(int64_t));
    tospace_end = tospace_begin + (HEAP_SIZE_BYTES / sizeof(int64_t));
    rootstack_end = rootstack_begin + (ROOTSTACK_SIZE_BYTES / sizeof(int64_t *));

    free_ptr = fromspace_begin;
    initialized = 1;
}

/* Bump allocate. The compiler has already emitted the
 * `if free_ptr + bytes < fromspace_end then () else collect(bytes)`
 * check, so we simply advance `free_ptr` and hand back the old value. */
int64_t *allocate(uint64_t bytes) {
    uint64_t words = round_to_word(bytes) / sizeof(int64_t);
    int64_t *result = free_ptr;
    free_ptr += words;
    return result;
}

/* --- collector -------------------------------------------------------- */

static void cheney(int64_t **rootstack_ptr);
static void copy_vector(int64_t **vector_ptr_loc);
static void process_vector(int64_t **scan_addr);

/*
 * copy_vector moves the object referenced by *vector_ptr_loc from
 * fromspace into tospace and updates the reference in place. If the
 * object has already been moved, its old tag word holds a forwarding
 * pointer and we only rewrite the reference.
 */
static void copy_vector(int64_t **vector_ptr_loc) {
    int64_t *old_ptr = *vector_ptr_loc;
    int64_t tag = old_ptr[0];

    if (is_forwarding(tag)) {
        *vector_ptr_loc = (int64_t *)tag;
        return;
    }

    int64_t length = get_length(tag);
    int64_t *new_ptr = free_ptr;

    /* Copy the tag word plus every field. */
    for (int64_t i = 0; i < length + 1; i++) {
        new_ptr[i] = old_ptr[i];
    }
    free_ptr = free_ptr + length + 1;

    /* Leave a forwarding pointer in the old object. */
    old_ptr[0] = (int64_t)new_ptr;
    *vector_ptr_loc = new_ptr;
}

/*
 * process_vector scans the fields of the object at *scan_addr, following
 * the pointer bitfield, and advances *scan_addr to the next object.
 */
static void process_vector(int64_t **scan_addr) {
    int64_t tag = **scan_addr;
    int64_t length = get_length(tag);
    int64_t ptr_bits = get_ptr_bitfield(tag);

    int64_t *next_ptr = *scan_addr + length + 1;
    *scan_addr += 1;

    while (*scan_addr != next_ptr) {
        if (ptr_bits & 1) {
            copy_vector((int64_t **)*scan_addr);
        }
        ptr_bits >>= 1;
        *scan_addr += 1;
    }
}

/*
 * cheney performs a breadth-first copy of all live data into tospace,
 * then swaps the two semispaces.
 */
static void cheney(int64_t **rootstack_ptr) {
    int64_t *scan_ptr = tospace_begin;
    free_ptr = tospace_begin;

    /* Copy the roots to seed the queue. */
    for (int64_t **root_loc = rootstack_begin; root_loc != rootstack_ptr; root_loc++) {
        copy_vector(root_loc);
    }

    /* Scan until we catch up with the free pointer. */
    while (scan_ptr != free_ptr) {
        process_vector(&scan_ptr);
    }

    /* Flip the semispaces. */
    int64_t *tmp = fromspace_begin;
    fromspace_begin = tospace_begin;
    tospace_begin = tmp;

    tmp = fromspace_end;
    fromspace_end = tospace_end;
    tospace_end = tmp;
}

void collect(int64_t **rootstack_ptr, uint64_t bytes_requested) {
    if (!initialized) {
        fprintf(stderr, "collect: heap not initialized\n");
        exit(EXIT_FAILURE);
    }
    if (rootstack_ptr < rootstack_begin || rootstack_ptr > rootstack_end) {
        fprintf(stderr, "collect: rootstack pointer out of range\n");
        exit(EXIT_FAILURE);
    }

    cheney(rootstack_ptr);

    /* If collection still didn't free enough room, grow both semispaces
     * and run the collector again against the larger heap. */
    uint64_t available = (uint64_t)(fromspace_end - free_ptr) * sizeof(int64_t);
    if (available < bytes_requested) {
        uint64_t occupied = (uint64_t)(free_ptr - fromspace_begin) * sizeof(int64_t);
        uint64_t needed = occupied + bytes_requested;
        uint64_t old_bytes = (uint64_t)(fromspace_end - fromspace_begin) * sizeof(int64_t);
        uint64_t new_bytes = old_bytes;

        while (new_bytes < needed) {
            new_bytes *= 2;
        }

        free(tospace_begin);
        tospace_begin = (int64_t *)malloc(new_bytes);
        tospace_end = tospace_begin + new_bytes / sizeof(int64_t);

        cheney(rootstack_ptr);

        /* cheney swapped semispaces; re-allocate the (now old) tospace. */
        free(tospace_begin);
        tospace_begin = (int64_t *)malloc(new_bytes);
        tospace_end = tospace_begin + new_bytes / sizeof(int64_t);
    }
}

/* --- IO --------------------------------------------------------------- */

void print_int(int64_t x) { printf("%ld", x); }
void print_bool(int8_t x) { printf(x ? "true" : "false"); }
void print_float(double x) { printf("%f", x); }
void print_newline(void) { printf("\n"); }

int64_t read_int(void) {
    int64_t n;
    if (scanf("%ld", &n) != 1) {
        fprintf(stderr, "read_int: failed to read an integer\n");
        exit(EXIT_FAILURE);
    }
    return n;
}

double read_float(void) {
    double f;
    if (scanf("%lf", &f) != 1) {
        fprintf(stderr, "read_float: failed to read a float\n");
        exit(EXIT_FAILURE);
    }
    return f;
}

/* --- entry ------------------------------------------------------------ */

int main(void) {
    initialize();
    int64_t r = start();
    (void)r;
    return 0;
}
