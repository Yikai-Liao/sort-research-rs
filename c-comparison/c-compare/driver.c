// Standalone driver for the paper's reference implementation
// (radixsort_permuted.c, f69e816). Mirrors its own benchmark.c: xorshift fill of
// uint32pair, sort by first entry, timed over repetitions.
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "radixsort.h"

static uint64_t seed = 0xdeadbeefc0febabe;
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
        fprintf(stderr, "usage: %s <n> [reps]\n", argv[0]);
        return 1;
    }
    size_t n = strtoull(argv[1], NULL, 0);
    int reps = argc > 2 ? atoi(argv[2]) : 15;
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
    printf("%zu,%.1f,%.3f\n", n, med, med / (double)n);
    free(times);
    free(data);
    return 0;
}

int always_supported(void) { return 1; }
