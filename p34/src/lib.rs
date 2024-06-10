#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct BigUInt4096 {
    chunks: [u64; 64],
}

pub enum BigUIntError {
    Overflow,
    Underflow,
}

impl Default for BigUInt4096 {
    fn default() -> Self {
        BigUInt4096 { chunks: [0u64; 64] }
    }
}

impl BigUInt4096 {
    fn overflowing_add(&self, other: &Self) -> (Self, bool) {
        let mut sum = BigUInt4096::default();
        let mut carry: bool = false;
        let mut carry2: bool;
        // convention: least significant u64 is at index 0
        for i in 0..64 {
            (sum.chunks[i], carry2) = self.chunks[i].overflowing_add(u64::from(carry));
            (sum.chunks[i], carry) = sum.chunks[i].overflowing_add(other.chunks[i]);
            carry = carry || carry2;
        }
        (sum, carry)
    }

    fn overflowing_sub(&self, other: &Self) -> (Self, bool) {
        let mut diff = BigUInt4096::default();
        let mut borrow: bool = false;
        let mut borrow2: bool;
        // convention: least significant u64 is at index 0
        for i in 0..64 {
            (diff.chunks[i], borrow2) = self.chunks[i].overflowing_sub(u64::from(borrow));
            (diff.chunks[i], borrow) = diff.chunks[i].overflowing_sub(other.chunks[i]);
            borrow = borrow || borrow2;
        }
        (diff, borrow)
    }

    // fn checked_mul(&self, other: &Self) -> Result<Self, BigUIntError> {
    //     let mut i: usize = 63;
    //     let mut j: usize = 63;
    //     let mut prod = BigUInt4096::default();
    //     let mut rowprod = BigUInt4096::default();
    //     let mut carry: u64 = 0;
    //     while self.chunks[i] == 0 {
    //         i -= 1;
    //     }
    //     while other.chunks[j] == 0 {
    //         j -= 1;
    //     }
    //     if i + j > 63 {
    //         Err(BigUIntError::Overflow)
    //     } else if i + j == 63 && self.chunks[i].widening_mul(self.chunks[j]).0 > 0 {
    //         Err(BigUIntError::Overflow)
    //     } else {
    //         for k in 0..i {
    //             rowprod.reset();
    //             for l in 0..j {
    //                 (rowprod.chunks[k + l], carry) =
    //                     self.chunks[k].carrying_mul(other.chunks[l], carry);
    //             }
    //             prod.checked_add(&rowprod)?;
    //         }
    //         Ok(prod)
    //     }
    // }

    // fn reset(&mut self) {
    //     for i in 0..64 {
    //         self.chunks[i] = 0;
    //     }
    // }
}

impl std::ops::Add<BigUInt4096> for BigUInt4096 {
    type Output = Self;

    fn add(self, other: BigUInt4096) -> Self::Output {
        let (sum, carry) = self.overflowing_add(&other);
        debug_assert!(!carry, "Overflow while adding");
        sum
    }
}

impl std::ops::Add<&BigUInt4096> for BigUInt4096 {
    type Output = Self;

    fn add(self, other: &BigUInt4096) -> Self::Output {
        let (sum, carry) = self.overflowing_add(other);
        debug_assert!(!carry, "Overflow while adding");
        sum
    }
}

impl std::ops::Add<BigUInt4096> for &BigUInt4096 {
    type Output = BigUInt4096;

    fn add(self, other: BigUInt4096) -> Self::Output {
        let (sum, carry) = self.overflowing_add(&other);
        debug_assert!(!carry, "Overflow while adding");
        sum
    }
}

impl std::ops::Add<&BigUInt4096> for &BigUInt4096 {
    type Output = BigUInt4096;

    fn add(self, other: &BigUInt4096) -> Self::Output {
        let (sum, carry) = self.overflowing_add(other);
        debug_assert!(!carry, "Overflow while adding");
        sum
    }
}

impl std::ops::Sub<BigUInt4096> for BigUInt4096 {
    type Output = Self;

    fn sub(self, other: BigUInt4096) -> Self::Output {
        let (diff, borrow) = self.overflowing_sub(&other);
        debug_assert!(!borrow, "Overflow while subtracting");
        diff
    }
}

impl std::ops::Sub<&BigUInt4096> for BigUInt4096 {
    type Output = Self;

    fn sub(self, other: &BigUInt4096) -> Self::Output {
        let (diff, borrow) = self.overflowing_sub(other);
        debug_assert!(!borrow, "Overflow while subtracting");
        diff
    }
}

impl std::ops::Sub<BigUInt4096> for &BigUInt4096 {
    type Output = BigUInt4096;

    fn sub(self, other: BigUInt4096) -> Self::Output {
        let (diff, borrow) = self.overflowing_sub(&other);
        debug_assert!(!borrow, "Overflow while subtracting");
        diff
    }
}

impl std::ops::Sub<&BigUInt4096> for &BigUInt4096 {
    type Output = BigUInt4096;

    fn sub(self, other: &BigUInt4096) -> Self::Output {
        let (diff, borrow) = self.overflowing_sub(other);
        debug_assert!(!borrow, "Overflow while subtracting");
        diff
    }
}

// impl std::ops::Mul for BigUInt4096 {
//     type Output = Self;

//     fn mul(self, other: Self) -> Self::Output {
//         self.checked_mul(&other)?
//     }
// }
