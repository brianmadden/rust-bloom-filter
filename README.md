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