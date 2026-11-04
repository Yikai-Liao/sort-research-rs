CXX ?= g++
CC ?= cc
CXXFLAGS ?= -O3 -DNDEBUG -std=c++20
CFLAGS ?= -O3 -DNDEBUG

all: driver_cxx driver_ref driver_policy

driver_cxx: cxx/driver.cpp cxx/prefix_radix.hpp
	$(CXX) $(CXXFLAGS) -o $@ cxx/driver.cpp

driver_ref: c-reference/driver.c c-reference/radixsort_permuted.c c-reference/radixsort.h
	$(CC) $(CFLAGS) -o $@ c-reference/driver.c c-reference/radixsort_permuted.c -lm

driver_policy: c-reference/driver_dyn.c c-reference/radixsort_permuted_dyn.c c-reference/radixsort.h
	$(CC) $(CFLAGS) -o $@ c-reference/driver_dyn.c c-reference/radixsort_permuted_dyn.c -lm

clang:
	$(MAKE) CXX=clang++ CC=clang all

clean:
	rm -f driver_cxx driver_ref driver_policy

.PHONY: all clang clean
