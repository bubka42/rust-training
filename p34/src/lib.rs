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
        Self {
            chunks: [0u64; 128],
        }
    }
}

impl BigUInt8192 {
    fn overflowing_add(&self, other: &Self) -> (Self, bool) {
        let mut sum = Self::default();
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
    pub fn new(init: &[u64]) -> Self {
        if init.len() > 64 {
            panic!("Invalid initialisation attempt");
        }
        let mut newint = Self::default();
        newint.chunks[..init.len()].clone_from_slice(init);
        newint
    }

    pub fn new_from_right(init: &[u64]) -> Self {
        if init.len() > 64 {
            panic!("Invalid initialisation attempt");
        }
        let mut newint = Self::default();
        newint.chunks[64 - init.len()..].clone_from_slice(init);
        newint
    }

    pub fn new_from_middle(init: &[u64], start: usize) -> Self {
        if start + init.len() > 64 {
            panic!("Invalid initialisation attempt");
        }
        let mut newint = Self::default();
        newint.chunks[start..start + init.len()].clone_from_slice(init);
        newint
    }

    fn overflowing_add(&self, other: &Self) -> (Self, bool) {
        let mut sum = Self::default();
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
        let mut diff = Self::default();
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

#[cfg(test)]
mod tests {
    use crate::BigUInt4096;

    // test from-left constructor
    #[test]
    fn new_test() {
        let num1 = BigUInt4096::new(&[2u64]);
        let num2 = BigUInt4096::new(&[2u64, 0u64]);
        assert_eq!(num1, num2);
    }

    // test from-right constructor
    #[test]
    fn new_from_right_test() {
        let num1 = BigUInt4096::new_from_right(&[2u64]);
        let num2 = BigUInt4096::new_from_right(&[0u64, 2u64]);
        assert_eq!(num1, num2);
    }

    // test from-middle constructor
    #[test]
    fn new_from_middle_test() {
        let num1 = BigUInt4096::new_from_middle(&[2u64], 14);
        let num2 = BigUInt4096::new_from_middle(&[0u64, 2u64], 13);
        assert_eq!(num1, num2);
    }

    // test widening-mul implementation
    #[test]
    fn widening_mul_test() {
        let one = 0xab00000000u64;
        let other = 0xbc00000000u64;
        assert_eq!(BigUInt4096::widening_mul(one, other), (0u64, 0x7d94u64));
    }

    // test overflowing-add for BigUInt4096 with no carry
    #[test]
    fn overflowing_add_4096_test1() {
        let num1 = BigUInt4096::new(&[2u64]);
        let num2 = BigUInt4096::new(&[2u64]);
        let num3 = BigUInt4096::new(&[4u64]);
        assert_eq!(num1.overflowing_add(&num2), (num3, false));
    }

    // test overflowing-add for BigUInt4096 with carry
    #[test]
    fn overflowing_add_4096_test2() {
        let num1 = BigUInt4096::new(&[2u64]);
        let num2 = BigUInt4096::new(&[u64::MAX]);
        let num3 = BigUInt4096::new(&[1u64, 1u64]);
        assert_eq!(num1.overflowing_add(&num2), (num3, false));
    }

    // test overflowing-add for BigUInt4096 with overflow
    #[test]
    fn overflowing_add_4096_test3() {
        let num1 = BigUInt4096::new_from_right(&[u64::MAX]);
        let num2 = BigUInt4096::new_from_right(&[1u64]);
        assert_eq!(num1.overflowing_add(&num2), (BigUInt4096::default(), true));
    }

    // test overflowing-sub for BigUInt4096 with borrow
    #[test]
    fn overflowing_sub_4096_test1() {
        let num1 = BigUInt4096::new(&[0u64, 1u64]);
        let num2 = BigUInt4096::new(&[1u64]);
        let num3 = BigUInt4096::new(&[u64::MAX]);
        assert_eq!(num1.overflowing_sub(&num2), (num3, false));
    }

    // test overflowing-sub for BigUInt4096 with borrow
    #[test]
    fn overflowing_sub_4096_test2() {
        let num1 = BigUInt4096::new(&[0u64, 4u64]);
        let num2 = BigUInt4096::new(&[u64::MAX]);
        let num3 = BigUInt4096::new(&[1u64, 3u64]);
        assert_eq!(num1.overflowing_sub(&num2), (num3, false));
    }

    // test overflowing-sub for BigUInt4096 with overflow
    #[test]
    fn overflowing_sub_4096_test3() {
        let num1 = BigUInt4096::new(&[1u64]);
        let num2 = BigUInt4096::new(&[2u64]);
        let num3 = BigUInt4096::new(&[u64::MAX; 64]);
        assert_eq!(num1.overflowing_sub(&num2), (num3, true));
    }

    // test overflowing-mul for BigUInt4096 with single non-zero chunks
    #[test]
    fn overflowing_mul_4096_test1() {
        let num1 = BigUInt4096::new_from_middle(&[0xd9u64], 26);
        let num2 = BigUInt4096::new_from_middle(&[0x45u64], 14);
        let num3 = BigUInt4096::new_from_middle(&[0x3a7du64], 40);
        assert_eq!(num1.overflowing_mul(&num2), (num3, BigUInt4096::default()));
    }

    // test overflowing-mul for BigUInt4096 with multiple non-zero chunks
    #[test]
    fn overflowing_mul_4096_test2() {
        let num1 = BigUInt4096::new_from_middle(&[0x1u64, 0x1u64], 20);
        let num2 = BigUInt4096::new_from_middle(&[0x1u64, 0x1u64], 32);
        let num3 = BigUInt4096::new_from_middle(&[0x1u64, 0x2u64, 0x1u64], 52);
        assert_eq!(num1.overflowing_mul(&num2), (num3, BigUInt4096::default()));
    }

    // test overflowing-mul for BigUInt4096 with single non-zero chunks and overflow
    #[test]
    fn overflowing_mul_4096_test3() {
        let num1 = BigUInt4096::new_from_middle(&[0x7fu64], 40);
        let num2 = BigUInt4096::new_from_middle(&[0xeeu64], 34);
        let num3 = BigUInt4096::new_from_middle(&[0x7612u64], 10);
        assert_eq!(num1.overflowing_mul(&num2), (BigUInt4096::default(), num3));
    }

    // the corresponding tests for the operator versions are below

    #[test]
    fn add_test1() {
        let num1 = BigUInt4096::new(&[2u64]);
        let num2 = BigUInt4096::new(&[2u64, 0u64]);
        let num3 = BigUInt4096::new(&[4u64]);
        assert_eq!(num1 + num2, num3);
    }

    #[test]
    fn add_test2() {
        let num1 = BigUInt4096::new(&[2u64, 0u64]);
        let num2 = BigUInt4096::new(&[u64::MAX]);
        let num3 = BigUInt4096::new(&[1u64, 1u64]);
        assert_eq!(num1 + num2, num3);
    }

    #[test]
    #[should_panic]
    fn add_test3() {
        let num1 = BigUInt4096::new_from_right(&[u64::MAX]);
        let num2 = BigUInt4096::new_from_right(&[1u64]);
        println!("{}", (num1 + num2).chunks[0]);
    }

    #[test]
    fn sub_test1() {
        let num1 = BigUInt4096::new(&[0u64, 1u64]);
        let num2 = BigUInt4096::new(&[1u64]);
        let num3 = BigUInt4096::new(&[u64::MAX]);
        assert_eq!(num1 - num2, num3);
    }

    #[test]
    fn sub_test2() {
        let num1 = BigUInt4096::new(&[0u64, 4u64]);
        let num2 = BigUInt4096::new(&[u64::MAX]);
        let num3 = BigUInt4096::new(&[1u64, 3u64]);
        assert_eq!(num1 - num2, num3);
    }

    #[test]
    #[should_panic]
    fn sub_test3() {
        let num1 = BigUInt4096::new(&[1u64]);
        let num2 = BigUInt4096::new(&[2u64]);
        println!("{}", (num1 - num2).chunks[0]);
    }

    #[test]
    fn mul_test1() {
        let num1 = BigUInt4096::new_from_middle(&[0xd9u64], 26);
        let num2 = BigUInt4096::new_from_middle(&[0x45u64], 14);
        let num3 = BigUInt4096::new_from_middle(&[0x3a7du64], 40);
        assert_eq!(num1 * num2, num3);
    }

    #[test]
    fn mul_test2() {
        let num1 = BigUInt4096::new_from_middle(&[0x1u64, 0x1u64], 20);
        let num2 = BigUInt4096::new_from_middle(&[0x1u64, 0x1u64], 32);
        let num3 = BigUInt4096::new_from_middle(&[0x1u64, 0x2u64, 0x1u64], 52);
        assert_eq!(num1 * num2, num3);
    }

    #[test]
    #[should_panic]
    fn mul_test3() {
        let num1 = BigUInt4096::new_from_middle(&[0x7fu64], 40);
        let num2 = BigUInt4096::new_from_middle(&[0xeeu64], 34);
        println!("{}", (num1 * num2).chunks[0]);
    }
}
