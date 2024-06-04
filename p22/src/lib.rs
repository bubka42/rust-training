pub mod calc;

#[cfg(test)]
mod tests {
    use crate::calc::celsius2farenheit;
    use crate::calc::farenheit2celsius;
    use crate::calc::fibonacci_loop;
    use crate::calc::fibonacci_rec;

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
