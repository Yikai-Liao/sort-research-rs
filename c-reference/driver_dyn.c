// Driver for the (locally patched) reference implementation: same xorshift fill
// as benchmark.c, block size selectable between the reference's fixed 512 and the
// Rust port's policy.
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "radixsort.h"

void permuted_set_block_size(size_t block);

static const uint64_t seed = 0xdeadbeefc0febabe;
static uint64_t cksum;

static void fill_array(uint32pair *buf, size_t n) {
    size_t i;
    uint64_t a = seed, sum = 0;

    for (i = 0; i < n; i++) {
        buf[i][0] = (uint32_t)(a >> 32);
        buf[i][1] = (uint32_t)a;
        sum += a;

        a ^= a << 13;
        a ^= a >> 7;
        a ^= a << 17;
    }
    cksum = sum;
}

// Same policy the Rust port uses: b = max(ceil(4*sqrt(9n / (2*R*s))), floor(1024/s), 1)
static size_t policy_block(size_t n, size_t record_bytes) {
    const double radix = 256.0;
    double balanced = sqrt(9.0 * (double)n / (2.0 * radix * (double)record_bytes));
    size_t scaled = (size_t)ceil(4.0 * balanced);
    size_t floor_blk = 1024 / record_bytes;
    size_t b = scaled > floor_blk ? scaled : floor_blk;
    return b ? b : 1;
}

static int cmp_u64(const void *a, const void *b) {
    uint64_t x = *(const uint64_t *)a, y = *(const uint64_t *)b;
    return (x > y) - (x < y);
}

static double now_ns(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (double)ts.tv_sec * 1e9 + (double)ts.tv_nsec;
}

int main(int argc, char **argv) {
    if (argc < 2) {
        fprintf(stderr, "usage: %s <n> [reps] [block|policy]\n", argv[0]);
        return 1;
    }
    size_t n = strtoull(argv[1], NULL, 0);
    int reps = argc > 2 ? atoi(argv[2]) : 15;
    int use_policy = argc > 3 && strcmp(argv[3], "policy") == 0;
    const char *label = use_policy ? "policy" : "fixed";

    size_t block;
    if (use_policy) {
        block = policy_block(n, sizeof(uint32pair));
    } else if (argc > 3) {
        block = strtoull(argv[3], NULL, 0);
    } else {
        block = 512;
    }
    permuted_set_block_size(block);

    uint32pair *data = aligned_alloc(128, (n * sizeof *data + 127) & ~(size_t)127);
    if (!data) { perror("aligned_alloc"); return 1; }

    double *times = malloc(sizeof(double) * reps);
    for (int r = 0; r < reps; r++) {
        fill_array(data, n);
        double t0 = now_ns();
        permuted_impl.full_sort(data, n);
        double t1 = now_ns();
        times[r] = t1 - t0;

        uint64_t sum = 0;
        for (size_t i = 0; i < n; i++) {
            if (i && data[i - 1][0] > data[i][0]) { fprintf(stderr, "not sorted\n"); return 1; }
            sum += (uint64_t)data[i][0] << 32 | data[i][1];
        }
        if (sum != cksum) { fprintf(stderr, "checksum mismatch\n"); return 1; }
    }

    qsort(times, reps, sizeof(double), cmp_u64);
    double med = times[reps / 2];
    printf("%s,%zu,%zu,%.1f,%.3f\n", label, n, block, med, med / (double)n);
    free(times);
    free(data);
    return 0;
}

int always_supported(void) { return 1; }
