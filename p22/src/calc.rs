/// Convert celsius to farenheit
///
/// ```
/// use p22::calc::celsius2farenheit;
///
/// assert_eq!(celsius2farenheit(0), 32);
/// assert_eq!(celsius2farenheit(100), 212);
/// ```
pub fn celsius2farenheit(celsius: i32) -> i32 {
    (9 * celsius) / 5 + 32
}

/// Convert farenheit to celsius
///
/// ```
/// use p22::calc::farenheit2celsius;
///
/// assert_eq!(farenheit2celsius(-40), -40);
/// assert_eq!(farenheit2celsius(50), 10);
/// ```
pub fn farenheit2celsius(farenheit: i32) -> i32 {
    (farenheit - 32) * 5 / 9
}

/// Print nth fibonacci number using a loop
///
/// ```
/// use p22::calc::fibonacci_loop;
///
/// assert_eq!(fibonacci_loop(5), 5);
/// assert_eq!(fibonacci_loop(10), 55);
/// ```
pub fn fibonacci_loop(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut first: u64 = 0;
    let mut second: u64 = 1;
    for _ in 1..n {
        second += first;
        first = second - first;
    }
    second
}

/// Print nth fibonacci number using a loop
///
/// ```
/// use p22::calc::fibonacci_rec;
///
/// assert_eq!(fibonacci_rec(8), 21);
/// assert_eq!(fibonacci_rec(11), 89);
/// ```
pub fn fibonacci_rec(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        m => fibonacci_rec(m - 1) + fibonacci_rec(m - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::celsius2farenheit;
    use super::farenheit2celsius;
    use super::fibonacci_loop;
    use super::fibonacci_rec;

    #[test]
    fn test_celsius2farenheit() {
        assert_eq!(celsius2farenheit(5), 41);
        assert_eq!(celsius2farenheit(-10), 14);
        assert_eq!(celsius2farenheit(-100), -148);
    }

    #[test]
    fn test_farneheit2celsius() {
        assert_eq!(farenheit2celsius(23), -5);
        assert_eq!(farenheit2celsius(-13), -25);
        assert_eq!(farenheit2celsius(392), 200);
    }

    #[test]
    fn test_fibonacci_loop() {
        assert_eq!(fibonacci_loop(4), 3);
        assert_eq!(fibonacci_loop(9), 34);
        assert_eq!(fibonacci_loop(13), 233);
    }

    #[test]
    fn test_fibonacci_rec() {
        assert_eq!(fibonacci_rec(6), 8);
        assert_eq!(fibonacci_rec(7), 13);
        assert_eq!(fibonacci_rec(12), 144);
    }
}
