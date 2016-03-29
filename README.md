## DEPRECATED

### Please note, this project is deprecated and no longer being maintained, please use an up-to-date version at [jonalmeida/bloom-filter](https://github.com/jonalmeida/bloom-filter).

---

rust-bloom-filter
=================

Bloom filter library implemented in Rust -- includes MurmurHash3
implementation, and simple bit vector library

Still early implementation. Currently includes: working bloom filter
library, working 32-bit murmurhash3 implementation, as well as a
working bit vector library that can `set`, `unset`, `flip` bits.

Building
========

Libraries were built and tested using the master branch of Rust pulled
on Feb 18, 2014. Libraries can be built using the included Makefile.

Use
===
bloom
-----
The bloom crate exports one module, bloom_filter. The bloom_filter
module exports a single object, `BloomFilter` with three public methods,
`new`, `insert`, and `maybe_present`. The `new` method is static and
can be used to create a new `BloomFilter` with specified expected number
of insertions, and desired false positive rate. Example:

	// Create new BloomFilter with 1000 expected elements, and false positive rate of 3%
	let mut bf = BloomFilter::new(1000, 0.03);

`insert` will insert a string value into the bloom filter, and
`maybe_present` will return true if queried value might be present, or
false if it is *definitely not* present.


murmur3
-------

The murmur library exports two public functions, `murmur3_32_seeded`
and `murmur3_32`. The `murmur3_32` function is simply a convenience
function that calls `murmur3_32_seeded` with a seed value of 0. Both
functions take a `&str` to be hashed and return a `u32` value.


bit_vec
-------

The bit_vec library contains a struct, `Bit_vec` that stores an vector
of `u8` bytes, and the desired size of the bit vector as an
`uint`. Bounds checking is provided by checking against the size
value.

A `Bit_vec` can be created with the constructor function `new` with
a single `uint` size parameter.
`let my_bv: Bit_vec = Bit_vec::new(24);`

The struct has four public methods available, `set`, `unset`, `flip`,
and `is_set`. `set` will set a bit at the specified index position to
1, `unset` will set a bit at the specified index position to 0. `flip`
will invert a bit at the specified index position, e.g. a 0 becomes a
1 and a 1 becomes a 0. `is_set` will return true if the bit at the
specified index position is a 1, and false otherwise.

### Examples

	extern mod bit_vec;
	use bit_vec::bit_vec::Bit_vec;

	fn main() {

       let mut bv = Bit_vec::new(8);
       bv.set(5);
       test_for_5(&bv);
       bv.unset(5);
       test_for_5(&bv);

	}

	fn test_for_5(bv: &Bit_vec) {
       if bv.is_set(5) == true { println!("Hooray!"); } else { println!("Boo!"); }
	}


Should result in the following output:

	   Hooray!
	   Boo!
