#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct BigUInt4096 {
    chunks: [u64; 64],
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct BigUInt8192 {
    chunks: [u64; 128],
}

impl Default for BigUInt4096 {
    fn default() -> Self {
        BigUInt4096 { chunks: [0u64; 64] }
    }
}

impl Default for BigUInt8192 {
    fn default() -> Self {
        BigUInt8192 {
            chunks: [0u64; 128],
        }
    }
}

impl BigUInt8192 {
    fn overflowing_add(&self, other: &Self) -> (Self, bool) {
        let mut sum = BigUInt8192::default();
        let mut carry: bool = false;
        let mut carry2: bool;
        // convention: least significant u64 is at index 0
        for i in 0..128 {
            (sum.chunks[i], carry2) = self.chunks[i].overflowing_add(u64::from(carry));
            (sum.chunks[i], carry) = sum.chunks[i].overflowing_add(other.chunks[i]);
            carry = carry || carry2;
        }
        (sum, carry)
    }

    fn reset(&mut self) {
        for i in 0..128 {
            self.chunks[i] = 0;
        }
    }

    fn split(&self) -> (BigUInt4096, BigUInt4096) {
        let mut left = [0u64; 64];
        let mut right = [0u64; 64];
        left.clone_from_slice(&self.chunks[..64]);
        right.clone_from_slice(&self.chunks[64..]);
        (BigUInt4096 { chunks: left }, BigUInt4096 { chunks: right })
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

    fn widening_mul(one: u64, other: u64) -> (u64, u64) {
        let prod = (one as u128) * (other as u128);
        (prod as u64, (prod >> 64) as u64)
    }

    fn overflowing_mul(&self, other: &Self) -> (Self, Self) {
        let mut prod = BigUInt8192::default();
        let mut rowprod = BigUInt8192::default();
        let mut chunkprod: u64;
        let mut carry: u64;
        for i in 0..64 {
            rowprod.reset();
            for j in 0..64 {
                (chunkprod, carry) = Self::widening_mul(self.chunks[i], other.chunks[j]);
                (rowprod.chunks[i + j], rowprod.chunks[i + j + 1]) =
                    match rowprod.chunks[i + j].overflowing_add(chunkprod) {
                        (sum, true) => (sum, carry + 1),
                        (sum, _) => (sum, carry),
                    };
            }
            (prod, _) = prod.overflowing_add(&rowprod);
        }
        prod.split()
    }
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

impl std::ops::Mul<BigUInt4096> for BigUInt4096 {
    type Output = Self;

    fn mul(self, other: BigUInt4096) -> Self::Output {
        let (prod, carry) = self.overflowing_mul(&other);
        debug_assert_eq!(carry, BigUInt4096::default(), "Overflow while multiplying");
        prod
    }
}

impl std::ops::Mul<&BigUInt4096> for BigUInt4096 {
    type Output = Self;

    fn mul(self, other: &BigUInt4096) -> Self::Output {
        let (prod, carry) = self.overflowing_mul(other);
        debug_assert_eq!(carry, BigUInt4096::default(), "Overflow while multiplying");
        prod
    }
}

impl std::ops::Mul<BigUInt4096> for &BigUInt4096 {
    type Output = BigUInt4096;

    fn mul(self, other: BigUInt4096) -> Self::Output {
        let (prod, carry) = self.overflowing_mul(&other);
        debug_assert_eq!(carry, BigUInt4096::default(), "Overflow while multiplying");
        prod
    }
}

impl std::ops::Mul<&BigUInt4096> for &BigUInt4096 {
    type Output = BigUInt4096;

    fn mul(self, other: &BigUInt4096) -> Self::Output {
        let (prod, carry) = self.overflowing_mul(other);
        debug_assert_eq!(carry, BigUInt4096::default(), "Overflow while multiplying");
        prod
    }
}
