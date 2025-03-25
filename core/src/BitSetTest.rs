use bitvec::prelude::*;

///
/// BitSet is a bitset implemented as a bit vector.
/// It need dependancy on bit-vec crate.
/// 
pub fn test() {
    /* XXX: Bitset */
    let mut bitset = bitvec![0; 1024];
    bitset.set(100, true);
    bitset.set(500, true);
    if bitset[100] {
        println!("Bit 100 is set");
    }
    bitset.set(500, false);
    for (i, bit) in bitset.iter().enumerate() {
        if *bit {
            println!("Bit {} is set", i);
        }
    }
    let val = bitset.count_ones();
    //let val = bitset.get(100).unwrap_or_else(|| &false);
    println!("Value: {}", val);
}
