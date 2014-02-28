
## Makefile for rust-bloom-filter, bit-vec library, and murmurhash3 library

CC=rustc
CFLAGS=-L .
TESTFLAG = --test

all: bloom-filter

bloom-filter: murmur bit-vec
	$(CC) $(CFLAGS) bloom.rs

murmur:
	$(CC) murmur.rs

bit-vec:
	$(CC) bit_vec.rs

bloom-test: murmur3 bit-vec
	$(CC) $(CFLAGS) $(TESTFLAG) bloom.rs

murmur3-test:
	$(CC) $(TESTFLAG) murmur.rs

bit-vec-test:
	$(CC) $(TESTFLAG) bit_vec.rs


clean:
	rm -rf *rlib
