#[crate_id = "bit_vec#0.1"];
#[crate_type = "lib"];

//! A simple bit vector library
//! By: Brian A. Madden - brian.a.madden@gmail.com

pub mod bit_vec {

    //! Bit vector object. Allows for setting, unsetting, flipping of
    //! bits.  Indexing beyond the bounds of the vector will raise an
    //! error.
    
    use std::vec::from_elem;
    
    pub struct Bit_vec {        
        // Just the bits, nothing but the bits...
        bits: ~[u8],
        // Size of the bit array for bounds checking
        size: uint
    }

    impl Bit_vec {
        
        /// Create a new `bitvec` with `size` bits
        pub fn new(size: uint) -> Bit_vec { 
            Bit_vec { bits: from_elem(size, (0 as u8)), size: size}
        }

        pub fn is_set(&self, pos: uint) -> bool { 
            if pos > self.size { fail!("Attempted to index beyond bounds of bit vector."); }
            if (1 << (pos % 8)) & self.bits[pos / 8] > 0 { true } else { false }
        }
        
        /// Set the bit at `pos` to 1
        pub fn set(&mut self, pos: uint) {
            if pos > self.size { fail!("Attempted to index beyond bounds of bit vector."); }
            self.bits[pos / 8] |= (1 << (pos % 8));
        }

        /// Set the bit at `pos` to 0
        pub fn unset(&mut self, pos: uint) {
            if pos > self.size { fail!("Attempted to index beyond bounds of bit vector."); }
            self.bits[pos / 8] &= { 
                0xFF ^ (1 << (pos % 8))
            }
        }
        
        /// Flip the bit at `pos`. If the bit is 0 it becomes 1; if
        /// the bit is 1 it becomes 0.
        pub fn flip(&mut self, pos: uint) {
            if pos > self.size { fail!("Attempted to index beyond bounds of bit vector."); }
            self.bits[pos / 8] ^= (1 << (pos % 8));
        }

        /// Return the raw bytes of the bit vector
        pub fn get_bytes(~self) -> ~[u8] {
            self.bits
        }
                    
    }

    #[test]
    fn bit_vec_create_test() {
        let tester: Bit_vec = Bit_vec::new(8);
        assert!(tester.bits[0] == 0);
    }

    #[test]
    fn bit_vec_set_test() {
        let mut tester: Bit_vec = Bit_vec::new(8);
        tester.set(5);
        assert!(tester.bits[0] == 32);
        let res = tester.is_set(5);
        assert!(res == true);
    }

    #[test]
    fn bit_vec_is_set_test() {
        let mut tester: Bit_vec = Bit_vec::new(8);
        tester.set(5);
        assert!(tester.is_set(5) == true);
        assert!(tester.is_set(6) == false);
    }
    
    #[test]
    fn bit_vec_unset_test() {
        let mut tester: Bit_vec = Bit_vec::new(8);
        tester.set(5);
        assert!(tester.is_set(5) == true);
        tester.unset(5);
        assert!(tester.is_set(5) == false);
        assert!(tester.bits[0] == 0);
    }

    #[test]
    fn bit_vec_flip_test() {
        let mut tester: Bit_vec = Bit_vec::new(8);
        tester.flip(5);
        assert!(tester.is_set(5) == true);
        tester.flip(5);
        assert!(tester.is_set(5) == false);
    }

    #[test]
    #[should_fail]
    fn bit_vec_out_of_bounds_test() {
        let mut tester: Bit_vec = Bit_vec::new(8);
        tester.set(15);
    }
    
}
