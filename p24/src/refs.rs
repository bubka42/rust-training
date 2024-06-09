/// accepts two mutable references and returns one of them
///
/// ```
/// use p24::refs::f1;
///
/// let mut first = 14;
/// let mut second = 10;
/// *f1(&mut first, &mut second, false) += 1;
/// assert_eq!(first, 15);
/// ```
pub fn f1<'a>(first: &'a mut u32, second: &'a mut u32, flag: bool) -> &'a mut u32 {
    if flag {
        second
    } else {
        first
    }
}

/// accepts mutable slice and returns mutable reference to n-th element
///
/// ```
/// use p24::refs::f2;
///
/// let mut slice = [5, 10, 12, 18, 32];
/// *f2(&mut slice, 3) += 1;
/// assert_eq!(slice[3], 19);
/// ```
pub fn f2(slice: &mut [u32], n: usize) -> &mut u32 {
    &mut slice[n]
}

/// accepts mutable slice and returns mutable reference to n-th element from the end
///
/// ```
/// use p24::refs::f3;
///
/// let mut slice = [5, 10, 12, 18, 32];
/// *f3(&mut slice, 3) += 1;
/// assert_eq!(slice[1], 11);
/// ```
pub fn f3(slice: &mut [u32], n: usize) -> &mut u32 {
    let l = slice.len();
    &mut slice[l - n - 1]
}

/// accepts mutable slice and partitions it into four mutable slices
/// convenion: remainder is distributed from end
/// e.g., 41 -> (10, 10, 10, 11), 43 -> (10, 11, 11, 11), etc.
///
/// ```
/// use p24::refs::f4;
///
/// let mut slice = [5, 10, 12, 18, 32];
/// assert_eq!(f4(&mut slice).3[0], 18);
/// ```
pub fn f4(slice: &[u32]) -> (&[u32], &[u32], &[u32], &[u32]) {
    let l = slice.len();
    let (a, b, c) = match l % 4 {
        0 => (l / 4, l / 2, 3 * l / 4),
        1 => ((l - 1) / 4, (l - 1) / 2, 3 * (l - 1) / 4),
        2 => ((l - 2) / 4, l / 2 - 1, 3 * (l - 2) / 4 + 1),
        3 => ((l - 3) / 4, (l - 3) / 2 + 1, 3 * (l - 3) / 4 + 2),
        _ => unreachable!(),
    };
    (&slice[..a], &slice[a..b], &slice[b..c], &slice[c..])
}

#[cfg(test)]
mod test {
    use super::f1;
    use super::f2;
    use super::f3;
    use super::f4;

    #[test]
    fn f1_test() {
        let mut first = 4;
        let mut second = 10;
        *f1(&mut first, &mut second, true) += 5;
        assert_eq!(second, 15);
        assert_eq!(first, 4);
    }

    #[test]
    fn f2_test() {
        let mut slice = [1, 3, 6, 4, 2, 9, 15, 7, 5, 10, 11, 20];
        assert_eq!(*f2(&mut slice, 8), 5);
        *f2(&mut slice, 6) -= 2;
        assert_eq!(slice[6], 13);
    }

    #[test]
    fn f3_test() {
        let mut slice = [1, 3, 6, 4, 2, 9, 15, 7, 5, 10, 11, 20];
        assert_eq!(*f3(&mut slice, 7), 2);
        *f3(&mut slice, 10) *= 12;
        assert_eq!(slice[1], 36);
    }

    #[test]
    fn f4_test1() {
        let mut vec = Vec::from_iter(0..43);
        let slices = f4(vec.as_mut_slice());
        assert_eq!(slices.0.len(), 10);
        assert_eq!(slices.1.len(), 11);
        assert_eq!(slices.2.len(), 11);
        assert_eq!(slices.3.len(), 11);
        assert_eq!(slices.1[2], 12);
        assert_eq!(slices.2[10], 31);
        assert_eq!(slices.3[0], 32);
    }

    #[test]
    fn f4_test2() {
        let mut vec = Vec::from_iter(0..8);
        let slices = f4(vec.as_mut_slice());
        assert_eq!(slices.0.len(), 2);
        assert_eq!(slices.1.len(), 2);
        assert_eq!(slices.2.len(), 2);
        assert_eq!(slices.3.len(), 2);
    }
}
