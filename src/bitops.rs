/// returns the index of the first set bit from the least significant digit
pub fn bitscan(bb: u64) -> u64 {
    assert!(bb != 0);
    let mut i = 0;
    while bb & (1 << i) == 0 { i += 1 };
    i
}

/// returns the index of the first set bit from the most significant digit
pub fn bitscanr(bb: u64) -> u64 {
    assert!(bb != 0);
    let mut i = 63;
    while bb & (1 << i) == 0 { i -= 1 };
    i
}

/// clears the first set bit from the least significant digit returning its index
pub fn popbit(bb: &mut u64) -> u64 {
    let i = bitscan(*bb);
    *bb &= !(1 << i);
    i
}

/// clears the first set bit from the most significant digit returning its index
pub fn popbitr(bb: &mut u64) -> u64 {
    let i = bitscanr(*bb);
    *bb &= !(1 << i);
    i
}

