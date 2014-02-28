#[crate_id = "bloom#0.1"];
#[crate_type = "lib"];

//! A simple bloom filter implementation.
//! A bloom filter is a compact probabilistic data structure that
//! affords storage savings in favor of a chance of false positives
//! when querying the structure. More info can be found at
//! http://en.wikipedia.org/wiki/Bloom_filter.
//! By: Brian A. Madden - brian.a.madden@gmail.com

extern mod bit_vec;
extern mod murmur3;



use bit_vec::bit_vec::BitVec;
use murmur3::murmur3::murmur3_32_seeded;

use std::num::ln;
use std::num::powf;

use std::f64::ceil;

/// The BloomFilter object. Supports two methods, `insert` and
/// `maybe_present`.
struct BloomFilter {
    bits: BitVec,    // Bit vector
    num_hashes: uint,  // # of hashes needed
}

    
impl BloomFilter {
        
    /// Static constructor method. 
    /// Params: 
    ///   expected_inserts: uint - Expected number of items that
    ///                            will be inserted into the bloom filter
    ///   fpr: f64 - Desired false positive rate
    ///
    /// Both values will be used to calculate how large in bits
    /// the bloom filter should be, as well as how many hash
    /// functions are needed to have a bloom filter with the
    /// desired false positive rate.
    ///
    /// Returns: BloomFilter
    
    fn new(expected_inserts: uint, fpr: f64) -> BloomFilter {
        // Figure out necessary size of bit_vec (m bits)
        // m = -(n ln(p)) / ln(2)^2

        // Figure out necessary number of hash functions (k)
        // k = (m / n) ln(2)

        // n = expected_inserts
        // p = fpr

        // Verify that fpr != 0, this will cause errors
        if fpr <= 0.0 {
            fail!("False positive rate must be > 0.0!");
        }
        
        let m: uint = ceil((-1.0 * (expected_inserts as f64)
                            * ln(fpr)) / powf(ln(2.0), 2.0)) as uint;

        let k: uint = ceil(((m as f64) / 
                            (expected_inserts as f64)) * ln(2.0)) as uint;
        
        BloomFilter {bits: BitVec::new(m), num_hashes: k}
        
    }

    /// Insert a value into the bloom filter
    /// Params:
    ///   value: &str - Value to insert into the bloom filter
    /// Returns: ()

    fn insert(&mut self, value: &str) {
        // Generate a bit index for each of the hash functions needed
        for i in range(0, self.num_hashes) {
            let bit_index = ((murmur3_32_seeded(value, i as u32) 
                              % (self.bits.size as u32)) as uint);
            self.bits.set(bit_index);
        }  
    }
    
    /// Test to see if a value is maybe present. Maybe present
    /// because there is a chance of false positives when querying
    /// the structure.
    /// Params:
    ///   value: &str - The value to test for
    /// Returns: true if value maybe present, false otherwise
    fn maybe_present(&self, value: &str) -> bool {
        for i in range(0, self.num_hashes) {
            let bit_index = ((murmur3_32_seeded(value, i as u32) 
                              % (self.bits.size as u32)) as uint);

            if !self.bits.is_set(bit_index) {
                return false;
            }
        }
        return true;
    }
    
}


// Tests
#[test]
fn test_insert_and_check() {
    // Create new
    let mut bf = BloomFilter::new(2, 0.001);
    // Insert "test"
    bf.insert(&"test");
    // Assert its there
    assert!(bf.maybe_present(&"test"));
}

#[test]
fn test_check_only() {
    // Create new
    let mut bf = BloomFilter::new(2, 0.001);
    // BF is empty, all maybe_present should be false
    assert!(bf.maybe_present(&"not") == false);
    assert!(bf.maybe_present(&"foo") == false);
    assert!(bf.maybe_present(&"abcdefghijklmnop") == false);
    bf.insert("abc");
    assert!(bf.maybe_present(&"abc"));
}

#[test]
#[should_fail]

fn test_fpr_leq_0() {
    let bf = BloomFilter::new(2, 0.0);
}

