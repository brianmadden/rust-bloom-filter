rust-bloom-filter
=================

Bloom filter library implemented in Rust -- includes MurmurHash3
implementation, and simple bit vector library

Currently **does not** actually contain a working bloom filter
implementation. When I set out to write the library I realized that
there was a bit of groundwork that needed to be done first.  This
repository contains the start of that groundwork, and as it evolves
will eventually contain everything needed for a full-featured bloom
filter library.

Still very early implementation. Currently includes working 32-bit
murmurhash3 implementation, as well as a working bit vector library
that can `set`, `unset`, `flip` bits.


Building
========

Libraries were built and tested using the master branch of Rust on Feb
18, 2014. Libraries can be compilied by calling `rustc murmur.rs ;
rustc bit_vec.rs`. Tests can be compiled using the `--test` flag
during compliation.

Use
===

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
`Hooray!
 Boo!`
